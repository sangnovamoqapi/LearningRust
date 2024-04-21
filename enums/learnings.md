1. Enums allow to store data as well, which when initialized is a call to constructor returning the enum type, this is useful when enum variants have different types instead of using struct with type and data field
2. There are no nulls in Rust, instead Option<T> {Some:T, None} enum
3. Now if x is 5 and y is Option<i8> = Some(5), x +y wont't compile because of type mismatch this would be a problem in other languages where we expect some values to be null and unintentionally use it in some expression which can give run time error, but not here
