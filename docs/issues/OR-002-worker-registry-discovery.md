# OR-002 Worker Registry and Discovery

## Objective

Implement the worker registry and discovery contract for OminiX Runtime workers.

## Scope

- Add a registry module to `ominix-router-core` or a sibling crate if transport code is required.
- Track worker endpoint, runtime namespace, model IDs, capabilities, health, and load.
- Support add, update, drain, offline, and remove.
- Define stale heartbeat behavior.

## Acceptance Criteria

- Registry state transitions are covered by tests.
- Draining workers are visible but do not accept new requests.
- Offline workers are not routable.
- Registry output can feed the routing policy without translation loss.

## Agent Notes

The registry must be ready for multiple runtime backends: OminiX CUDA, OminiX Ascend, and a temporary compatibility adapter.

