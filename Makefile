MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

build:
	cargo build

run-local:
	cargo run

run-grpcui:
	grpcui -plaintext -port 4000 localhost:3000

gen-client:
	mkdir -p web/src/rpc
	rm -f web/src/rpc/*
	protoc --proto_path=proto/. \
           --plugin="protoc-gen-ts=./web/node_modules/.bin/protoc-gen-ts" \
           --js_out=import_style=commonjs,binary:web/src/rpc \
           --ts_out=service=grpc-web:web/src/rpc \
           proto/*.proto
	find web/src/rpc -type f -name "*_pb.js" | xargs gsed -i -e "1i /* eslint-disable */"
	find web/src/rpc -type f -name "*_pb_service.js" | xargs gsed -i -e "1i /* eslint-disable */"

run-web:
	cd web && npm start