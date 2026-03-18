# silicon-prion

rust is supposed to be memory-safe. i wanted to see what alzheimer's looks like in silicon. 

this script allocates a massive contiguous block of heap memory, strips away all of rust's safety guarantees using `unsafe` raw pointers, and spawns multiple threads to intentionally introduce bit-flips. the corruption spreads based on a localized infection algorithm. 

watching the heap slowly rot and cascade into a segfault is depressing. i drank two pints of vodka and wrote this instead of doing actual client work. 

**build:**
`cargo build --release`
run it until your os kills the process.
