#!/bin/sh

protoc \
    engine/src/state.proto \
    --ts_out="src/generated" \
    --ts_opt=no_namespace \
    -I=..