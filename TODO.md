# Pending Work

- Improve schema coverage further: support enums, discriminated unions (`oneOf`/`allOf`), and emit strongly typed params instead of plain `String` where OpenAPI specifies numbers/booleans.
- Generate authentication helpers tailored to each provider (e.g., API keys, email headers) so consumers don’t have to manually set these on every builder.
- Add higher-level client ergonomics such as pagination helpers, automatic retries/backoff, and a richer `ApiError` that surfaces HTTP status codes and response bodies.
- Allow template-driven customization of default headers/query params per endpoint (not only globally via `ApiClient`) for cases like Cloudflare’s account-level headers.
- Consider emitting per-module documentation (README/examples) so users can discover how to compose the builders, and wire CI/tests that validate a sample request using mocked servers.
