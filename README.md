# brainfuck-rs

A [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter.

```sh
A Brainfuck interpreter

Usage: brainfuck [OPTIONS] <COMMAND> [PROGRAM]

Arguments:
  <COMMAND>
          Possible values:
          - tokenize: Transform the program into [`Token`]s
          - ir:       Transform the program into [`Instruction`]s
          - run:      Parse the program and run it

  [PROGRAM]


Options:
  -f, --file <FILE>


  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
