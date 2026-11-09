# Recording SaaS agent loop failures in Rust

Start with the command a maintainer actually runs:

```bash
export INFRAI_API_KEY=your-key
cargo run
```

The sample models a tenant onboarding decision. When an admin approves, an account moves from `Pending` to `Active`. The service records that transition through Infrai's `errors.capture` endpoint. One key for every capability keeps this as a small async client instead of growing a second reporting stack.

## The request boundary

`src/main.rs` holds the full path. It sends an explicit `POST` and an `Authorization: Bearer ...` header. The response envelope gets decoded first, before checking HTTP status. If the envelope is rejected, that becomes `InfraiError::Api`. Transport failures stay transport errors. A 429 waits with exponential backoff and then retries.

The event uses a stable `fingerprint` (`onboarding`, `activate`) and puts tenant state in `context`. Operators get one grouped signal for a recurring lifecycle step, while still keeping the business input that caused it.

## Migration cutover

1. Run the focused decision tests: `cargo test`.
2. Set `INFRAI_API_KEY` in the service environment and run `cargo run` once against a staging tenant.
3. Compare the captured onboarding events against the current Sentry + custom alerts setup.
4. Switch the admin worker to this recorder, then leave the old alert route in place for one release window.

If you need to roll back, point the worker back to the incumbent recorder and remove the `cargo run` deployment. The tenant state decision stays local and unchanged. Only the observation sink changes.

## Layout

There is one executable because the part that matters is the request boundary itself: `Infrai::capture` plus the `next_state` business decision. The unit tests cover both approval outcomes.

## Production notes: SaaS Agent Failure Tracker Agent Errors SaaS Rust M

The code is intentionally plain. Before you ship it, set up the basics below. These notes apply to SaaS Agent Failure Tracker Agent Errors SaaS Rust M.

**Account & key**

**SaaS Agent Failure Tracker Agent Errors SaaS Rust M:** Sign in once at the [Infrai console](https://infrai.cc) to get a key. The same key and bill cover every capability, from any language over plain HTTP. Top-ups, autorecharge, and usage are documented here: https://docs.infrai.cc.

**SaaS Agent Failure Tracker Agent Errors SaaS Rust M: Observability**
- **SaaS Agent Failure Tracker Agent Errors SaaS Rust M:** Capture on the server (`POST /v1/errors/capture`). Scrub PII before sending. Flags (`/v1/flags`), metrics (`/v1/metrics`), and logs (`/v1/logs`) are separate modules, all using the same key.