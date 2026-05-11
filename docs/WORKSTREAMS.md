# Router Workstreams

These workstreams are designed for parallel agent execution.

## OR-001 Router Core Domain Model

Implement and harden core request, worker, health, load, admission, and routing decision types.

Acceptance:

- `cargo test` passes.
- Types are dependency-light and transport agnostic.
- Unit tests cover happy path and rejection states.

## OR-002 Worker Registry and Discovery

Build the registry that stores runtime worker endpoints and handles heartbeat updates.

Acceptance:

- Registry supports add, update, drain, offline, and remove.
- Stale heartbeat handling is deterministic.
- Tests cover concurrent update semantics at the API boundary.

## OR-003 Admission and Queueing Policy

Add queue classes and deadline-aware admission policy.

Acceptance:

- Per-model and global capacity limits exist.
- Queue class assignment is explicit.
- Rejection reasons are structured.

## OR-004 Load-Aware Routing

Extend routing policy beyond least-loaded baseline.

Acceptance:

- Policy can use queue depth, running requests, KV utilization, and estimated decode throughput.
- Policy handles model and capability mismatch.
- Policy supports worker drain.

## OR-005 Runtime Client Adapter

Define the client boundary from router to OminiX Runtime.

Acceptance:

- Client interface does not import CUDA, Ascend, or SGLang internals.
- Request handoff, stream events, abort, and final status are covered.
- Fake runtime client supports integration tests.

## OR-006 Abort, Timeout, Retry Lifecycle

Implement request lifecycle rules around aborts, deadlines, and retries.

Acceptance:

- Retry is blocked after user-visible output unless explicitly allowed.
- Timeout emits runtime abort.
- Tests cover runtime failure before and after first token.

## OR-007 Health and Load Aggregation

Add health snapshots and load aggregation for multi-worker deployments.

Acceptance:

- Load snapshots are versioned or timestamped.
- Health transitions are observable.
- Draining workers receive no new requests.

## OR-008 OminiX API Integration Runbook

Document and implement the integration from OminiX API to Router.

Acceptance:

- OminiX API can route a normalized generation request through router core.
- API errors map from router errors.
- Runbook includes local fake-runtime test and 8x5090 CUDA validation path.

