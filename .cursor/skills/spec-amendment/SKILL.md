---
name: spec-amendment
description: Amends .DOCS/ALFDF-MVP0-Project-Spec-v0.1.md via a dedicated STEP-id.amend step and ADR. Use only when a guide step uncovers a genuine spec flaw; never mix with production code commits.
---

# Skill: Amend ALFDF MVP0 project spec

WHEN Only when a step uncovers a genuine spec flaw.

PROCEDURE
1. Create a dedicated step `STEP-<id>.amend` in `.DOCS/ALFDF-MVP0-Stepbystep-Guide-v0.md`.
2. Edit `.DOCS/ALFDF-MVP0-Project-Spec-v0.1.md`; add an ADR under `docs/adr/` recording the change and rationale.
3. If the wire format or JSON Schema changes, bump schema major and add migration notes in the ADR.
4. PR title: `[STEP-<id>.amend] spec: <summary>`. No production code change in the same commit.
