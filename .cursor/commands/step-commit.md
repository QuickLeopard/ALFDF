---
name: step-commit
description: Commit only — ALFDF step workflow (RED/GREEN/docs)
---

You are in **ALFDF**. Enforce **one step = one PR**, branch `step/STEP-<id>-<slug>`, and step id in every subject ([`.cursor/rules/70-commit-policy.mdc`](../rules/70-commit-policy.mdc)).

1. Read and follow [`.cursor/skills/step-ship/SKILL.md`](../skills/step-ship/SKILL.md): use **RED**, **GREEN / refactor**, or **Step review / docs** mode only. **Do not** push or open a PR unless the user asks separately.
2. If step id (`STEP-<id>`), scope (`test(<scope>):` / `feat(<crate>):`), or RED vs GREEN vs docs is unclear, **ask** — do not guess from extra slash text (no argument contract).
3. Use HEREDOC `git commit` bodies when needed; add `Refs: .DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md#STEP-<id>` when you know the id.
4. Obey all **Stop conditions** in `step-ship/SKILL.md` before any commit.
