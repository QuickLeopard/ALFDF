---
name: find-skills
description: Discovers skills for ALFDF tasks—checks this repo's .cursor/skills first, then the open skills ecosystem via npx skills. Use when the user asks how to do X, wants a skill for X, or asks to find or install agent skills.
---

# Find skills (ALFDF)

Help users find capability extensions for the **ALFDF** Rust workspace. Always prefer **existing project skills** before installing from the ecosystem.

## When to use

- "How do I …", "find a skill for …", "is there a skill that …"
- User wants to extend agent workflows for this repo
- User asks about `@find-skills` or installing skills

## Step 0 — Check in-repo skills first (mandatory)

Before `npx skills find`, map the request to skills already in [`.cursor/skills/`](../):

| Skill | Use when |
|-------|----------|
| [step-ship](../step-ship/SKILL.md) | Commit, push, open PR, ship a guide step |
| [step-finish-presentation](../step-finish-presentation/SKILL.md) | Step review `README.md` + `slides.md`, guide checkbox |
| [spec-amendment](../spec-amendment/SKILL.md) | Change `.DOCS/ALFDF-MVP0-Project-Spec-v0.1.md` |
| [split-step](../split-step/SKILL.md) | PR > 300 LOC production or size limits hit |
| [refactor-sweep](../refactor-sweep/SKILL.md) | Phase-end YAGNI/DRY sweep |
| [jscpd](../jscpd/SKILL.md) | Copy-paste / duplication detection (`just jscpd`) |
| [dry-refactoring](../dry-refactoring/SKILL.md) | Remove clones after jscpd |
| [find-skills](SKILL.md) | Discover **additional** external skills (this file) |

Also use [.cursor/rules/](../../rules/) and [.cursor/system.md](../../system.md) for project law.

If an in-repo skill fits, **use it** — do not install an external duplicate.

## Step 1 — Understand the need

1. Domain (Rust, MCP, CI, tests, docs, …)
2. Task (specific outcome)
3. Whether it belongs in a **guide step** (`STEP-<id>`) vs ad-hoc tooling

For ALFDF implementation work, default to [.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md](../../../.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md) — not a random skill.

## Step 2 — External search (only if in-repo is insufficient)

**Skills CLI** — package manager for the open skills ecosystem: https://skills.sh/

```bash
npx skills find [query]
```

Rust / ALFDF-oriented examples:

- Duplication → `npx skills find copy paste rust` (project already has [jscpd](../jscpd/SKILL.md))
- CI / GitHub Actions → `npx skills find github actions rust`
- Property tests → `npx skills find proptest rust`

Optional: check the [skills.sh leaderboard](https://skills.sh/) for popular skills before CLI search.

## Step 3 — Verify before recommending

Do **not** recommend from search results alone:

1. **Install count** — prefer 1K+; caution under 100
2. **Source** — prefer known orgs; unknown authors need extra scrutiny
3. **Fit for ALFDF** — Rust workspace, no conflicting workflow (STEP-* PRs, `just verify`)
4. **Dependencies** — per [.cursor/system.md](../../system.md) rule 7: new crates.io/npm tools need PR justification + MIT/Apache-2.0

## Step 4 — Present options

Include: name, what it does, install count, **project-local** install command, skills.sh link.

Example (external skill):

```text
Skill: example-rust-ci — GitHub Actions patterns for Rust.
Install into this repo (not global):

  npx skills add owner/repo@skill-name -y

Browse: https://skills.sh/owner/repo/skill-name
```

## Step 5 — Install into this project (not global by default)

**Default:** project-local under `.cursor/skills/`:

```bash
cd /path/to/ALFDF
npx skills add <owner/repo@skill> -y
```

- **Do not** use `-g` unless the user explicitly wants a personal/global skill.
- After install, **adapt** the skill for ALFDF (paths, `just` commands, STEP-* commits) like [jscpd](../jscpd/SKILL.md) and [dry-refactoring](../dry-refactoring/SKILL.md).
- If the CLI updates [`skills-lock.json`](../../../skills-lock.json), commit it with the skill change.
- Run `just verify` (and `just jscpd` if the skill adds scripts) before proposing a PR.

## Step 6 — Create a project skill instead (often better)

If the workflow is ALFDF-specific and no good external skill exists:

```bash
# Optional scaffold; then edit under .cursor/skills/<name>/SKILL.md
npx skills init my-skill-name
```

Or add `.cursor/skills/<name>/SKILL.md` manually per Cursor format (`name`, `description` in frontmatter). Follow [create-skill](https://cursor.com/docs) conventions: third-person description, WHAT + WHEN.

For spec or guide changes use [spec-amendment](../spec-amendment/SKILL.md) — do not encode spec edits only in a generic skill.

## ALFDF constraints (always)

- One guide step = one PR; step id in commits (`STEP-<id>`)
- No new dependency without PR-body justification
- Do not install skills that encourage skipping TDD, `just verify`, or step-review folders
- External skills that add `npm`/`cargo` deps may require a dedicated guide step (e.g. STEP-A4 for workspace deps)

## When nothing fits

1. Say no matching in-repo or external skill was found
2. Offer to help directly using project rules and docs
3. Suggest a new `.cursor/skills/<name>/` skill if the task will repeat

## Related

- [skills-lock.json](../../../skills-lock.json) — tracks CLI-installed skills (`jscpd`, `dry-refactoring`)
- [05-tech-stack.mdc](../../rules/05-tech-stack.mdc) — pinned toolchain and expected gates
