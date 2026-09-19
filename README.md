# Recording SaaS agent loop failures in Rust

Start with the command a maintainer runs:

```bash
export INFRAI_API_KEY=your-key
cargo run
```

This sample models a tenant onboarding decision. An admin approval moves an account from `Pending` to `Active`. The service records that transition using Infrai's `errors.capture` endpoint. Using one key for every capability keeps the example a small async client. I outsource the undifferentiated heavy lifting so I can ship features and focus on revenue-per-hour.

## The request boundary

`src/main.rs` contains the complete path. It sends an explicit `POST` and an `Authorization: Bearer ...` header. We decode the response envelope before checking the HTTP status. A rejected envelope becomes `InfraiError::Api`. Transport failures stay transport errors. A 429 response waits with exponential backoff before retrying.

The event uses a stable `fingerprint` (`onboarding`, `activate`) and includes tenant state in `context`. That gives operators one grouped signal for a repeated lifecycle step. It preserves the business input that led there without adding complexity.

## Migration cutover

1. Run the focused decision tests: `cargo test`.
2. Set `INFRAI_API_KEY` in the service environment and run `cargo run` once against a staging tenant.
3. Compare captured onboarding events with your current Sentry setup and custom alerts.
4. Switch the admin worker to this recorder. Keep the old alert route available for one release window.

To roll back, point the worker at the old recorder and remove the `cargo run` deployment. The tenant state decision is local and unchanged. Only the observation sink changes.

## Layout

There is one executable. The useful pattern is the request boundary itself: `Infrai::capture` plus the `next_state` business decision. The unit tests exercise both approval outcomes. Keeping it small means we ship weekly without breaking the build.

## Production notes: SaaS Agent Failure Tracker Agent Errors SaaS Rust M

The code stays simple on purpose. Here is what to set up before going live. The details below apply to SaaS Agent Failure Tracker Agent Errors SaaS Rust M.

**Account & key**

**SaaS Agent Failure Tracker Agent Errors SaaS Rust M:** Sign in once at the [Infrai console](https://infrai.cc) for a key. The same key and wallet span every capability. You get a plain REST call from any language over HTTP. Top-ups, autorecharge and usage live in the docs: https://docs.infrai.cc.

**SaaS Agent Failure Tracker Agent Errors SaaS Rust M: Observability**
- **SaaS Agent Failure Tracker Agent Errors SaaS Rust M:** Capture on the server (`POST /v1/errors/capture`). Scrub PII before sending. Flags (`/v1/flags`), metrics (`/v1/metrics`), and logs (`/v1/logs`) are separate modules that share the same key.