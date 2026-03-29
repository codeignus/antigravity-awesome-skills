---
name: recommend-awesome-skills
description: Use when the user asks for awesome skill recommendations, wants help choosing skills for an environment or project, or wants skills installed.
---

# recommend-awesome-skills

Use this skill when the user wants help choosing awesome skills for their current project, editor, or agent environment.

Primary workflow:
1. Ask context questions first.
2. If the user has not already specified install scope, ask whether installation should be project-based or global before loading the catalog.
3. Decide the target directory before loading the catalog.
4. Once the context and install scope are clear, prefer using a subagent to run `awesome-skills-cli catalog-for-agent` and return only a focused shortlist.
5. Recommend a focused set of skills with short reasons.
6. Ask for confirmation before installing anything.
7. If the user confirms, install with `awesome-skills-cli add --path <dir> <skill-id...>`.

Question order:
1. What kind of work are you doing right now?
2. Which coding environment or agent are you using?
3. If the user has not already said, ask: should I set this up for just this project or globally for your machine?
4. If needed, confirm the target directory.

Installation directory guidance:
- Recommend `.agents/skills` first when the environment is unknown or mixed because it is the most universal and highest-priority default.
- If the environment is known, suggest the matching directory such as `.claude/skills`, `.gemini/skills`, `.codex/skills`, `.cursor/skills`, or `.kiro/skills`.
- For project-based setup, prefer a repo-local directory.
- For global setup, prefer the environment's home-directory skills folder.

Catalog usage rules:
- Do not fetch `catalog-for-agent` before asking the install-scope and context questions.
- Do not dump the entire catalog to the user.
- Use the condensed catalog only to choose a small, relevant shortlist.
- Prefer running the catalog search in a subagent so the large metadata load does not consume the main session context beyond the final shortlist.
- If the catalog was loaded in the main session and the conversation starts drifting into unrelated work, warn that the large catalog context should not be carried into other tasks.
- Keep recommendations tied to the user's current work, not generic popularity.

Installation rules:
- Never install skills without explicit confirmation.
- When the user approves, install only the agreed skill IDs.
- If the user also needs help using the CLI itself or troubleshooting whether the binary is installed, load `awesome-skills-cli` for command and PATH guidance.

Keep this skill focused on recommendation and installation flow. Do not spend tokens on CLI troubleshooting unless that becomes necessary.
