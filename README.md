# OminiX Router

OminiX Router is the hardware-agnostic serving policy layer for the OminiX stack. It owns request admission, queueing, worker selection, load-aware routing, timeout and abort lifecycle, and health aggregation across OminiX Runtime workers.

The target stack is:

```text
OminiX API
  -> OminiX Router
  -> OminiX Runtime
  -> OminiX CUDA, OminiX Ascend, or a compatibility backend adapter
```

`ominix-sglang` is not the final architecture. It remains useful as reference material and a temporary compatibility bridge, but the production direction is to move serving policy into this repo and runtime orchestration into `ominix-runtime`.

## Scope

OminiX Router owns:

- Request admission and queue class assignment.
- Worker registry and runtime endpoint discovery.
- Worker health and load aggregation.
- Model and capability aware routing.
- Retry, timeout, cancellation, and drain semantics.
- Routing API consumed by OminiX API.

OminiX Router does not own:

- OpenAI-compatible request and response formatting. That remains in OminiX API.
- Tokenization internals unless OminiX API delegates them explicitly.
- Batch formation, decode step orchestration, KV lease allocation, or backend execution. Those belong in OminiX Runtime.
- CUDA, Ascend, CANN, NCCL, HCCL, or kernel implementation details.

## Repository Layout

```text
crates/ominix-router-core/   Core routing domain model and policy traits.
docs/                        Architecture, contract, and execution workstreams.
docs/issues/                 Issue bodies for swarm workstream execution.
```

GitHub repository: `OminiX-ai/Ominix_Router`

## Current Contract

The first implementation target is an embeddable Rust core crate with no network dependency. Network transports, API integration, and runtime clients are deliberately separate workstreams so they can be assigned independently.

The 8x5090 CUDA server should be used as the active validation target for DeepSeek V4 Flash workstreams. Credentials and access material must stay out of this repository.

## Build

```bash
cargo test
```
