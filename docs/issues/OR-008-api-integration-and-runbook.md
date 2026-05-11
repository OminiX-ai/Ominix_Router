# OR-008 OminiX API Integration and Runbook

## Objective

Integrate OminiX API with OminiX Router and document the validation runbook.

## Scope

- Map OminiX API generation requests to router requests.
- Map router errors to OminiX API errors.
- Route successful requests to a fake runtime first.
- Document the 8x5090 CUDA validation path without storing credentials.

## Acceptance Criteria

- OminiX API can run a local fake-runtime integration test through router.
- The runbook covers local tests, router logs, runtime handoff, and DeepSeek V4 Flash validation.
- No credentials are committed.

## Agent Notes

Use the provisioned 8x5090 CUDA host for final validation once runtime and backend issues are ready.

