# Awesome Skills CLI

> Skill library and Rust CLI for AI coding assistants.

## What's Here

| Path | Purpose |
|---|---|
| `skills/` | 1,329+ skill directories, each containing a `SKILL.md` playbook |
| `skills_index.json` | Flat index of all skills with metadata |
| `data/bundles.json` | Curated skill groups by role |
| `data/editorial-bundles.json` | Editorial bundle definitions |
| `data/aliases.json` | Skill name aliases for discoverability |
| `data/catalog.json` | Catalog data for search |
| `.github/workflows/release.yml` | Release pipeline for Rust binary artifacts |

## How to Use

### CLI

**Quick install (Linux / macOS):**

```bash
curl -fsSL https://raw.githubusercontent.com/codeignus/awesome-skills-cli/main/scripts/install.sh | bash
```

Or install manually:

```bash
# Download the latest binary for your platform from the
# [latest GitHub release](https://github.com/codeignus/awesome-skills-cli/releases/latest),
# make it executable, and place it somewhere on your PATH (e.g. ~/.local/bin).
```

**Usage:**

```bash
awesome-skills-cli add brainstorming --path ~/.claude/skills
awesome-skills-cli setup --path .agents/skills
awesome-skills-cli setup recommend-awesome-skills --path .agents/skills
awesome-skills-cli search "testing"
awesome-skills-cli info brainstorming
awesome-skills-cli update
awesome-skills-cli version
awesome-skills-cli catalog-for-agent --limit 250 --offset 0
awesome-skills-cli catalog-for-agent --limit 250 --offset 250
```

**User-facing commands:**

| Command | Description |
|---|---|
| `list [--category X]` | List all skills (full detail, meant for human browsing) |
| `search <query>` | Fuzzy search skills by name or keyword |
| `info <skill-id>` | Show detailed info for one skill |
| `add <skill-id...> --path <dir>` | Copy one or more skills to a directory |
| `setup [skill-id...] --path <dir>` | Copy all built-in meta skills, or only the named meta skills |
| `update` | Self-update to the latest release |
| `version` | Print version info |

**Agent-facing command:**

| Command | Description |
|---|---|
| `catalog-for-agent --limit N --offset M` | Output a condensed JSON array slice on stdout and pagination metadata on stderr for agent batching |

Use `list` when you (the user) want to browse skills. Use `catalog-for-agent` in your agent's configuration or MCP setup so the LLM can discover available skills efficiently.

Use `setup` when you want the CLI to install its own embedded meta skills into a skills directory. These meta skills live under `src/skills/` in the repo, are bundled into the binary separately, and are intentionally excluded from the main indexed catalog used by `list`, `search`, `catalog-for-agent`, and `info`.

## Development

```bash
cargo test
cargo build --release
```

Lint workflows with [actionlint](https://github.com/rhysd/actionlint):

```bash
actionlint
```

## Skill Categories

| Category | Focus |
|---|---|
| Architecture | System design, ADRs, C4, scalable patterns |
| Business | Growth, pricing, CRO, SEO, go-to-market |
| Data & AI | LLM apps, RAG, agents, observability |
| Development | Language mastery, framework patterns, code quality |
| Infrastructure | DevOps, cloud, serverless, deployment, CI/CD |
| Security | AppSec, pentesting, vuln analysis, compliance |
| Testing | TDD, test design, fixes, QA workflows |
| Workflow | Automation, orchestration, jobs, agents |

## Releases

Tagged releases build `awesome-skills-cli` with Cargo for Linux and macOS targets and publish the binaries plus SHA256 checksums as GitHub Release assets.
