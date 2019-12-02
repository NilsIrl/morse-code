# morse-code
GCI Task

# Usage

```bash
$ git clone git@github.com:NilsIrl/morse-code.git
$ cargo run -- encode <<< "SOS"
... ___ ...
$ cargo run -- encode <<< "SOS" | cargo run -- decode
SOS
```

The `encode` subcommand encodes and the `decode` subcommand, you guessed it,
decodes.

Input is given from
[stdin](https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)).
