This is a Chess game that can only be played locally on one machine.
The game was built using a rust web framework, Yew. Normal chess rules
apply besides some special rules like castling, en passant, and automatic
king checking.

HOW TO BEGIN PLAYING:

WASM is needed:
rustup target add wasm32-unknown-unknown

Trunk is needed to run the application, install:
cargo install --locked trunk

Finally run application:
trunk serve --open

DEVELOPEMENT (not needed to play):

Watch rust file changes:
cargo watch -- trunk serve

Watch tailwind output:
npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch
