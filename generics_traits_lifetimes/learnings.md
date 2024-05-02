1. we can constrain the generic types in rust by using traits to ensure comparison or mutation of the generic type is certainly not corrupting
2. There is overhead cost for generics as compiler does reverse lookup and generates code for concrete types from where the function is called. But what about call from libraries?
3. Traits are used to bound generic type to have certain behaviour, they are like interfaces but with some difference.
4. We can implement standard library trait to a custom type or custom trait to a external type but not externatl trait to external type, as an example: i. Display on Tweet, ii. Summary on Vec<T> but not iii. Display on Vec<T>
5. We can have default implementations in traits, which can be overriden in the implementating type, syntax remains same
6. Traits by this way can have a method that is required to be implemented, but uses it in other method expecting result from the implementating type allowing layer of useful abstractions.
7. Traits can be used as parameter which denotes in method signature that the method accepts any type implementing the mentioned trait
Syntax fn notify(item: &impl Summary);
Elaborate Syntax fn notify<T: Summary>(item: &T);
Also known as "Trait bounds"
8. Multiple trait bounds are denoted with '+' in between
Syntax fn notify(item: &(impl Summary + Display))
Elaborate Syntax fn notify<T: Summary + Display>(item: &T);
9. The generic function could have multiple trait bounds with multiple generic items like so:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 
in such cases we can use where clause like:
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
10. As parameters functions can also return types that implement traits
fn returns_summarizable() -> impl Summary 
useful in Closures and iterators
11. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library
12. impl<T: Display> ToString for T {
    // --snip--
}
Here ToString is a trait defining methods like to_string in rust library and expecting type which implements trait Display so 3.to_string() works as numbers type internally would be implementing Display trait, but for this to work number type must also be implementing ToString trait, this trait features achives blanket implementation and thus achieves polymorphism
13. Rust favors composition over inheritance and achieves code reuse through traits and generics.
14. Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.