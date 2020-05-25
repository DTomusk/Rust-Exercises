// the newtype pattern is a way to achieve encapsulation

// type alias, an instance of Kilometers is just an i32 with a different name
type Kilometers = i32;

// type aliases are generally used to reduce repetition, instead of writing this whole definition multiple times you can now just write Wesley
type Wesley = Box<dyn Fn() + Send + 'static>;

pub fn type_stuff() {
    let a: i32 = 1;
    let b: Kilometers = 17;

    // a and b have the same type because Kilometers is an alias for i32
    println!("{}", a + b);
}

// the never type (!) never returns
// never can be coerced into any type
// panic! returns a never type

// dynamically sized types must always be put behind a pointer
// every trait is a dyamically sized type
// the name of the trait can be used to refer to the trait
// to use traits as trait objects they must be put behind a pointer 
