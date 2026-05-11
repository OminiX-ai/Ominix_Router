# OR-003 Admission and Queueing Policy

## Objective

Add admission and queueing policy so OminiX Router can reject overload early and classify admitted work.

## Scope

- Define queue classes for realtime, interactive, and batch traffic.
- Add per-model and global queue limits.
- Add deadline-aware admission hooks.
- Return structured rejection reasons.

## Acceptance Criteria

- Tests cover admit, queue, and reject paths.
- Queue class is visible in the routing trace.
- Admission policy is independent of backend hardware.
- OminiX API can map rejection reasons to user-facing errors.

## Agent Notes

Do not implement batch formation here. Batch planning belongs to `ominix-runtime`.

