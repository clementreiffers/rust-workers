# Rust to WebAssembly Cloudflare Worker

the Goal is to build a powerful WebAssembly application from Rust, to compute loud calculations instead 
doing it with JavaScript.

## Overview

1. [Requirements](#requirements)
2. [Run it](#run-it)
   1. [Dev Mode](#dev-mode)
   2. [Production Mode](#production-mode)
3. [WebAssembly](#webassembly)
4. [Links](#links)

## Requirements

you need to install [Rust](https://www.rust-lang.org) and [NodeJS](https://nodejs.org/en).

Rust is needed to build the project, and NodeJS to get all requirements to launch the project.

To be able to launch the project in production mode, you need to install all requirements from your OS
following [this documentation](https://github.com/cloudflare/workerd).

> **Note**
> if you don't want to install them directly on your machine: 
> I created a docker on this link : ghcr.io/clementreiffers/workerd:latest

## Run it

### Dev Mode

With `wrangler`, you can build, and deploy your Worker with the following commands: 

```bash
# run your Worker in an ideal development workflow (with a local server, file watcher & more)
npx wrangler dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
npx wrangler publish
```

thanks to the `wrangler.toml`, it will build the project before using it.

Read the latest `worker` crate documentation here: https://docs.rs/worker

### Production mode  

there is a `Makefile`, so you can run `make run-worker` to have the worker running directly on your machine only using 
the [workerd runtime](https://github.com/cloudflare/workerd).


## WebAssembly

`workers-rs` (the Rust SDK for Cloudflare Workers used in this template) is meant to be executed as 
compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and
modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple. 

Read more about this on the [`workers-rs` project README](https://github.com/cloudflare/workers-rs).


## Links

- [Template used for this project](https://github.com/snoyberg/live-coding/tree/e572e7cc827ef425527bbe3e713edd4b617fe225/2021-02-05-snoypredict)
- [Most simple rust router for WASM Worker](https://github.com/cloudflare/workerd/tree/main/samples/hello-wasm)
