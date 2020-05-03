# stir

`stir` aims to provide an easy to use Intermediate Representation. The ultimate
goal of `stir` is to parallelize the generated IR code, while applying some
other optimization passes.

`stir` is built around the idea of building blocks. Some blocks have reference
to other blocks, that they can call and execute. Each block implements the
following functionalities:

* Debugging
* Pretty-printing
* Interpretation
* Translation to LLVM
* Optimisation

To put blocks together, use a Recipe. To run the Recipe, just `fry` it ! A
recipe needs an entry block (or main block) to start interpreting from.

Use the [`fry`](https://github.com/cohenarthur/fry) binary to intepret STIR code.
`fry` can interpret code pretty-printed from the `stir` crate, or code directly
written in .stir file ! For a rundown of the syntax, check [SYNTAX.md](SYNTAX.md)

To use STIR as a representation for your language, simply add a translation unit
from your AST to STIR building blocks.

## Features

* [x] Interpretation
* [ ] JIT Interpretation!
* [ ] Translation to LLVM
* [ ] IR multithreading

## Available building blocks

* [x] Boolean
* [x] IfElse
* [x] Loop
