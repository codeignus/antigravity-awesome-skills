# Antigravity Awesome Skills

> Installable skill library of 1,329+ agentic skills for AI coding assistants.

This is a fork of [sickn33/antigravity-awesome-skills](https://github.com/sickn33/antigravity-awesome-skills) stripped down to the essentials: skills and index data. All CI, tooling, documentation, and release infrastructure lives in the upstream repo.

## What's Here

| Path | Purpose |
|---|---|
| `skills/` | 1,329+ skill directories, each containing a `SKILL.md` playbook |
| `skills_index.json` | Flat index of all skills with metadata (id, category, description, tags) |
| `data/bundles.json` | Curated skill groups by role (Web Wizard, Security Engineer, etc.) |
| `data/editorial-bundles.json` | Editorial bundle definitions |
| `data/aliases.json` | Skill name aliases for discoverability |
| `data/catalog.json` | Catalog data for search |

## How to Use

Copy any skill's `SKILL.md` into your AI tool's skills directory:

- **Claude Code**: `.claude/skills/`
- **Gemini CLI**: `.gemini/skills/`
- **Codex CLI**: `.codex/skills/`
- **Cursor**: `.cursor/skills/`
- **OpenCode**: `.agents/skills/`
- **Kiro**: `.kiro/skills/`

Example:

```bash
cp -r skills/brainstorming ~/.claude/skills/
```

Then reference it in your prompts:

> Use @brainstorming to plan a SaaS MVP.

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

## Upstream

For contributing, CI, releases, plugins, and full documentation, see the upstream repo:

**[github.com/sickn33/antigravity-awesome-skills](https://github.com/sickn33/antigravity-awesome-skills)**

## License

MIT
