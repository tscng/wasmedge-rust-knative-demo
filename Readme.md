WasmEdge Rust Plugin Demonstration
===
This repository implements a demonstration of a WASM KNative service, written in Rust.  
CI/CD and basic setup is adapted from [tscng/wasmedge-rust-plugin-demo](https://github.com/tscng/wasmedge-rust-plugin-demo).

## Webserver
Using [patches for WasmEdge](https://wasmedge.org/docs/develop/rust/http_service/server), the axum server framework can be used.  
Code is adapted from the sample repo at [WasmEdge/wasmedge_hyper_demo](https://github.com/WasmEdge/wasmedge_hyper_demo).  
The webserver exposes a GET endpoint for [KNative readiness probes](https://knative.dev/docs/serving/services/configure-probing/).

## KNative Wasm Setup


## KNative Deployment