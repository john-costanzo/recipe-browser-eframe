doit: main # wasm 

main: main.rs recipe_browser.rs
	cargo build && ../target/debug/recipe_browser

wasm:
	cargo install --locked trunk && \
	(cd ..; trunk serve&) # &&  \
#	sleep 10 && \
#	/usr/bin/chromium --new-window 'http://127.0.0.1:8080/index.html#dev'
