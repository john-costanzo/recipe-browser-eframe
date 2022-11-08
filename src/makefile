doit: main

main: main.rs recipe_browser.rs
	cargo build && ../target/debug/recipe_browser

wasm:
	cargo install --locked trunk && (cd ..; trunk serve)
