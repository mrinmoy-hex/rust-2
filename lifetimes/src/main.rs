/// Rust lifetimes explain how long references remain valid.
///
/// Every reference has a lifetime, even when the compiler can infer it for us.
/// We only need explicit lifetime annotations when the compiler cannot determine
/// the relationship between borrowed values. In short, lifetimes tell Rust how
/// long a reference may be used without becoming invalid.
///
/// Rust has three lifetime elision rules that the compiler follows automatically:
///
/// 1. Each elided lifetime in a function parameter gets its own distinct lifetime.
///    Example: `fn foo(x: &str, y: &str)` is treated as
///    `fn foo<'a, 'b>(x: &'a str, y: &'b str)`.
/// 2. If there is exactly one input lifetime, that lifetime is assigned to all
///    elided output lifetimes. Example: `fn first<'a>(x: &'a str) -> &str` is
///    effectively `fn first<'a>(x: &'a str) -> &'a str`.
/// 3. If there are multiple input lifetimes, but one of them is `&self` or
///    `&mut self` in a method, then that lifetime is assigned to the output.
///    Example: `impl Foo { fn get(&self) -> &str }` becomes
///    `fn get<'a>(&'a self) -> &'a str`.
///
/// If none of the above applies, the compiler requires explicit lifetime
/// annotations. This is the "generic lifetime" rule set Rust uses for inference.
///
/// A lifetime annotation looks like `'a` and is placed on a type or function.
/// The annotation means: "this reference must live at least as long as `'a`".
///
/// Input lifetimes are the lifetimes of arguments. Output lifetimes describe the
/// lifetime of a returned reference. When a function returns a reference derived
/// from one of its inputs, the returned reference must not outlive the input it
/// came from.

// This struct stores a borrowed string slice that must remain valid as long as
// the struct itself exists.
struct ImportExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportExcerpt<'a> {
    /// Creates a new excerpt that borrows a portion of a larger string.
    fn new(part: &'a str) -> Self {
        Self { part }
    }
}

fn main() {
    // A lifetime is valid when both input strings are alive for the entire call.
    // Here both are string literals, so they live for the whole program.
    let string1 = "long string is long";
    let string2 = "xyz";

    let result = longest(string1, string2);
    println!("The longest string is: {result}");

    // We can also borrow a slice from a larger string and store it in a struct.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportExcerpt::new(first_sentence);
    println!("Excerpt: {}", excerpt.part);

    // A second example showing that a shorter lifetime can be tied to a specific scope.
    let shortest = "tiny";
    let longest_literal = "a much longer literal";
    let chosen = longest(shortest, longest_literal);
    println!("The winner between the two string literals is: {chosen}");
}

/// Returns the longer of two string slices.
///
/// This function is valid because the returned reference has the same lifetime as
/// the inputs: it can only be used while both inputs are still alive.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
