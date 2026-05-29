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

1. Collect deterministic probe facts:
   - binary existence and `--version`
   - PATH/default binary and duplicate install candidates
   - config existence, parse status, and runtime-specific schema warnings
   - environment variable conflicts from process and common shell files
   - gateway/base_url TCP reachability
   - obvious MCP/Skills path references and broken links
   - Hermes provider/API key env requirements and `.env` permissions
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

`agent-doctor repair <runtime>` runs read-only probes and prints the safe repair preview.

`agent-doctor repair <runtime> --apply` runs the execute skeleton:

1. `probe_runtime` (before)
2. build repair plan from redacted facts
3. backup runtime config files under `~/.config/agent-doctor/backups/`
4. apply typed actions (rule playbooks still expanding; unimplemented actions are skipped)
5. `probe_runtime` (after)
6. write `AuditReport` with verification summary and rollback hint

Runtime-specific rule playbooks extend step 4. **Hermes v1 (shipped):**

- tighten `~/.hermes/.env` permissions to `600` when too open
- deduplicate repeated API key env entries (keep last non-empty value)
- fill missing/empty `model.*` fields from the active Agent Doctor profile preset
- when the provider API key is missing: append a `VAR=` placeholder to `~/.hermes/.env` (or create the file) and write a local guide under `~/.config/agent-doctor/guides/hermes-api-key-<VAR>.md`
- when `hermes` is not on PATH: run the official Hermes installer script (same approach as [CC Switch](https://github.com/farion1231/cc-switch); requires network and user confirmation via `--apply`)

**Never shipped / not planned for v1:** auto-filling or uploading API keys; AI choosing arbitrary shell commands.

**Rollback:** backups live under `~/.config/agent-doctor/backups/<runtime>-<timestamp>/`. Restore with:

```bash
agent-doctor repair hermes --rollback
agent-doctor repair hermes --rollback --backup hermes-2026-05-28T12-00-00
```

The desktop app shows suggested fixes after diagnosis, runs apply (backup → playbook → re-probe → audit), can open the API key guide, and can roll back from the latest backup.
