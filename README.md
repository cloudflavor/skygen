#### Project Skygen

Project Skygen aims to create a unified tool that can parse and create Rust SDKs for CSPs that use the OpenAPIv3 Spec for their API.  
For the time being it can generate bindings for Cloudflare, Exoscale, Hetzner, DigitalOcean and Scaleway.  
This tool does not aim to support any OpenAPIv3 spec, but rather contain enough specific logic that it can generate
Rust SDKs for the CSPs mentioned above.


```
skygen generate -c <config.toml> -i <spec.yaml|url> -o <output-dir> 
```


Needs taplo-cli installed to format the resulting Cargo.toml file.  
