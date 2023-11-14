Trunk is needed to run the application, install: https://trunkrs.dev/

Run application:
trunk serve

For developement:
Watch rust file changes:
cargo watch -- trunk serve

Watch tailwind output:
npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch
