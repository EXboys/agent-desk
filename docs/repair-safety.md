# Repair Safety and Compliance Model

Agent Doctor treats repair as a controlled enterprise workflow, not as a free-form agent with shell access.

## Principles

- **Local first**: probes, backups, repair plans, and audit reports are local by default.
- **Redacted by default**: secrets and sensitive logs must be redacted before an AI model sees diagnostic data.
- **Typed actions only**: repair execution is limited to known action kinds such as backup, config patch, install/update, symlink cleanup, verify, and manual review.
- **Confirmation for writes**: file changes, reinstalls, and destructive operations require explicit confirmation.
- **Rollback metadata**: every real repair run must produce a backup snapshot and audit report.
- **Policy gates**: enterprise policy can block raw log upload, raw config upload, destructive actions, or non-approved gateways.

## Diagnostic Data Classes

| Class | Examples | Default AI visibility |
|-------|----------|-----------------------|
| `public` | runtime id, installed flag, version | visible |
| `local_path` | config path, binary path, backup root | redacted to `$HOME` form |
| `config_shape` | field names, schema errors, parse errors | visible |
| `secret` | API keys, bearer tokens, OAuth tokens | hidden |
| `sensitive_log` | logs that may include prompts, tokens, file names | inline secrets redacted |

## Repair Flow

1. Collect deterministic probe facts.
2. Create a backup snapshot before modifying files.
3. Redact the diagnostic bundle.
4. Let AI summarize and rank likely causes from redacted facts.
5. Convert the plan into typed repair actions.
6. Ask for confirmation before writes.
7. Execute the approved actions.
8. Verify runtime health and write an audit report.

## Enterprise Controls

Teams can require:

- Company gateway only for AI diagnosis.
- No raw secrets, logs, or config values outside the local machine.
- Mandatory backup snapshot before repair.
- Mandatory diff preview before config writes.
- High-risk actions disabled unless policy explicitly allows them.
- Local audit reports for IT support or control-plane ingestion.

The current `agent-doctor repair <runtime>` command prints the safe repair preview. Runtime-specific repair playbooks will build on this foundation.
