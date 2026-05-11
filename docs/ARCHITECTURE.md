# OminiX Router Architecture

## Purpose

OminiX Router is the serving policy layer between OminiX API and OminiX Runtime. It provides a stable routing surface while CUDA, Ascend, SGLang compatibility adapters, and future backends evolve independently.

The design goal is to keep hardware-independent policy in one place and push hardware-coupled scheduling into OminiX Runtime and backend adapters.

## Stack Boundary

```text
Client
  -> OminiX API
     - OpenAI-compatible API surface
     - request parsing and response streaming
     - auth, model listing, user-facing errors
  -> OminiX Router
     - admission
     - queueing
     - worker registry
     - load-aware routing
     - health and drain state
     - retry, timeout, abort lifecycle
  -> OminiX Runtime
     - request state machine
     - prefill/decode plan
     - KV lease contract
     - backend adapter ABI
  -> Backend
     - OminiX CUDA
     - OminiX Ascend
     - temporary SGLang compatibility backend
```

## Why Retire OminiX-SGLang

The `ominix-sglang` bridge proved that OminiX API can call into an optimized inference engine, but it keeps too much of the production path behind SGLang internals. The new split keeps:

- OminiX API as the public serving API.
- OminiX Router as the hardware-agnostic serving policy plane.
- OminiX Runtime as the backend-facing model execution control plane.
- OminiX CUDA and OminiX Ascend as hardware implementation planes.

SGLang remains useful as a reference implementation and optional compatibility backend, not as the core OminiX serving abstraction.

## Router Responsibilities

Router responsibilities are intentionally narrow:

- Accept normalized requests from OminiX API.
- Decide whether a request is admitted immediately, queued, or rejected.
- Select a runtime worker based on model, capabilities, load, health, and policy.
- Keep worker registry state fresh.
- Retry or fail requests based on explicit timeout and error contracts.
- Propagate aborts from OminiX API to OminiX Runtime.
- Drain workers safely for maintenance and rollout.

## Non-Responsibilities

The router must not absorb runtime behavior:

- It does not form GPU/NPU batches.
- It does not allocate KV blocks.
- It does not run token sampling.
- It does not understand CUDA streams, CANN streams, NCCL, HCCL, or kernel launch details.
- It does not own model weights.

## Key Interfaces

The initial interface is `ominix-router-core`, an embeddable Rust crate with pure domain types and routing policy traits. Transport layers can wrap it without changing the policy contract.

Required future interfaces:

- Runtime worker registration API.
- Runtime worker heartbeat API.
- Runtime load snapshot API.
- Request route API from OminiX API.
- Abort and timeout propagation API.

## DeepSeek V4 Flash Execution Target

DeepSeek V4 Flash validation should run against the provisioned 8x5090 CUDA host. Access credentials are not stored in the repository. The router should treat the CUDA workers as TP8-capable runtime endpoints exposed by OminiX Runtime.

## Success Criteria

- OminiX API can call OminiX Router without SGLang-specific API assumptions.
- Router can choose among multiple runtime workers using health and load.
- Router can reject requests cleanly when capacity is unavailable.
- Router can drain a worker without accepting new requests on it.
- Router can propagate aborts and timeouts to OminiX Runtime.
- All policy behavior is covered by deterministic unit tests.

