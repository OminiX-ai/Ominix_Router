# OR-005 Runtime Client Adapter

## Objective

Define the client boundary from OminiX Router to OminiX Runtime.

## Scope

- Define a request handoff interface.
- Define stream event handling.
- Define final status handling.
- Define abort propagation.
- Add a fake runtime client for integration tests.

## Acceptance Criteria

- The client boundary imports no CUDA, Ascend, CANN, NCCL, HCCL, or SGLang internals.
- Fake runtime client can simulate success, pre-token failure, post-token failure, timeout, and abort.
- OminiX API can call the adapter in a local test.

## Agent Notes

Keep the interface compatible with `ominix-runtime` issue ORT-003 and ORT-004.

