---
name: awesome-skills-cli
description: Offline command reference for the awesome-skills-cli binary. Use when you need exact command syntax, setup guidance, install locations, or PATH troubleshooting for this CLI itself.
---

# awesome-skills-cli

Use this skill when the user needs help operating `awesome-skills-cli` itself.

What this skill covers:
- `list`, `search`, `catalog-for-agent`, `info`, `add`, `setup`, `update`, and `version`
- suggested install targets such as `.agents/skills`, `.claude/skills`, `.opencode/skills`, `.gemini/skills`, `.codex/skills`, `.cursor/skills`, and `.kiro/skills`
- checking whether `awesome-skills-cli` is available in `PATH`
- explaining the difference between indexed skills and the meta skills installed by `setup`

Command reference:

```bash
awesome-skills-cli list
awesome-skills-cli list --category testing
awesome-skills-cli search "rust testing"
awesome-skills-cli catalog-for-agent
awesome-skills-cli info brainstorming
awesome-skills-cli add brainstorming test-driven-development --path .agents/skills
awesome-skills-cli setup --path .agents/skills
awesome-skills-cli update
awesome-skills-cli version
```

Command behavior:
- `list` is for human browsing with full detail.
- `search` is fuzzy and good for narrowing candidates.
- `catalog-for-agent` emits a condensed JSON catalog for LLM consumption.
- `info` shows one indexed skill in more detail.
- `add` copies indexed skills from the embedded catalog into the target directory.
- `setup` copies the CLI's embedded meta skills into the target directory.
- `update` self-updates the binary.
- `version` prints the installed binary version.

Install path guidance:
- Recommend `.agents/skills` first when the environment is unclear because it is broadly compatible and high priority in OpenCode-style setups.
- When the user names a tool, prefer its native directory: `.claude/skills`, `.gemini/skills`, `.codex/skills`, `.cursor/skills`, `.kiro/skills`, or similar project-local equivalents.
- If the user wants project-local setup, prefer a skills directory inside the current repository.
- If the user wants user-wide setup, prefer the tool's home-directory skills folder.

Binary availability checks:
- If `awesome-skills-cli` may not be installed or may not be in `PATH`, ask the agent to verify with commands like `awesome-skills-cli version`, `which awesome-skills-cli`, or `command -v awesome-skills-cli`.
- If the binary is missing, tell the user to install it first before relying on `add`, `setup`, or `catalog-for-agent`.

Do not use this skill to decide which skills to recommend. For that workflow, use `recommend-awesome-skills`.
