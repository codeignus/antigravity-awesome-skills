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
4. Once context and install scope are clear, run chunked map-reduce catalog workflow.
5. Present aggregated shortlist with reasons.
6. Ask for confirmation before installing anything.
7. If the user confirms, install with `awesome-skills-cli add --path <dir> <skill-id...>`.

Question order:
1. What kind of work are you doing right now?
2. Which coding environment or agent are you using?
3. If the user has not already said, ask: should I set this up for just this project or globally for your machine?
4. If needed, confirm the target directory.

Installation directory guidance:
- Recommend `.agents/skills` first when the environment is unknown or mixed because it is the most universal and highest-priority default.
- If the environment is known, suggest a matching directory such as `.claude/skills`, `.gemini/skills`, `.codex/skills`, `.cursor/skills`, or `.kiro/skills`.
- For project-based setup, prefer a repo-local directory.
- For global setup, prefer the environment's home-directory skills folder.

## Chunked Map-Reduce Workflow

Use `awesome-skills-cli catalog-for-agent --limit 250 --offset X` to pull the catalog in sequential chunks.

### Step 1: Probe for total

Run `awesome-skills-cli catalog-for-agent --limit 1 --offset 0`. The command emits pagination metadata on stderr in the format:

```
catalog-for-agent: total=<N> offset=0 limit=1 returned=1 truncated=false has_more=true
```

Extract `total` from stderr. Compute the number of chunks: `ceil(total / 250)`. For example, `total=1329` with limit 250 produces 6 chunks at offsets 0, 250, 500, 750, 1000, 1250.

### Step 2: Map — dispatch chunks

Dispatch one subagent per chunk. Each subagent receives the user's requirements and the chunk JSON and returns only a shortlist of relevant skill IDs with one-line reasons.

Where parallel subagents are not available, process chunks sequentially in the main agent: evaluate each chunk, keep only the shortlist, discard the chunk data before moving to the next.

### Step 3: Reduce — aggregate shortlists

Collect all chunk-level shortlists. Deduplicate by skill ID. Compare relevance across chunks. Produce the final best set before presenting anything to the user.

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
