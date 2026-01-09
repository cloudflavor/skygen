#### Project Skygen

Skygen is a Rust-based OpenAPI v3 code generator that turns provider specs
(Cloudflare, DigitalOcean, Exoscale, Hetzner, Scaleway) into ready-to-publish
SDK crates.  

The skygen CLI ingests a spec plus a small TOML config and bootstraps a full
crate: Cargo.toml, module tree, reusable client/runtime, and provider-specific
scaffolding. Templates leverage embedded assets and Tera, while a resolver
module walks the OpenAPI document to expand $refs with cycle detection.  

Generated SDKs emphasize a minimal footprint via feature-gated dependencies
(reqwest transport optional, TLS protocols selectable) and expose pluggable
transports so users can swap in custom HTTP stacks.  

```
skygen generate -c <config.toml> -i <spec.yaml|spec.json> -o <output-dir>
```
