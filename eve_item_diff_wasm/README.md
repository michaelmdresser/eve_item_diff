# WASM eve_item_diff

WASM-ready version of eve_item_diff for browser usage.

## Build
``` sh
wasm-pack build --no-typescript --target web
```

## Use direcly via JS

Place the `.js` and `.wasm` files in a directory (e.g. `eve_item_diff`) on the web
server. Then use a snippet like this to load and use:

```html
<script type="module">
import init, { diff } from './eve_item_diff/eve_item_diff_wasm.js';
async function run() {
  await init();
  let {left_items, right_items, left_missing, right_missing} = diff("Paladin x5", "Paladin x3");
  console.log("items:", items);
}
run();
</script>
```

Reference for the above: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html 
