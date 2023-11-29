Gimmicks with project currently:

- You can only move pieces by clicking the piece and then clicking a second time where you would like to move it
- Sometimes you need to click to reset your first click, if it seems like you cannot move anything
- Not all movements are checked to be valid yet
- You can take the king

Trunk is needed to run the application, install: https://trunkrs.dev/

Run application:
trunk serve --open

For developement:
Watch rust file changes:
cargo watch -- trunk serve

Watch tailwind output:
npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch
