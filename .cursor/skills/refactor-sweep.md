# Skill: YAGNI / DRY refactor sweep (scheduled per phase)

PROCEDURE
1. Run `cargo udeps`, `scripts/jscpd_gate.sh`, unused-pub lint.
2. Remove every `pub` without a caller; inline trivial one-shot abstractions.
3. Extract true duplication only (not coincidental similarity).
4. Commit as `refactor(<crate>): STEP-<id> sweep` — tests must stay green.
