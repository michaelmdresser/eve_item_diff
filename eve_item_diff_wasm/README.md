# WASM eve_item_diff

WASM-ready version of eve_item_diff for browser usage.

## Build

See root `Justfile`

## Use directly via JS

Place the `.js` and `.wasm` files in a directory (e.g. `eve_item_diff`) on the web
server. Then use a snippet like this to load and use:

```html
<script type="module">
import init, { diff } from './eve_item_diff/eve_item_diff_wasm.js';
async function run() {
  await init();
  let {left_items, right_items, left_missing, right_missing, left_missing_formatted, right_missing_formatted} = diff("Paladin x5", "Paladin x3");
  console.log("left items:", left_items);
  console.log("right items:", right_items);
}
run();
</script>
```

Reference for the above: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html 
