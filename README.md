# Rust Hangman

## TODO

- [x] add and work with crates  `console`, `dialoguer` for the CLI
- [x] save/read locally username & scores (+ timestamp) in JSON files: `walkdir`, `serde` & `serde_json`
    - [ ] sort scores
    - [ ] display best scores
    - [ ] refactor score/player/game (now overkill)
- [x] modularize (`mod utils`)
- [ ] handle error well
- [x] create types for the game (+)
- [ ] choose language (create a config if needed)
- [ ] hint with ollama (+++) as an option
- [x] do unit test (++)
- [ ] comment code only as needed (try rust doc)