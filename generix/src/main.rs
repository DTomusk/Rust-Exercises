// "generics are abstract stand-ins for concrete types or other properties"
// functions can take generic arguments without knowing the specific type
// Option<T> and Result<T, E> both use generics

// fields can be any type, could be useful
struct gen<T, U, V> {
    x: T,
    y: U,
    z: V
}

// methods can only be called if field types are as below
impl gen<i32, bool, String> {
    fn do_something(&self) {
        if self.y {
            println!("{:?}", &self.z);
        } else {
            println!("{:?}", &self.x);
        }
    }
}

struct jan<T> {
    mor: T,
    gan: T,
}

impl jan<i32> {
    fn product(&self) -> i32 {
        self.mor * self.gan
    }
}

// similar to how the enums result and option work
enum choice<T, U, V> {
    a(T),
    b(U),
    c(V),
}

fn main() {
    let meow = gen { x: String::from("melancholia"), y: "Chortle", z: false };
    // this won't work because the type of the fields has to be the same
    //let mister = jan { mor: 1.2, gan: 7 };
}

fn largest<T>(list: &[T]) -> &T {
    &list[0]
}

/*
Rust code doesn't run any slower using generics
Rust uses monomorphization which turns generic code into concrete by filling in the types itself
The compiler looks at all the places where generic code is called and swaps them out for what they really are
*/
