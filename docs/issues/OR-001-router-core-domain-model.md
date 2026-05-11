# OR-001 Router Core Domain Model

## Objective

Harden the core Rust domain model for OminiX Router.

## Background

OminiX Router replaces the final architectural need for `ominix-sglang` as a serving policy layer. The router must expose clean request, worker, admission, and routing types that are independent of CUDA, Ascend, SGLang, and Python runtime internals.

## Scope

- Review `crates/ominix-router-core`.
- Add missing fields needed for OminiX API integration.
- Keep the crate transport agnostic.
- Keep the crate free of backend-specific dependencies.
- Add deterministic tests for each public policy path.

## Acceptance Criteria

- `cargo test` passes.
- Domain types cover request ID, model ID, worker ID, runtime namespace, health, load, admission, and route target.
- Rejection states are represented with typed errors.
- Docs explain which layer owns OpenAI API parsing, tokenization, and backend execution.

## Agent Notes

Do not add HTTP, gRPC, CUDA, CANN, or SGLang dependencies in this issue. Keep this issue limited to core semantics.

