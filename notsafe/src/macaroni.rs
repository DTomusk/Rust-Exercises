// macro refers to declarative macros and three kinds of procedural macros:
// custom #[derive] macros
// attribute like macros that define custom attributes
// function like macros that look like function calls but operate on the tokens specified (?)

// macros are a way of writing code that writes other code

// a simplified version of the vec! macro definition
// macro export shows that this macro should be made available whenever this crate is brought into scope
#[macro_export]
macro_rules! vec {
    // pattern to match against
    // macro patterns match against Rust code structure
    // the outer parentheses encompass the whole pattern
    // $x:expr matches any rust expression and calls it $x
    // , shows that a common could optionally follow the expression
    // * says that the pattern matches zero or more of the preceding pattern
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // pushes the $x's to the vec however many times
            $(
                temp_vec.push($x);
            )*
            // returns the finished temp_vec
            temp_vec
        }
    };
}
