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
<details>
  <summary>References Lifetimes</summary>
    1. Every reference in Rust has a lifetime, which being scope until its valid
    2. Like types they are usually inferred, but we must annotate when lifetimes could be related in different ways.
    3. Aim being to avoid dangling references.
    4. if we assign reference of an inner scope to outer scope variable and use it, the borrow checker will give compile time error as lifetime of the inner scope variable is less then the outer scope variable who borrowed the reference.
    5. fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    This does not work as Rust cannot tell which reference we are returning as its necessary for rust to know the lifetime of the variable.
    6. Rust compiler could infer lifetimes relationships if they are same, need to follow set of rules called lifetime elision rules
    7. Just as functions accept generic types for parameters, they can accept references with any lifetime by specifying a generic lifetime parameter
    Syntax: &'a i32 or &'a mut i32
    8. Note, we just annotate lifetimes and not change any, borrow checker will adhere to and will reject if not followed the aforementioned constraints. We onl specify the lifetime of returned reference is same as smaller of the lifetimes of parameters.
    9. Lifetimes become the part of the function signature just like the types, thus it cannot be called from a place where dangling reference could have been inferred.
    10. This is helpful for compiler to point to our error more precisely rather then in a chain of downstream events getting called.
    11. Valid code: 
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    Invalid code: 
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    12. If x and y have different lifetimes, the function will return a reference that lives for as long as both x and y are valid.

</details>