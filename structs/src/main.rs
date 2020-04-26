// a struct is like a class's attributes
struct Person {
    // fields are strings not &str because we want instances to own their data
    surname: String,
    height: f64,
    zodiac: String,
    sexy: bool,
}

// struct without named fields (tuyple struct)
struct Dimensions(i32, i32, i32);

// unit type struct with no fields, useful when you want to implement traits without using values 
struct Sword();

fn main() {
    // creating an instance of Person
    // the entire instance must be mutable, you can't have just some mutable fields
    let mut mordecai = Person {
        surname: String::from("Biggidybong"),
        zodiac: String::from("Aquarium"),
        sexy: true,
        height: 2.01,
    };

    mordecai.sexy = false;
    println!("{}", &mordecai.surname);

    let mikael = constructor(String::from("Bulbasaur"), String::from("Disjoint sets"), false);

    // set fields to another instance's
    let mitchel = Person {
        surname: mordecai.surname,
        zodiac: mikael.zodiac,
        sexy: false,
        height: -7.6,
    };

    // Struct update syntax to create a new instance from another instance
    let titchmel = Person {
        surname: String::from("Stormzy"),
        ..mordecai
    };

    let size = Dimensions(9.6, 1.1, 0.0);
}

fn constructor(surname: String, zodiac: String, sexy: bool) -> Person {
    Person {
        surname,
        zodiac,
        sexy,
        height: 0.54,
    }
}
