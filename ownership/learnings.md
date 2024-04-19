1. only 1 owner of value, when goes out of scope its dropped
2. a = b transfers ownership of value of a to b. and a cannot be accessed further. Its called moved, kind of shallow copy; use clone() to deep copy
3. Scalar types however are always deep copied.
4. Group of scalars can use Copy trait to have trivial copying.
5. As ownership is transferred to the latest assingment; functions take the ownership of the variable passed given that its complex type.
6. We can get back ownership by returning the original parameter (if multiple can use tuple).
7. functions can also give ownership for the variable that they return. 