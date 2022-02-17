# brainfrsk

A Rust brainf\*ck interpreter.

This is my first program in the Rust language, and I found it very
interesting ("fighting the borrow checker" for the first time etc).

It seems to work with simple programs, for example, it can run the
current world record 'Hello, World!' program.

However, there are definitely a few small isues, such as it doesn't
seem to output (flush stdout?) until the program is over, so
infinitely running programs like cat don't print anything.

As my first Rust code, please excuse the code quality or any 
bodges required to get the program running, and if you have any
tips (eg. I don't think [this](https://github.com/adamsoutar/brainfrsk/blob/master/src/vm.rs#L35) is the idiomatic way to do the 
iterator in `bracket_match`), they'd be greatly appreciated.
