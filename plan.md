The plan is to keep the tape ptr in a register at r8

We'll use it to load offsets from the stack base pointer
into rax to do maths

Loops will be done using jump labels

We'll store the furthest we've ever got in r9
If you go further than r9, we will zero the memory where you are
This avoids the issue of running into garbage memory above the stack.

Scratch that, we don't need r9, we can use the stack ptr.
All of this gives us *near unlimited* memory tape, in the 0+ 
direction. It will, however, be limited by stack space
(8mb on macOS - that's 1 million cells)