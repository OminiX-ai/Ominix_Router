# OR-007 Health and Load Aggregation

## Objective

Implement health and load aggregation across runtime workers.

## Scope

- Add timestamped load snapshots.
- Add health transition events.
- Track stale worker state.
- Provide an aggregate view for observability and routing.

## Acceptance Criteria

- Tests cover heartbeat freshness and stale workers.
- Draining and unhealthy workers are excluded from new routing decisions.
- Aggregated model capacity can be reported to OminiX API.

## Agent Notes

This issue should prepare the router for multiple OminiX Runtime workers serving DeepSeek V4 Flash on the 8x5090 CUDA target.

