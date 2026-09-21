# Quiz

A small quiz website powered by [quizzapi.fr](https://quizzapi.fr), built to try out the [Topcoat](https://github.com/tokio-rs/topcoat) framework.

## Stack

- [Topcoat](https://github.com/tokio-rs/topcoat) v0.9.0
- Tailwind CSS 4 with shadcn-style components, managed by `topcoat ui`
- [reqwest](https://crates.io/crates/reqwest) for the quizzapi.fr HTTP calls
- [serde](https://crates.io/crates/serde) for the API payloads
- [rand](https://crates.io/crates/rand) to shuffle the correct answer among the wrong ones
- [tokio](https://crates.io/crates/tokio) as the async runtime

## Todo

- [ ] Home page settings: difficulty and number of questions
- [ ] Show which questions were answered correctly on the result screen
