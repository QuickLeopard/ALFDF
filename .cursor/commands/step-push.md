---
name: step-push
description: Push only — clean tree, step branch (ALFDF)
---

You are in **ALFDF**. Enforce **one step = one PR** and branch `step/STEP-<id>-<slug>` ([`.cursor/rules/70-commit-policy.mdc`](../rules/70-commit-policy.mdc)).

1. Read and follow [`.cursor/skills/step-ship.md`](../skills/step-ship.md) **Push mode** only.
2. **Do not** run `git commit` unless the user also explicitly asked to commit in the same turn.
3. If the branch is not a `step/…` branch or the tree is dirty, follow `step-ship.md` stop rules and **ask** — do not guess.
4. Never force-push `main`.
