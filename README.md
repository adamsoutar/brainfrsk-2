# brainfrsk 2

An optimising Brainf\*ck to x86-64 assembly compiler.

This compiler can produce optimised binaries for macOS (Sytem V
calling convention) by outputting AMD64 asm (similar to [my other project](https://github.com/adamsoutar/ass)).

## Speed

I've written several Brainf\*ck projects over the years.

Here's a comparison of the execution time for a "Hello world!" program in each:

| Project                                                | Implementation   | Time   | Ratio             |
| ------------------------------------------------------ | ---------------- | ------ | ----------------- |
| [brainfJSk](https://github.com/adamsoutar/brainfJSk)   | JS Interpreter   | 0.073s | 1x                |
| [brainGoop](https://github.com/adamsoutar/brainGoop)   | Go Interpreter   | 0.009s | 8x                |
| [brainfrsk 1](https://github.com/adamsoutar/brainfrsk) | Rust Interpreter | 0.008s | 9x                |
| brainfrsk 2                                            | Rust Compiler    | 0.002s | **36x faster** 🤯 |

## Try it yourself

```
git clone https://github.com/adamsoutar/brainfrsk-2
cd brainfrsk-2
./compile.sh
```

This (assuming you're on macOS) will produce and run an output binary at `./output`.

## Optimisations

More than just writing it in pure assembly code, the compiler
makes your code faster by batching repeated operations into one.

For example, `+++++` is re-written as `add 5`.

## Tape size & limits

The tape is automatically expanded (it's "unlimited"). Cells are 64-bit
integers.

Memory is automatically allocated on the stack. This _technically_
means the tape is limited by the stack size. On macOS that is
`8mb`, a.k.a 1 _million_ cells.

---

<h6 align="center">by Adam Soutar</h6>
