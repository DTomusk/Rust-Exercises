// traits are like interfaces in other languages
// use traits to define shared behaviour in an abstract way
// a type's behaviour is the set of methods we can call on that type
// different types have the same behaviour if we can call the same methods on them

// a trait basically defines requirements, any struct that implements Sing has to define the
// behaviour for fn sing()
// this trait isn't the most ambitious example
pub trait Sing {
    // can define default behaviour if not implemented explicitly
    fn sing(&self) -> String {
        String::from("Dance monkey dance monkey hey hey hey")
    }

    // unless we provide an implementation for a class, anything with the Sing trait will have the
    // default behaviour for song()
    fn song(&self) -> String {
        String::from("Grandpa's shoes are on the news")
    }
}

pub struct Person {
    name: String,
    age: i32,
    gender: char,
}

// specific implementation overrides the default
impl Sing for Person {
    fn sing(&self) -> String {
        String::from("Lalalalala skibbidy bibbidy bap")
    }
}

pub struct Bird {
    species: String,
    wingspan: f64,
}

impl Sing for Bird {
    fn sing(&self) -> String {
        String::from("Tweet tweet squaaaack")
    }
}

pub struct Cow {
    zodiac: String,
    goodness_of_breathe: bool,
}

// use an empty impl block to use default behaviour
impl Sing for Cow {}

// this function can be called on anything that implements Sing
// the following function signatures are equivalent
// pub fn crying<T: Sing>(item: T) {}
fn crying(item: impl Sing) {
    println!("{:?}", item.song());
    println!("{:?}", item.sing());
}

fn calling<T: Sing>(one: T, two: T) {
    println!("{:?}", one.song());
    println!("{:?}", two.sing());
}

fn do_something<T, U, V>(one: T, two: U, three: U, four: V)
    // specify which traits the generic arguments have to implement
    where T: Clone,
        U: PartialOrd + Sing,
        V: Clone + Sing
{}

// can return generic types that implement traits
fn return_trait() -> impl Sing {
    Person {
        name: String::from("Jemimah"),
        age: 62,
        gender: 'n',
    }
}
