---
name: recommend-awesome-skills
description: Use when user asks for awesome skill recommendations, wants help choosing skills for an environment or project, or wants skills installed.
---

# recommend-awesome-skills

Use this skill when the user wants help choosing awesome skills for their current project, editor, or agent environment.

Primary workflow:
1. Ask context questions first.
2. If the user has not already specified install scope, ask whether installation should be project-based or global before loading the catalog.
3. Decide on target directory before loading the catalog.
4. Once context and install scope are clear, determine whether to use targeted search or the full catalog workflow.
5. Present aggregated shortlist with reasons.
6. Ask for confirmation before installing anything.
7. If the user confirms, install with `awesome-skills-cli add --path <dir> <skill-id...>`.

Question order (SKIP any already answered by context or prompt):
1. What kind of work are you doing right now?
2. Which coding environment or agent are you using?
3. CRITICAL: Should I set this up for just this project or globally for your machine?
4. If needed, confirm the target directory.

Installation directory defaults:
- **Global (mixed/unknown env)**: `~/.agents/skills` (highest-priority universal default)
- **Global (known env)**: e.g., `~/.claude/skills`, `~/.gemini/skills`, `~/.cursor/skills`
- **Project-based**: Repo-local directory (e.g., `./.agents/skills`)

## Catalog Search vs Browse

### Explicit Searches
Use `awesome-skills-cli search <keyword>` **ONLY** when the user explicitly asks for a narrow, specific tool or workflow by name (e.g., "I want a brainstorming skill", "get me the postgres skill").
Do NOT use this just because you see specific frameworks like `vite` or `playwright` in their workspace. Environmental context should be used for filtering during a browse, not for triggering individual search commands.

### General Discovery
Use `awesome-skills-cli list --limit 250 --offset <N>` to evaluate the *entire* catalog against the user's stack when they ask for general recommendations (e.g., "what should I install?").

**CRITICAL BROWSING RULES:**
1. Limit MUST NEVER exceed `250`.
2. **DO NOT** use `jq`, `python`, or bash scripts to parse outputs.
3. To bypass terminal truncation, **MUST** dump chunks to a file (e.g. `> /tmp/chunk.txt`) and read via your native file-reading tool (`read_file`/`view_file`).

#### Step 1: Probe for total
Run `awesome-skills-cli list --limit 1 --offset 0 2>&1 >/dev/null` to cleanly surface metadata. Extract `total` to compute 250-item chunks (offsets 0, 250, 500...).

#### Step 2: Map (Chunk to file & read)
**STRONGLY ADVISE** dispatching one subagent per chunk to preserve your context window. Each runs `list --limit 250 --offset <N> > /tmp/chunk_<N>.txt`, reads it natively, and returns a shortlist.
**If subagents are unavailable**, process sequentially: dump to `/tmp/chunk.txt`, read into context, extract relevant IDs, and critically, purge the chunk data from your memory before pulling the next offset.

#### Step 3: Reduce
Collect and deduplicate shortlists. Use `has_more` metadata to know when chunks are exhausted. Produce the final best set.

### Step 4: Present and confirm

Show the aggregated recommendations with reasons. Ask for confirmation before installing.

Catalog usage rules:
- Do not fetch the catalog before asking the install-scope and context questions.
- Do not dump the entire catalog to the user.
- Use the condensed catalog only to choose a small, relevant shortlist.
- Keep recommendations tied to the user's current work, not generic popularity.

Installation rules:
- Never install skills without explicit confirmation.
- When the user approves, install only the agreed skill IDs.
- If the user also needs help using the CLI itself or troubleshooting whether the binary is installed, load `awesome-skills-cli` for command and PATH guidance.

Keep this skill focused on recommendation and installation flow. Do not spend tokens on CLI troubleshooting unless that becomes necessary.
