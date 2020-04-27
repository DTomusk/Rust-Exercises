// a struct is like a class's attributes
#[derive(Debug)]
struct Person {
    // fields are strings not &str because we want instances to own their data
    surname: String,
    height: f64,
    weight: f64,
    zodiac: String,
    sexy: bool,
}

impl Person {
    fn bmi(&self) -> f64 {
        self.weight / (self.height*self.height)
    }

    fn related(&self, other: &Person) -> bool {
        if self.surname == other.surname {
            true
        } else {
            false
        }
    }

    // associated function, doesn't take self as a parameter 
    fn hot_boy(sur: String, zod: String, hei: f64, wei: f64) -> Person {
        Person {
            surname: sur,
            zodiac: zod,
            height: hei,
            weight: wei,
            sexy: true,
        }
    }
}

// struct without named fields (tuyple struct)
struct Dimensions(f32, f32, f32);

impl Dimensions {
    fn volume(&self) -> f32 {
        self.0*self.1*self.2
    }
}

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
        weight: 50.6
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
        weight: 12.3,
    };

    // Struct update syntax to create a new instance from another instance
    let titchmel = Person {
        surname: String::from("Stormzy"),
        ..mordecai
    };

    let size = Dimensions(9.6, 1.1, 32.0);

    println!("{:?}", size.volume());

    println!("{:#?}", titchmel);

    println!("Titchmel's bmi: {:?}", &titchmel.bmi());
}

fn constructor(surname: String, zodiac: String, sexy: bool) -> Person {
    Person {
        surname,
        zodiac,
        sexy,
        height: 0.54,
        weight: 6.63,
    }
}
