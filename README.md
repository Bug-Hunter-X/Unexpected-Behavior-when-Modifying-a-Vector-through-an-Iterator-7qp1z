# Rust Iterator Mutation Bug

This repository demonstrates a subtle bug that can occur when modifying a vector using a mutable iterator in Rust.  The code attempts to modify the first element of a vector via an iterator but prints the original vector values after the modification. The solution illustrates the proper way to handle this kind of modification using iter_mut().