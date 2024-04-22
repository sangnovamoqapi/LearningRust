1. There are two types of creates binary and library. Libraries are packages that are used as api to other binaries or libraries.
2. A binary crate can have only 1 library crate; there is no restriction for libraries
3. Everything is private by default, only variants in enum are public by default
4. Nested paths-> 'use std::{io, cmp::Ordering};'
