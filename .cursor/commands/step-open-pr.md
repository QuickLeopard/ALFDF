---
name: step-open-pr
description: Open or update PR — clean, pushed branch (ALFDF)
---

You are in **ALFDF**. Enforce **one step = one PR** ([`.cursor/rules/70-commit-policy.mdc`](../rules/70-commit-policy.mdc)).

1. Read and follow [`.cursor/skills/step-ship.md`](../skills/step-ship.md) **PR mode** only.
2. Before `gh pr create`, check whether a PR already exists for this head branch (`gh pr view` / `gh pr list --head …`); if yes, update description with `gh pr edit` instead of opening a duplicate.
3. PR title: `[STEP-<id>] <short title>`. PR body must include real outcomes only ([`.cursor/system.md`](../system.md): no invented hashes, timings, or benchmark numbers). Add dependency justification if `Cargo.toml` / `Cargo.lock` changed.
4. If the tree is dirty or commits are not on `origin`, stop and **ask** — do not guess from slash text.
