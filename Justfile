test:
    cargo test

deploy:
    [ -d "../michaelmdresser.github.io" ]
    cd eve_item_diff_wasm && wasm-pack build --no-typescript --target web
    cp eve_item_diff_wasm/pkg/eve_item_diff_wasm.js ../michaelmdresser.github.io/eve_item_diff/
    cp eve_item_diff_wasm/pkg/eve_item_diff_wasm_bg.wasm ../michaelmdresser.github.io/eve_item_diff/
