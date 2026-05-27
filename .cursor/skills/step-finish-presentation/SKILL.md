---
name: step-finish-presentation
description: Creates docs/step-reviews/STEP-id artifacts and marks the step Done in the ALFDF step guide. Use before closing a guide step PR or when rule 80 step presentation is required.
---

# Skill: Step-finish presentation and review

WHEN After implementation for a guide step is complete and `just verify` (or the project-equivalent) is green, before marking the step’s `- [ ] Done` checkbox in `.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md` or opening the step PR for final review. The step review folder **and** the guide checkbox update belong in the **same** step PR (one step = one PR); do not document “merge then tick the guide on `main`” as a follow-up. If a step was split, each sub-step PR gets its own folder.

PROCEDURE
1. Resolve the step id `STEP-<id>` from the guide heading (e.g. `### STEP-A1 — …`) and commit footer (`Refs: .DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md#STEP-<id>`).
2. Create `docs/step-reviews/STEP-<id>/` using that id, e.g. `STEP-A1` → `docs/step-reviews/STEP-A1/`. Split steps: `STEP-A1a`, `STEP-A1b`. Amendments: `STEP-A1.amend`.
3. Add `README.md` in that folder. Required sections (headings or equivalent):
   - Step id and title
   - What shipped (bullets, file paths where helpful)
   - Whole-project progress (phase, step tally, or pointer to checklist lines in `.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md`)
   - Commands run — exact command lines and pass/fail (no invented output)
   - Tests — what ran and result
   - Benchmarks — budgets/results when the step defined benches; otherwise `Benchmarks: not applicable` with a one-line reason tied to the step
   - Risks and follow-ups
   - **Review** — what was verified, what to watch next, deferrals with links (issue/ADR) when applicable
4. Add `slides.md` in the same folder: a **concise** slide deck (not a copy-paste of `README.md`). Use `---` between slides if using Marp, or one `##` slide title per slide in plain Markdown. Minimum coverage: title; what changed; project progress; tests/benches; review/follow-ups.
5. In the **same** PR, add or set under the step heading in `.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md` a `- [x] Done` line (create `- [ ] Done` if missing) when the step is complete. Do not leave “merge then tick the guide on `main`” as a documented follow-up.
6. Optionally export `slides.md` to `presentation.html` or `presentation.pdf` when tooling already exists in the repo or environment. Do **not** add a new dependency or toolchain solely to export unless the PR body documents justification per project dependency rules.
7. PR description must link `docs/step-reviews/STEP-<id>/` and repeat the command list with pass/fail summary. Align with anti-fabrication: no fake hashes, timings, or benchmark numbers. PR title format per guide: `[STEP-<id>] <short title>`.

CHECKLIST
- [ ] Folder exists: `docs/step-reviews/STEP-<id>/`
- [ ] `README.md` present with mandatory **Review** section (no deferral of the guide checkbox to a post-merge PR; same PR as implementation)
- [ ] `slides.md` present as the canonical deck
- [ ] `.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md`: this step’s checkbox is `- [x] Done` in this PR when the step is complete
- [ ] Optional `presentation.html` / `presentation.pdf` only if exported without new deps (unless justified)
