# Router Contract

This contract defines the first stable boundary for OminiX Router.

## Inbound Request

OminiX API sends a normalized request containing:

- Request ID.
- Model ID.
- Prompt token count or tokenized prompt handle.
- Maximum new tokens.
- Priority.
- Required capabilities, such as `cuda`, `ascend`, `tp8`, `prefill`, or `decode`.
- Deadline and timeout policy.
- Streaming or non-streaming response mode.

The router does not parse OpenAI JSON directly in the core crate. OminiX API owns that translation.

## Admission

The router returns one of:

- Admit immediately.
- Queue with a queue class.
- Reject with a structured reason.

Admission policy may use global queue size, per-model queue size, per-tenant policy, deadline pressure, and runtime worker availability.

## Worker Registry

A worker descriptor contains:

- Worker ID.
- Runtime namespace.
- Supported model IDs.
- Supported capabilities.
- Health state.
- Load snapshot.

The registry must support `Starting`, `Available`, `Draining`, `Unhealthy`, and `Offline` states.

## Routing Decision

A routing decision contains:

- Request ID.
- Primary worker target.
- Optional alternative targets.
- Estimated wait time.
- Human-readable decision reason for debugging.

Routing must be deterministic for a fixed input snapshot unless a policy explicitly includes randomization.

## Runtime Handoff

After routing, OminiX API or the router transport layer hands the request to OminiX Runtime. The handoff must include:

- Request ID.
- Model ID.
- Tokenized input or tokenizer-owned handle.
- Generation parameters.
- Deadline.
- Abort handle.
- Streaming response sink.

## Abort and Timeout

The router must expose explicit abort and timeout propagation. A request that times out in OminiX API must produce a runtime abort. A worker that fails during decode must mark the request as failed and allow policy-driven retry only when the runtime reports that retry is safe.

## Retry

Retry is allowed only for:

- Requests not yet accepted by runtime.
- Requests that failed before emitting user-visible tokens.
- Requests that runtime marks retryable.

Retry is not allowed after partial output unless OminiX API opts into a specific recovery mode.

## Observability

The router must emit:

- Admission decision.
- Queue wait duration.
- Worker selected.
- Worker load snapshot at selection time.
- Runtime handoff latency.
- Abort and timeout events.
- Final request state.

## Compatibility Rule

The router may call a temporary SGLang compatibility runtime adapter, but no router core API may depend on SGLang Python types, SGLang scheduler internals, or SGLang-specific KV structures.

