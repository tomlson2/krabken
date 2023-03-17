

[![Build Status](https://travis-ci.org/krabken/krabken.svg?branch=master)](https://travis-ci.org/krabken/krabken)
[![Krabken](https://img.shields.io/crates/d/krabken.svg)](https://crates.io/crates/krabken)
[![Krabken](https://img.shields.io/crates/dv/krabken.svg)](https://crates.io/crates/krabken)
[![Crates.io](https://img.shields.io/crates/v/krabken.svg)](https://crates.io/crates/krabken)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
<pre>
 ╔╗ ╔═╗ ║║ ╔═╗  ╔╗ ╔╦═  ║║ ╔═╗  ╔╦╗ ╔╦╗ ╔═╗ ║║
 ║║ ║ ║ ║║ ║═╬╗ ║║ ╠╩╗  ║║ ║╣   ║║║ ║║║ ╠╣═╣ ║║
 ║║ ║ ║ ║║ ╚═╝║ ║║ ╩═╝  ║║ ╚═╝  ╩ ╩═╩ ╩ ║║═╣ ║║
 ║║ ╠═╝ o║╚═╗   ╝╚═╗    ╔╝ ║╣   ╔╗╔  ╔═╗ ║║═╣ ╔╝
o╝╚╝o╚══ ╚══╝     ╚═╝    ╚══╝   ╚╝╚  ╚═╝o╚══╝o╚═╝ I am working on it.
</pre>

```rs
 ___  ______ ______ ______ ______ ______ ______ ______ ______ ______ ___ 
|___||______||______||______||______||______||______||______||______||___|
|___|/______|/______|/______|/______|/______|/______|/______|/______|/___|
```

# krabken
Rust &lt;> Kraken 

### About
This is a work in progress but hopefully it can grow into an easy to use Rust program that can interact with the Kraken exchange. This is only my second attempt at building something substantial in Rust so it won't be perfect but there is a lot of learning about new libraries for creating API calls, parsing that data, and then working with that data to get what I want.

### Links
- [Kraken API](https://www.kraken.com/help/api)
- [Kraken API docs](https://www.kraken.com/help/api#general-usage)

### Functionality
- [x] Handle authentication to Kraken using encrypted API-sign
- [x] Build methods for client connection to get and post to API
- [x] Create structure for querying and storing market data.
- [ ] Trading functionality
- [ ] Build out technical analysis and data tools
- [ ] Manage requests and keep track of available requests before throttle
- [ ] Paper trading
