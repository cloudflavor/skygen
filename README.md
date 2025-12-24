#### Project Skygen

Project Skygen is an effort to generate ergonomic and performant Rust SDKs for Cloud Service Providers using their OpenAPI v3 spec. It currently supports Cloudflare, Exoscale, Hetzner, DigitalOcean, and Scaleway.

Skygen includes provider-specific logic sufficient to generate correct and usable Rust SDKs, for the supported CSPs it does not aim to be a OpenAPIv3 to Rust SDK generator.

```
skygen generate -c <config.toml> -i <spec.yaml|spec.json> -o <output-dir>
```
