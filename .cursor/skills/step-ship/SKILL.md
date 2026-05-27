---
name: step-ship
description: Commits, pushes, and opens PRs for ALFDF guide steps (RED/GREEN/docs modes). Use when the user says commit, push, open PR, ship this step, or finish this guide step.
---

# Skill: Step ship — commit, push, PR

Use when the user says: **commit**, **push**, **open PR**, **create PR**, **ship this step**, or **finish this guide step**.

Do **not** run `git commit` unless the user explicitly asked to **commit** or used **ship this step** / **finish this guide step**.

Hard refs: [.cursor/system.md](../../system.md), [.cursor/rules/70-commit-policy.mdc](../../rules/70-commit-policy.mdc), [step-finish-presentation](../step-finish-presentation/SKILL.md), [80-step-presentation.mdc](../../rules/80-step-presentation.mdc), [split-step](../split-step/SKILL.md).

---

## Modes (pick by user intent)

### RED commit mode

- **When:** User asked to commit a **failing** test first (TDD RED), or the change set is only `test(...)` / test fixtures.
- **Subject:** `test(<scope>): STEP-<id> …` with real step id (e.g. `STEP-B2`). Scope = crate or area touched.
- **Verify:** Run the **narrowest** test that proves RED (e.g. `cargo test -p <crate> <filter>`). **Do not** require `just verify` green on the whole workspace.
- **Stage:** Only test/support files needed for RED. Avoid staging production `feat`/`fix` code in the same commit unless it is unavoidable stub wiring and the commit type remains `test(...)`.
- **Footer (recommended):** `Refs: .DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md#STEP-<id>`
- **Commit:** Use HEREDOC for multi-line body when needed.

### GREEN / refactor commit mode

- **When:** User asked to commit implementation that makes tests pass, or a refactor with tests still green.
- **Subject:** `feat(<crate>): STEP-<id> …`, `fix(<crate>): STEP-<id> …`, or `refactor(<crate>): STEP-<id> …` per [system.md](../../system.md).
- **Verify:** `just verify` **must** be green before commit.
- **Order:** When `scripts/tdd-order.sh` exists (STEP-A2+), new `crates/**/src/**/*.rs` must follow it (preceding `test(...):` commit when required). Until then, keep RED before GREEN commits manually.
- **Footer (recommended):** `Refs: .DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md#STEP-<id>`

### Step review / docs commit mode

- **When:** Step presentation and guide checkbox are ready to land in the **same** PR as the step (rule 80).
- **Follow:** [step-finish-presentation](../step-finish-presentation/SKILL.md) in full.
- **Verify:** `just verify` green before commit.
- **Subject:** `docs(step-review): STEP-<id> …` or `docs(step-guide): STEP-<id> …` as appropriate.

### Push mode

- **When:** User asked to **push**.
- **Working tree:** Must be **clean** (no unstaged/uncommitted changes) unless the user also asked to **commit** in the same instruction; if dirty, stop and ask.
- **Branch:** Prefer `step/STEP-<id>-<slug>`. If on another branch for a guide step, stop and ask whether to rename or create the correct branch ([70-commit-policy.mdc](../../rules/70-commit-policy.mdc)).
- **Command:** `git push -u origin HEAD` when upstream missing; otherwise `git push`.
- **Never** force-push `main`.

### PR mode (create / update)

- **When:** User asked **open PR** / **create PR**.
- **Working tree:** Must be **clean** and commits **pushed** to `origin`. If dirty or unpushed, stop and ask (unless user explicitly combined “commit, push, open PR”).
- **Gather (before `gh pr create`):** `git status`, `git diff`, `git log origin/main..HEAD --oneline`, `git diff origin/main...HEAD`, and confirm branch tracks `origin/<branch>`.
- **Title:** `[STEP-<id>] <short title>` per `.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md`.
- **Body sections:** Summary; Test plan (checkboxes, real `just verify` / targeted test outcomes — **no invented** hashes, timings, or benchmark numbers per [system.md](../../system.md)); **Dependency justification** if `Cargo.toml` / `Cargo.lock` changed; **Benchmark evidence** only from actual runs if claiming numbers; link `docs/step-reviews/STEP-<id>/README.md`.
- **Command:** `gh pr create --base main` (or `gh pr edit` if PR already exists for this branch).

### Ship this step (combined)

Run in order when the user says **ship this step** or **finish this guide step**:

1. Ensure implementation + tests complete; `just verify` green.
2. Ensure [step-finish-presentation](../step-finish-presentation/SKILL.md) artifacts exist and guide checkbox updated in the same PR.
3. Commit any remaining docs (docs commit mode).
4. Push mode.
5. PR mode.

---

## Stop conditions (do not proceed silently)

- User asked only **open PR** but `git status` is not clean → stop; ask to commit or stash.
- **GREEN / docs / PR** mode while `just verify` is red → stop; fix or report.
- Completing a guide step without `docs/step-reviews/STEP-<id>/README.md` + `slides.md` → stop; follow [step-finish-presentation](../step-finish-presentation/SKILL.md).
- New crates.io / workspace dependency without PR-body justification + license note → stop; [.cursor/system.md](../../system.md) rule 7.
- Production diff likely > 300 LOC (tests excluded) → stop; invoke [split-step](../split-step/SKILL.md) / [70-commit-policy.mdc](../../rules/70-commit-policy.mdc).

---

## Hooks policy

- **Do not** add Cursor hooks that auto-commit, auto-push, or auto-open PRs.
- Optional **future** hardening (only if the user requests it): `beforeShellExecution` hook to **block or ask** on dangerous patterns (`git push --force`, `git commit --no-verify`, `gh pr merge --admin`). Hooks must not stage files or complete workflows on behalf of the user.
