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
                validate: func(tree: snapshot) -> result<list<violation>, diagnostic>;
            }

            world validator {
                export api;
            }
        "#,
        world: "validator",
    });
}

use bindings::exports::kyyn::validator::api::{Diagnostic, Guest, Severity, Snapshot, Violation};

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

    fn validate(tree: Snapshot) -> Result<Vec<Violation>, Diagnostic> {
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

        let entries = tree
            .entries
            .into_iter()
            .map(|entry| {
                String::from_utf8(entry.contents)
                    .map(|contents| (entry.path, contents))
                    .map_err(|error| Diagnostic {
                        code: "non-utf8-entry".into(),
                        message: error.to_string(),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let systems = tree.systems.iter().map(String::as_str).collect::<Vec<_>>();
        let mut violations = crate::registry::validate_entries_with(&entries, &systems)
            .into_iter()
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

fn severity_rank(severity: &Severity) -> u8 {
    match severity {
        Severity::Error => 0,
        Severity::Warning => 1,
    }
}

bindings::export!(TemplateValidator with_types_in bindings);
