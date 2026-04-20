# Discussion Question Response

Team members: Benny Rakower, Tracy Dang, Megha Thumma

Every time collatz_length calls itself, a new stack frame gets pushed onto the call stack storing the current n, the accumulator, and where to return to. If this happens enough times, you get a stack overflow, which is exactly what happened when we first ran longest_collatz(1_000_000).

A loop would not have this problem since it just reuses the same stack frame the whole time instead of stacking new ones.

Rust does not guarantee tail call optimization (TCO), so even though our helper functions are written tail recursively, the compiler does not automatically convert them into loops. Erlang and Haskell guarantee this but Rust does not. That is why we had to spawn the helper on a larger stack thread to avoid the crash while still following the no loops requirement.