use oop::{Screen, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button{
                width: 32,
                height: 48,
                label: String::from("Batmanmobile"),
            }),
        ],
    };

    screen.run();
}

// trait objects incur a runtime cost because of dynamic dispatch, the compiler can't know
// the possible types beforehand so it has to handle them during runtime
// normal generics don't suffer from this because the compiler works out what type they'll be
// subtitutes the right types in (monomorphisation)

// trait objects must be object safe
// this means they can't return Self and they can't have generic parameters 
