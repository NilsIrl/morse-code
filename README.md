# morse-code
GCI Task

# Usage

```bash
$ git clone git@github.com:NilsIrl/morse-code.git
$ cd morse-code
$ cargo run -- encode <<< "SOS"
... ___ ...
$ cargo run -- encode <<< "SOS" | cargo run -- decode
SOS
$ cargo run -- encode --dash \* --dot \( <<< "morse code is obselete"
** *** (*( ((( ( / *(*( *** *(( ( / (( ((( / *** *((( ((( ( (*(( ( * (
$ cargo run -- decode --dash \* --dot \( <<< '** *** (*( ((( ( / *(*( *** *(( ( / (( ((( / *** *((( ((( ( (*(( ( * ('
MORSE CODE IS OBSELETE
```

Use the `--help` argument to get more information.

The `encode` subcommand encodes and the `decode` subcommand, you guessed it,
decodes.

Input is given from
[stdin](https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)).
