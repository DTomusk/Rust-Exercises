mod ips;
// imagine playing a video game, the player character has a number of actions and each action
// has a different behaviour
// an enum is almost like a switch case dealy
// an enum can only be one of its variants
enum Action {
    walk,
    jump,
    consoom,
    menu,
    dab,
}

enum Class {
    proletarian(Proletarian),
    pariyah,
    brahman,
    upperMiddle,
    sophomore,
}

impl Class {
    // social classes are similar to each other, but also different
    // that's why defining them in an enum is convenient
    // if there's a thing every class does we can write a function for that here
}

struct Character {
    name: String,
    class: String,
}

struct Proletarian {
    // option is an enum with variants Some() and None
    // instead of having nulls, you can explicitly say whether you expect something not to
    // be able to have a value
    possessions: Option<String>,
    will2live: bool,
    // true, false, or none, doesn't make sense in context but you can still do it
    will2power: Option<bool>,
}

// imagine events are associated with actions and the characters they're performed by
// not sure if that would make sense, but it's an illustration
struct Event {
    movement: Action,
    character: Character,
}

// function for handling behaviour for variants of action
fn handle_event(event: Action) -> Option<String> {
    // use the match control flow operator to deal with different variants
    // if the thing you've given me is this variant do this, if it's this variant do that or whatever
    // matches are exhaustive so they have to account for all possibilities (_ handles whatever's left)
    match event {
        Action::walk => Some(String::from("Massive")),
        _ => None,
    }
}

fn main() {
    let this = Action::jump;
    let that = Action::dab;

    handle_event(this);

    let julian = Class::proletarian(Proletarian{
        possessions: Some(String::from("Two bears")),
        will2live: false,
        will2power: None,
    });

    // if let is essentially a match statement that only checks for one variant
    if let Action::jump = that {
        println!("I don't really understand if let yet");
    } else {
        println!("I think I get it more now");
    }
}
