
# krabken
rust &lt;> kraken
### About
This is a work in progress but hopefully it can grow into an easy to use rust program that can interact with the Kraken exchange. This is only my second attempt at building something substantial in Rust so it won't be perfect but there is a lot of learning about new libraries for creating API calls, parsing that data, and then working with that data to get what I want.

### Why Rust?
I've been working with python a lot and I love it but I wanted to try something new and I've been hearing a lot of great things about Rust. I've been experimenting with it for a while and I thought this would be a good way to build something with it. I also wanted to try to build a library that could be used by other people.

### Dependencies
* [reqwest](https://github.com/seanmonstar/reqwest)
* [serde](https://github.com/serde-rs/serde)
* [serde_json](https://github.com/serde-rs/json)
* [serde_derive](https://github.com/serde-rs/serde_derive)
* [base64](https://github.com/silentbicycle/rust-base64)
* [sha1](https://github.com/m4b/sha1-rs)
* [chrono](https://github.com/chronotope/chrono)

### Functionality
- [x] Handle authentication to Kraken using encrypted API-sign
- [x] Build methods for client connection to get and post to API
- [x] Create structure for querying and storing market data.
- [ ] Trading functionality
- [ ] Build out technical analysis and data tools
- [ ] Manage requests and keep track of available requests before throttle
- [ ] Paper trading
