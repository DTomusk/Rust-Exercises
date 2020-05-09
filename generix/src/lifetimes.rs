// a lifetimes is the scope for which a reference is valid
// usually lifetimes are implicit and inferred
// the main aim of lifetimes is to prevent dangling references
// rust uses a borrow checker to compare the scopes of variables and make sure lifetimes are valid
fn lifetime_stuff() {
    let strang = "dapht punc";
    let strong = "conyay west";

    let res = longest(strang, strong);

    let strung = String::from("dzei zee");
    {
        let streng = String::from("little wein");
        // regardless of whether ser is set to strung or streng it will go out of scope in these braces
        // because they have the same lifetime
        let ser = longest(strung.as_str(), streng.as_str());
    }

    // static lifetimes last for the entire duration of the program
    let s: &'static str = "I last forever";
}

// need to specify a lifetime in this case as the compiler doesn't know which reference to return and which to drop
// as x and y have the same generic lifetime the compiler knows they need to live as long as each other
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// for structs to hold references their lifetimes have to be annotated
// an instance of thingy can only last as long as the shortest lived field
// only if the referenced value is in scope can you use an instance of thingy
struct thingy<'a> {
    blorb: &'a str,
    merb: &'a str,
}

/*
the compiler uses three rules to figure out what lifetimes references have when they aren't explicit
one rule applies to input lifetimes and the other two apply to output lifetimes
if the compiler can't account for all references using the three rules then an error is flagged
1) each parameter that is a reference gets its own liftime parameter
2) there is exactly one input lifetime parameter which is assigned to all output lifetime parameters
3) if one of the inputs is &self or &mut self the lifetime of self is assigned to all output lifetime parameters
*/
