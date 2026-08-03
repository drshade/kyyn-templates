.PHONY: check

check:
	./scripts/check-templates.sh
	./ops/self-hosted-runner/check.sh
