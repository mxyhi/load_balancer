#!/bin/bash

if [ -z "$THREADS_NUM" ]; then
    THREADS_NUM=1
fi

if [ -z "$RUST_LOG" ]; then
    RUST_LOG="INFO"
fi

echo $'---
version: 1
threads: '$THREADS_NUM'
' >./conf.yaml

# 运行 Rust 应用程序
load_balancer -c ./conf.yaml
