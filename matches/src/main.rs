// function parameters, let statements and for loops can only accept irrefutable patterns

struct Numb {
    one: i32,
    two: i32,
    three: i32,
}

enum Blini {
    Cork,
    Fork,
    Mystery(Numb),
    Oak { i: i32, j: bool },
    Margaret(i32, u32, f64),
}

fn main() {
    eflit();
    willet((8.1, 6.2, 10.0));
    fore();

    // let is a pattern matching statement of the form: let PATTERN = EXPRESSION
    // here the tuple pattern on the left matches that on the right and each variable is assigned accordingly
    let (a, b, c, d) = (1, false, String::from("chowder"), -0.1);

    let torp = 62;

    match torp {
        7 | 18 | 0 => println!("Game time started"),
        8..=15 => println!("Such wonderful muskiness"),
        _ => println!("We are for the big"),
    }

    // you can destructure a struct using a pattern as follows
    let mis = Numb { one: torp, two: 128, three: 103 };
    let Numb { one: beep, two: boop, three: brap } = mis;

    match boop {
        // use ..= to refer to a range, works for integers and characters
        50..=120 => println!("You've done it Wilson"),
        _ => println!("Let's go to the mall"),
    }

    let bup = Numb { one: 1, two: 78, three: 31 };

    // match on specific field values
    match bup {
        Numb { one, two, three: 6 } => println!("Three is six when one is {} and two is {}", one, two),
        Numb { one, two: 12, three } => println!("Two is twelve when one is {} and three is {}", one, three),
        Numb { one, two, three } => println!("The values are {}, {}, {}", one, two, three),
    }

    let ran = Blini::Fork;

    // destructuring enums, including those with struct values
    match ran {
        Blini::Fork => println!("Mickey Rourke"),
        Blini::Cork => println!("Dork"),
        Blini::Mystery( Numb { one, .. } ) => println!("Value is {}", one),
        Blini::Oak { .. } => println!("AAron Batteries"),
        _ => (),
    }

    let a = 5;
    let b = 3;
    let c = 4;
    let d = 8;
    let e = 1;

    // matching a tuple and ignoring some of the values 
    match (a, b, c, d, e) {
        (5, _, _, 8, 1) => println!("You're a lucky one"),
        _ => println!("We ordered different drinks at the same bar"),
    }
}

fn eflit() {
    let me: Option<i32> = None;
    let you: Option<bool> = Some(false);
    let him = 89;
    let her = true;
    let them = String::from("Major key alert");

    if !her {
        println!("It's not her");
    } else if let Some(thing) = you {
        if thing {
            if him > 60 {
                println!("There's still so much to see");
            } else {
                println!("{}", them);
            }
        } else if let Some(ting) = me {
            println!("You've found the {}", ting);
        } else {
            println!("Everything's gone wrong today");
        }
    }
}

fn willet((x, y, z): (f64, f64, f64)) {
    let mut mingle = Vec::new();

    mingle.push(1.2);
    mingle.push(1.7);
    mingle.push(10.32);
    mingle.push(4.24);
    mingle.push(x);
    mingle.push(5.5);
    mingle.push(32.9);
    mingle.push(7.4);
    mingle.push(22.2);
    mingle.push(z);
    mingle.push(80.0);
    mingle.push(53.5);
    mingle.push(y);

    // perform a pop, if it matches the pattern run the code and loop
    while let Some(val) = mingle.pop() {
        println!("{}", val);
    }
}

fn fore() {
    let mut mec = vec!['b', 'a', 't', 'c', 'a', 't'];

    mec.push('#');
    mec.push('$');

    // iter turns a collection into an iterator, enumerate turns that into an iterator of tuples containing the index and value for each element of the iterator
    for (i, v) in mec.iter().enumerate() {
        println!("{}, {}", i, v);
    }
}
