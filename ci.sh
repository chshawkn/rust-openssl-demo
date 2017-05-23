#!/usr/bin/env bash

set -e

rustup target list | grep ios | awk '{print $1}' | xargs rustup target add

#aarch64-apple-ios armv7-apple-ios armv7s-apple-ios
RUST_IOS_ARCHS="i386-apple-ios x86_64-apple-ios"
RUST_ARCHS="${RUST_IOS_ARCHS} x86_64-apple-darwin"

LIB_NAME="openssl-1.1.0e"
LIB_VERSION="$(echo ${LIB_NAME} | awk -F- '{print $2}')"
LIB_URL_PREFIX="https://github.com/chshawkn/$(echo ${LIB_NAME} | awk -F- '{print $1}')-build/releases/download/v${LIB_VERSION}"

mkdir -p $(pwd)/tmp

RUST_ARCHS_ARRAY=(${RUST_ARCHS})
for ((i=0; i < ${#RUST_ARCHS_ARRAY[@]}; i++))
do
    ARCHIVE="${LIB_NAME}-${RUST_ARCHS_ARRAY[i]}.tar.gz"
    ARCHIVE_URL="${LIB_URL_PREFIX}/${ARCHIVE}"
    [ -f "tmp/${ARCHIVE}" ] || aria2c --file-allocation=none -c -x 10 -s 10 -m 0 --console-log-level=notice --log-level=notice --summary-interval=0 -d "$(pwd)/tmp" -o "${ARCHIVE}" "${ARCHIVE_URL}"

    rm -rf "tmp/${LIB_NAME}-${RUST_ARCHS_ARRAY[i]}"
    mkdir -p "tmp/${LIB_NAME}-${RUST_ARCHS_ARRAY[i]}"
    tar xzf "tmp/${ARCHIVE}" --strip-components=1 -C "tmp/${LIB_NAME}-${RUST_ARCHS_ARRAY[i]}"
done

#export OPENSSL_INCLUDE_DIR=""
#export OPENSSL_LIB_DIR=""
export OPENSSL_STATIC="static"
export OPENSSL_LIBS="ssl:crypto"

# If openssl is built with zlib, it will fail to link statically
# https://github.com/sfackler/rust-openssl/issues/590
# zlib is only used for TLS compression which you should not be using
# That can be turned off by passing no-comp to Configure

# Symbol rust_crypto_util_fixed_time_eq_asm missing for aarch64 builds
# https://github.com/DaGenix/rust-crypto/issues/383
# https://github.com/DaGenix/rust-crypto/pull/384

ORIGINAL_PATH="${PATH}"

cargo clean
RUST_ARCHS_ARRAY=(${RUST_IOS_ARCHS})
for ((i=0; i < ${#RUST_ARCHS_ARRAY[@]}; i++))
do
    RUST_ARCH="${RUST_ARCHS_ARRAY[i]}"
    echo ">>>>>>>>>> ---------- build with ${LIB_NAME}-${RUST_ARCH} ---------- >>>>>>>>>>"
    export OPENSSL_DIR="$(pwd)/tmp/${LIB_NAME}-${RUST_ARCH}"
    export OPENSSL_INCLUDE_DIR="${OPENSSL_DIR}/include"
    export OPENSSL_LIB_DIR="${OPENSSL_DIR}/lib"
    echo "OPENSSL_DIR: ${OPENSSL_DIR}"

    (cd bin-depends-on-rlib; cargo build --target=${RUST_ARCH} --verbose;)
done

##cargo install cargo-lipo
#export OPENSSL_DIR="$(pwd)/tmp/${LIB_NAME}-universal-apple-ios"
#cargo lipo
