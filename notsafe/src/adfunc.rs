// you can pass functions to functions as arguments
// this is useful for functions you've already defined (rather than using closures)
// function pointers implement all three closure traits (Fn, FnMut, and FnOnce)

fn add_one(x: i32) -> i32 {
    x + 1
}

fn double(x: i32) -> i32 {
    2 * x
}

// the parameter is a function with this signature
// fn is a type rather than a trait, so it can be specified directly instead of using generics with Fn trait bounds
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// you can't return closures directly because closures are represented by traits
// generally with traits you can just return a concrete type that implements that trait
// but closures don't have a concrete type that's returnable
// instead you can return a dynamically sized function pointer inside a box
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

pub fn funky() {
    let res = do_twice(add_one, 12);
    println!("{}", res);

    // any function with the right signature can be used as an argument in do_twice
    let ans = do_twice(double, 3);
    println!("{}", ans);

    let list_one = vec![1, 3, 6];
    // here we have a closure defined inline to convert each element in the iterator to a string
    let str_one: Vec<String> = list_one.iter().map(|i| i.to_string()).collect();
    println!("{:?}", str_one);

    let list_two = vec![2, 5, 1];
    // here we call the function to_string defined for the ToString trait to convert the elements of the iterator to strings
    let str_two: Vec<String> = list_two.iter().map(ToString::to_string).collect();
    println!("{:?}", str_two);
}
