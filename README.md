
# krabken
rust &lt;> kraken

### About
This is a work in progress but hopefully it can grow into an easy to use rust program that can interact with the Kraken exchange. This is only my second attempt at building something substantial in Rust so it won't be perfect but there is a lot of learning about new libraries for creating API calls, parsing that data, and then working with that data to get what I want.

### Functionality
- [x] Handle authentication to Kraken using encrypted API-sign
- [x] Build methods for client connection to get and post to API
- [x] Create structure for querying and storing market data.
- [ ] Trading functionality
- [ ] Build out technical analysis and data tools
- [ ] Manage requests and keep track of available requests before throttle
- [ ] Paper trading

### Usage
To get started, clone the repository and create a `config.json` file with your Kraken API credentials.

```
{
    "key": "YOUR_API_KEY",
    "secret": "YOUR_API_SECRET"
}
```

Then, build the project with `cargo build` and run `cargo run` to execute the program.