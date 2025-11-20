## start 
- Onetime install
# 1. Install wasm target (must-have)
rustup target add wasm32-unknown-unknown

# 2. Install the tool that shrinks Wasm files to tiny size
cargo install wasm-bindgen-cli

###
cargo new hello-wasm --bin
cd hello-wasm

### add crates
cargo add wasm-bindgen
cargo add console_error_panic_hook   # optional but makes errors readable in browser


## BUILD
cargo install wasm-pack   # only once ever
wasm-pack build --target web

### DEPENDENCY TO WORK WITH FILES

cargo add web-sys --features="console File FileList FileReader DataTransfer DragEvent HtmlInputElement HtmlElement Document Window"