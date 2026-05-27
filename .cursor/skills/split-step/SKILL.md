---
name: split-step
description: Splits an oversized ALFDF guide step into sub-steps when production diff exceeds 300 LOC or size limits are hit. Use before committing an oversized STEP-id change.
---

# Skill: Split a too-large step

TRIGGERS
- Production diff > 300 LOC.
- Any fn > 100 LOC or module > 500 LOC.

PROCEDURE
1. Stop. Do not commit the oversized change.
2. Edit `.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md`: replace `STEP-Xn` with `STEP-Xn.a`, `STEP-Xn.b`, … each within limits.
3. Commit `chore(plan): STEP-Xn split into a/b`.
4. Resume from `STEP-Xn.a`.
