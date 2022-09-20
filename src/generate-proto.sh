#!/bin/sh

protoc \
    src-tauri/src/state.proto \
    --ts_out="src/generated" \
    --ts_opt=no_namespace \
    -I=..