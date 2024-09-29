## `Pingora` load balancer

```sh
# env:
# http1 address; it is must be set
# export H1_ADDR="0.0.0.0:8188"
# api addresses; it is must be set
# export UPSTREAMS="127.0.0.1:3000,127.0.0.1:3001"
# optional threads number default 1
# export THREADS_NUM=1
# optional default INFO level
# export RUST_LOG=INFO
# optional custom host header
# export HEADER_HOST=""
# optional custom SNI domain
# export SNI_DOMAIN=""
# optional http2 address
# export H2_ADDR=""
# optional http2 cert path
# export H2_CERT_PATH=""
# optional http2 key path
# export H2_KEY_PATH=""
```

## docker

```sh
docker pull mxyhi/load-balancer
```
