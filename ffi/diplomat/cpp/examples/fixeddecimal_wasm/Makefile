# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
.PHONY: build test clean serve build-host test-host

ALL_HEADERS := $(wildcard ../../include/*.hpp) $(wildcard ../../../c/include/*.h)
ALL_RUST := $(wildcard ../../../src/*.rs)

CXX?=g++
EMCC?=emcc

$(ALL_RUST):

$(ALL_HEADERS):

../../../../../target/debug/libicu_capi_staticlib_test.a: $(ALL_RUST)
	cargo build -p icu_capi_staticlib -p icu_capi_staticlib --features provider_test
	mv ../../../../../target/debug/libicu_capi_staticlib.a ../../../../../target/debug/libicu_capi_staticlib_test.a

a.out: ../../../../../target/debug/libicu_capi_staticlib_test.a $(ALL_HEADERS) test.cpp
	$(CXX) -std=c++17 test.cpp ../../../../../target/debug/libicu_capi_staticlib_test.a -ldl -lpthread -lm -g

../../../../../target/wasm32-unknown-emscripten/release-opt-size/libicu_capi_staticlib_test.a: $(ALL_RUST)
	RUSTFLAGS="-Cpanic=abort" cargo +nightly-2022-04-05 build --profile=release-opt-size -p icu_capi_staticlib --features provider_test --target wasm32-unknown-emscripten -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
	mv ../../../../../target/wasm32-unknown-emscripten/release-opt-size/libicu_capi_staticlib.a ../../../../../target/wasm32-unknown-emscripten/release-opt-size/libicu_capi_staticlib_test.a

web-version.html: ../../../../../target/wasm32-unknown-emscripten/release-opt-size/libicu_capi_staticlib_test.a $(ALL_HEADERS) test.cpp
	$(EMCC) -std=c++17 test.cpp ../../../../../target/wasm32-unknown-emscripten/release-opt-size/libicu_capi_staticlib_test.a -ldl -lpthread -lm -g  -o web-version.html --bind --emrun -sENVIRONMENT=web -sWASM=1 -sEXPORT_ES6=1 -sMODULARIZE=1

node-version.js: ../../../../../target/wasm32-unknown-emscripten/release-opt-size/libicu_capi_staticlib_test.a $(ALL_HEADERS) test.cpp
	$(EMCC) -std=c++17 test.cpp ../../../../../target/wasm32-unknown-emscripten/release-opt-size/libicu_capi_staticlib_test.a -ldl -lpthread -lm -g  -o node-version.js --bind -sWASM=1 -sENVIRONMENT=node -sWASM_ASYNC_COMPILATION=0 -DNOMAIN

build: web-version.html node-version.js

test: node-version.js
	exec node ./node-test.js

serve: web-version.html
	emrun web-version.html

# These make it possible to ensure that the C++ code is up to date with the bindings
# without needing to set up emsdk. This way `make test-ffi` works without emsdk.
build-host: a.out

test-host: build-host
	./a.out

clean:
	rm -f web-version.html
	rm -f web-version.wasm
	rm -f web-version.js
	rm -f node-version.js
	rm -f node-version.wasm
	rm -f ../../../../../target/wasm32-unknown-emscripten/release-opt-size/libicu_capi_staticlib_test.a
	rm -f ../../../../../target/debug/libicu_capi_staticlib_test.a
	rm -f a.out
