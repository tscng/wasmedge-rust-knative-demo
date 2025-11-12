WasmEdge Rust KNative Demonstration
===
This repository implements a demonstration of a WASM KNative service, written in Rust.  
CI/CD and basic setup is builds on [tscng/wasmedge-rust-plugin-demo](https://github.com/tscng/wasmedge-rust-plugin-demo).

## Webserver
Using [patches for WasmEdge](https://wasmedge.org/docs/develop/rust/http_service/server), the axum server framework can be used.  
Code is adapted from the sample repo at [WasmEdge/wasmedge_hyper_demo](https://github.com/WasmEdge/wasmedge_hyper_demo).  
The webserver exposes a GET endpoint for [KNative readiness probes](https://knative.dev/docs/serving/services/configure-probing/).

## KNative Wasm Setup
For a WASM-ready development setup of Kubernetes + KNative, Microk8s and KWasm can be used.  
Instructions are mostly from [this walkthrough](https://github.com/keniack/awesome-raspberrypi-tutorials?tab=readme-ov-file).
- Install microk8s [(step 1+2)](https://microk8s.io/docs/getting-started)
- Set up KNative
```
microk8s enable community
microk8s enable knative
```
- Set up WASM runtime via KWasm
```
microk8s enable kwasm

cat <<EOF | microk8s kubectl apply -f -
apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: wasmedge
handler: wasmedge
EOF

kubectl delete ValidatingWebhookConfiguration validation.webhook.serving.knative.dev
```

## KNative Deployment
Install the service from the `service.yml` service manifest:
```
microk8s kubectl apply -f https://github.com/tscng/wasmedge-rust-knative-demo/releases/latest/download/service.yml
```
Verify readiness:
```
microk8s kubectl get kservice
```
Get the cluster ingress IP and service URL:
```
microk8s kubectl get service.serving.knative.dev
microk8s kubectl get service kourier-internal -n knative-serving
```
Test the HTTP server without DNS (exemplary IP and hostname):
```
curl -H "Host: rust-wasm-service.default.svc.cluster.local" http://10.152.183.112 
curl -H "Host: rust-wasm-service.default.svc.cluster.local" http://10.152.183.112/service -d "Hello"
```
