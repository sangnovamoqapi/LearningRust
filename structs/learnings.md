1. Entire struct must be mutable, not just few fields
2. Field Init shorthand basically naming the parameters of function same as the struct fields
3. With update syntax we cannot use user1 as it will be basically assignment and ownership will be moved
4. Traits like {:?} or {:#?}  help you print formatted  structs, we can use dbg! macro, however it takes ownership, so we must pass reference
5. we must have outer attribute #[derive(Debug)] for struct to debug it or pretty format print it
6. Rust auto dereferences so we can use '.' to access attributes instead of '->' notation like in C/C++
7. This works because we are clear about mutability and ownership in the function, so rust here makes borrowing implicit for function receivers basically 'self'
8. Associated functions are used as contructors example is String::from()
