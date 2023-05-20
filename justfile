optimize:
    if [[ $(uname -m) =~ "arm64" ]]; then \
    docker run --rm -v "$(pwd)":/code \
        --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        --platform linux/arm64 \
        cosmwasm/rust-optimizer-arm64:0.12.12; else \
    docker run --rm -v "$(pwd)":/code \
        --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        --platform linux/amd64 \
        cosmwasm/rust-optimizer:0.12.12; fi

astroport-build: 
    cd ../astroport-core && ./scripts/build_release.sh

astroport-copy:
    cp ../astroport-core/artifacts/*.wasm interchaintests/wasms

interchaintest: optimize
    mkdir -p interchaintests/wasms
    if [[ $(uname -m) =~ "arm64" ]]; then cp artifacts/cw_heartbeat-aarch64.wasm interchaintests/wasms/cw_heartbeat.wasm ; else cp artifacts/cw_heartbeat.wasm interchaintests/wasms; fi
    cd interchaintests/strangelove && go test ./...