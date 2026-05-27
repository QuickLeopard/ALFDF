---
name: refactor-sweep
description: Phase-end ALFDF refactor sweep using just jscpd, cargo udeps, and unused-pub checks. Use at end of a guide phase or when cleaning YAGNI/DRY debt across the workspace.
---

# Skill: YAGNI / DRY refactor sweep (scheduled per phase)

PROCEDURE
1. Run `just jscpd` (see [jscpd](../jscpd/SKILL.md)); on clones use [dry-refactoring](../dry-refactoring/SKILL.md). Then `cargo udeps` (when wired in CI), unused-pub lint.
2. Remove every `pub` without a caller; inline trivial one-shot abstractions.
3. Extract true duplication only (not coincidental similarity).
4. Commit as `refactor(<crate>): STEP-<id> sweep` — tests must stay green.
