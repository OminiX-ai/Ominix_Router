# OR-004 Load-Aware Routing

## Objective

Extend routing beyond the baseline least-loaded policy.

## Scope

- Use running request count, queue depth, waiting token estimate, KV utilization, health, and decode throughput estimate.
- Respect model and capability constraints.
- Respect worker drain state.
- Return primary and alternative targets.

## Acceptance Criteria

- Tests cover model mismatch, capability mismatch, unhealthy workers, full KV capacity, and worker drain.
- Routing decisions include a reason string.
- Policy produces deterministic output for a fixed worker snapshot.

## Agent Notes

This is hardware agnostic. Runtime-specific scheduling stays in `ominix-runtime`.

