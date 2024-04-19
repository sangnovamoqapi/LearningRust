1. References are like addresses allows function to read value owned by other variable
2. When reference go out of scope they don't drop the original owner as they don't own the owner but are a pointer.
3. This action of creating a reference and passing is called borrowing as the function kind of borrows the ownership but returns the caller.
4. References are also immutable; however we can pass mutable reference : owning_function(&mut a); however we can only have 1 reference to a mutable value.
5. This restriction of only 1 mutable reference is to avoid data racing condition
which occurs when 2 or more pointers try to read and at least 1 try to write at the same location without any mechanism used to prevent synchronize access.
6. We can still create multiple mutable references only in different scope and not simultaneously; or after using and never accessing again.
7. We can have any number of immutable reference but they cannot be accessed after a mutable reference.
8. Until the last immutable reference access is used, we cannot declare mutable references for the same value as the person declaring it immutable would expect it to be immutable until they read, if not it will throw error for the latest mutable reference declared before accessing of immutable reference making data mutability controlled. Awesome!!
9. Compiler throws dangling reference error when function returns a reference which it created itself, making it out of scope after return; however we can return the whole variable of course as it will move out ownership.
10. At any given time, you can have either one mutable reference or any number of immutable references.