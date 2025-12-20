#### Skygen

Skygen generates Rust clients from OpenApi v3.X specs.  
The reason for this is  because some of these Specs don't play nice with tools such as Progenitor that would allow us to generate Rust SDK Clients without having to roll out our own.  
It currently can generate Rust clients for Cloudflare, DigitalOcean, Exoscale and  Hetzner.
CAVEAT EMPTOR:  This app is a means to an end and the code has been generated with ChatGPT 5.2 is subject to  change and has not been tested. For openapi specs that are not part of the scope of this app, results may vary.
 

```
skygen generate -i <spec.yaml|url> -o <output-dir> 
```
