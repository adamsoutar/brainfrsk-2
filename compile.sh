# Run brainfrsk
RUST_BACKTRACE=1 cargo run input.bf > output.s && \

# Assemble x86_64 assembly into a binary
# (It is _not_ an assembler or linker)
gcc output.s -o output && \

# Run output
./output

# Print last cell value
# echo $?
