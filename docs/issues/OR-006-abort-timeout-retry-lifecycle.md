# OR-006 Abort, Timeout, Retry Lifecycle

## Objective

Implement request lifecycle rules for aborts, deadlines, and retries.

## Scope

- Propagate API aborts to runtime.
- Convert request deadline expiry to runtime abort.
- Permit retry only before runtime acceptance, before first emitted token, or when runtime explicitly marks retry safe.
- Emit structured lifecycle events.

## Acceptance Criteria

- Tests cover abort before runtime handoff, abort during prefill, abort during decode, and timeout.
- Retry is blocked after partial output by default.
- Lifecycle events are observable by OminiX API.

## Agent Notes

Do not hide runtime failures behind transparent retry once output has reached the user.

