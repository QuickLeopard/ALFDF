---
name: ship-step
description: Ship guide step — verify, docs/review, push, PR (ALFDF)
---

You are in **ALFDF**. This slash command is explicit intent to **commit (if needed), push, and open/update a PR** for the current guide step, subject to stop conditions.

1. Read and follow [`.cursor/skills/step-ship.md`](../skills/step-ship.md) **Ship this step (combined)** in order.
2. Enforce **one step = one PR**, `step/STEP-<id>-<slug>`, step id in subjects, and rule 80 (guide checkbox + `docs/step-reviews/STEP-<id>/` in the same PR) via [`.cursor/rules/70-commit-policy.mdc`](../rules/70-commit-policy.mdc) and [`.cursor/rules/80-step-presentation.mdc`](../rules/80-step-presentation.mdc).
3. If step id, review folder path, or PR title is missing, **ask** — do not parse trailing slash arguments.
4. Never force-push `main`; never skip `just verify` for non-RED work per `step-ship.md`.
