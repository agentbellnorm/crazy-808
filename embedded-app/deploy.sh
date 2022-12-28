#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

pwd

readonly PACKAGE_NAME=embedded-app
readonly TARGET_HOST=pi@mediapi
readonly TARGET_PATH=/home/pi/${PACKAGE_NAME}
readonly TARGET_ARCH=armv7-unknown-linux-musleabihf
readonly SOURCE_PATH=../target/${TARGET_ARCH}/release/${PACKAGE_NAME}

cargo build --release --package ${PACKAGE_NAME}
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}