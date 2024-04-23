1. If immutable reference of first element is used, we cannot mutate the vector
2. To store different types in vector like linear data structure, we can use nums
3. In Addition of two strings, the first strings ownership is taken by the add function and reference of second is used returning the ownership of the result. Also compiler coerces &String into &str by adding &s[..] as string slice
4. format! macro doesn't take ownership and is a good substitute of adding strings
5. Strings are a wrapper over Vec<u8> internally.
6. There is no indexing in strings, to avoid utf-8 characters requiring more than 1 byte to store and return unrelated data
7. String slicing is one of the way to index over utf-8 characters where we know what a character would be consisting of, but code will panic if tried to slice between a character.
8. chars() gives us basic chars a string is consisted of and bytes(): the actual bytes
9. grapheme clusters are not handled in rusts' standard librarys, where two characters combine to become one like diacritics and a consonent.10. copied() is used on a method to get Option<T> instead of Option<&T>, and unwarp_or(0) sets 0 to the new owner if copied() return None instead of the value
11. Hashmaps become the new owner of the fields inserted
12. Inserting in hasmaps overwrites last value if the key already exists
13. Use entry to insert only if it doesn't exist combined with or_insert(val)
14. or_insert(val) returns mutable reference to the value
