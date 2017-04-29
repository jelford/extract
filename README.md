# extract
[![Build Status](https://travis-ci.org/jelford/extract.svg?branch=master)](https://travis-ci.org/jelford/extract) 
[![crates.io](https://img.shields.io/crates/v/extract.svg)](https://crates.io/crates/extract)

Extract text from text using a regex - a simple way to consume keyed fields from poorly-(or un-)structured text.

# Usage

Extract accepts as an argument a regex with a single capture group, and will read lines from stdin, printing captured values.

    > echo "hello subject=world" | extract "subject=(.+)"
    world

`extract` keeps reading until it reaches the end of input, processing lines one-at-a-time:

    > cat multiline
    Hello subject=world
    Hello subject=Dorris
    > cat multiline | extract "subject=(\w+)"
    world
    Dorris


# Installation

From source:

    cargo install

From `crates.io`:

    cargo install extract

# License

MIT / Apache 2.

# Issues / Contributing

Feel free to open an issue / PR. I'd be interested in adding support for more structured outputs (e.g. JSON from a regex with named capture groups).

# FAQs

* How is this different to `grep -o`?

`extract` only prints the captured group, not the whole match.

* Can't you use `sed` for this?

Not first time, normally. Maybe your sed-fu is better than mine. This tool was originally created when I needed to extract ids from the output of `xinput --list`, in frustration after the fourth attempt to deliver the correct incantations to `sed`.
