//! `kyyn:validator@1` guest adapter for this template's schema.

mod bindings {
    wit_bindgen::generate!({
        inline: r#"
            package kyyn:validator@1.0.0;

            interface api {
                record entry {
                    path: string,
                    contents: list<u8>,
                }

                record snapshot {
                    format-version: u32,
                    entries: list<entry>,
                    systems: list<string>,
                }

                enum authority {
                    direct-human,
                    recipe-evaluation,
                    agent-transcribed-human,
                    existing-knowledge,
                    mechanical-derivation,
                    engine-generated,
                    observed-host,
                    unattributed,
                }

                record subject {
                    kind: string,
                    id: string,
                }

                record transition {
                    path: string,
                    subject: option<subject>,
                    field: option<string>,
                    before-sha256: option<string>,
                    after-sha256: option<string>,
                    authority: authority,
                    causal-reference: string,
                }

                record validation-context {
                    format-version: u32,
                    previous: option<snapshot>,
                    candidate: snapshot,
                    transitions: list<transition>,
                }

                enum severity {
                    error,
                    warning,
                }

                record violation {
                    path: string,
                    severity: severity,
                    message: string,
                }

                record diagnostic {
                    code: string,
                    message: string,
                }

                registry: func() -> result<list<u8>, diagnostic>;
                validate: func(context: validation-context) -> result<list<violation>, diagnostic>;
            }

            world validator {
                export api;
            }
        "#,
        world: "validator",
    });
}

use bindings::exports::kyyn::validator::api::{
    Authority, Diagnostic, Guest, Severity, Snapshot, Transition, ValidationContext, Violation,
};

struct TemplateValidator;

impl Guest for TemplateValidator {
    fn registry() -> Result<Vec<u8>, Diagnostic> {
        kyyn_core::ronfmt::to_ron(&crate::registry::registry())
            .map(String::into_bytes)
            .map_err(|error| Diagnostic {
                code: "registry-serialization".into(),
                message: error.to_string(),
            })
    }

    fn validate(context: ValidationContext) -> Result<Vec<Violation>, Diagnostic> {
        if context.format_version != 1 {
            return Err(Diagnostic {
                code: "unsupported-validation-context".into(),
                message: format!(
                    "validation context format {} is unsupported; expected 1",
                    context.format_version
                ),
            });
        }
        let tree = &context.candidate;
        if tree.format_version != 1 {
            return Err(Diagnostic {
                code: "unsupported-snapshot".into(),
                message: format!(
                    "snapshot format {} is unsupported; expected 1",
                    tree.format_version
                ),
            });
        }
        if !tree
            .entries
            .windows(2)
            .all(|pair| pair[0].path < pair[1].path)
            || !tree
                .systems
                .windows(2)
                .all(|pair| pair[0].as_str() < pair[1].as_str())
        {
            return Err(Diagnostic {
                code: "noncanonical-snapshot".into(),
                message: "snapshot entries and systems must be sorted and unique".into(),
            });
        }

        let entries = snapshot(tree.clone())?;
        let previous = context.previous.map(core_snapshot).transpose()?;
        let transitions = context
            .transitions
            .into_iter()
            .map(core_transition)
            .collect();
        let validation = kyyn_core::validator::ValidationContext {
            previous,
            candidate: kyyn_core::validator::Snapshot {
                entries: entries
                    .iter()
                    .map(|(path, text)| (path.clone(), text.as_bytes().to_vec()))
                    .collect(),
                systems: tree.systems.clone(),
            },
            transitions,
        };
        let systems = tree.systems.iter().map(String::as_str).collect::<Vec<_>>();
        let mut violations = crate::registry::validate_entries_with(&entries, &systems)
            .into_iter()
            .chain(crate::validate::validate_transition(&validation))
            .map(|violation| Violation {
                path: violation.path,
                severity: match violation.severity {
                    kyyn_core::violation::Severity::Error => Severity::Error,
                    kyyn_core::violation::Severity::Warning => Severity::Warning,
                },
                message: violation.message,
            })
            .collect::<Vec<_>>();
        violations.sort_by(|left, right| {
            (&left.path, severity_rank(&left.severity), &left.message).cmp(&(
                &right.path,
                severity_rank(&right.severity),
                &right.message,
            ))
        });
        Ok(violations)
    }
}

fn snapshot(tree: Snapshot) -> Result<Vec<(String, String)>, Diagnostic> {
    if tree.format_version != 1 {
        return Err(Diagnostic {
            code: "unsupported-snapshot".into(),
            message: format!(
                "snapshot format {} is unsupported; expected 1",
                tree.format_version
            ),
        });
    }
    if !tree
        .entries
        .windows(2)
        .all(|pair| pair[0].path < pair[1].path)
        || !tree
            .systems
            .windows(2)
            .all(|pair| pair[0].as_str() < pair[1].as_str())
    {
        return Err(Diagnostic {
            code: "noncanonical-snapshot".into(),
            message: "snapshot entries and systems must be sorted and unique".into(),
        });
    }
    tree.entries
        .into_iter()
        .map(|entry| {
            String::from_utf8(entry.contents)
                .map(|contents| (entry.path, contents))
                .map_err(|error| Diagnostic {
                    code: "non-utf8-entry".into(),
                    message: error.to_string(),
                })
        })
        .collect()
}

fn core_snapshot(tree: Snapshot) -> Result<kyyn_core::validator::Snapshot, Diagnostic> {
    let systems = tree.systems.clone();
    let entries = snapshot(tree)?
        .into_iter()
        .map(|(path, text)| (path, text.into_bytes()))
        .collect();
    Ok(kyyn_core::validator::Snapshot { entries, systems })
}

fn core_transition(transition: Transition) -> kyyn_core::validator::Transition {
    kyyn_core::validator::Transition {
        path: transition.path,
        subject: transition
            .subject
            .map(|subject| kyyn_core::validator::Subject {
                kind: subject.kind,
                id: subject.id,
            }),
        field: transition.field,
        before_sha256: transition.before_sha256,
        after_sha256: transition.after_sha256,
        authority: match transition.authority {
            Authority::DirectHuman => kyyn_core::validator::Authority::DirectHuman,
            Authority::RecipeEvaluation => kyyn_core::validator::Authority::RecipeEvaluation,
            Authority::AgentTranscribedHuman => {
                kyyn_core::validator::Authority::AgentTranscribedHuman
            }
            Authority::ExistingKnowledge => kyyn_core::validator::Authority::ExistingKnowledge,
            Authority::MechanicalDerivation => {
                kyyn_core::validator::Authority::MechanicalDerivation
            }
            Authority::EngineGenerated => kyyn_core::validator::Authority::EngineGenerated,
            Authority::ObservedHost => kyyn_core::validator::Authority::ObservedHost,
            Authority::Unattributed => kyyn_core::validator::Authority::Unattributed,
        },
        causal_reference: transition.causal_reference,
    }
}

fn severity_rank(severity: &Severity) -> u8 {
    match severity {
        Severity::Error => 0,
        Severity::Warning => 1,
    }
}

bindings::export!(TemplateValidator with_types_in bindings);
