# Rust Hangman

## TODO

- [x] add and work with crates  `console`, `dialoguer` for the CLI
- [x] save/read locally username & scores (+ timestamp) in JSON files: `walkdir`, `serde` & `serde_json`
    - [x] sort scores
    - [x] display best scores
    - [ ] refactor score/player/game (now overkill)
- [x] modularize (`mod utils`)
- [ ] handle error well
- [x] create types for the game (+)
- [ ] choose language (create a config if needed)
- [x] hint with ollama (+++) as an option
- [x] do unit test (++)
- [ ] comment code only as needed (try rust doc)
- [ ] replace print by write term