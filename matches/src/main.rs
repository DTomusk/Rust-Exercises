fn main() {
    println!("Hello, world!");
    eflit();
    willet();
    fore();
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

fn willet() {
    let mut mingle = Vec::new();

    mingle.push(1.2);
    mingle.push(1.7);
    mingle.push(10.32);
    mingle.push(4.24);
    mingle.push(5.5);
    mingle.push(32.9);
    mingle.push(7.4);
    mingle.push(22.2);
    mingle.push(80.0);
    mingle.push(53.5);

    // perform a pop, if it matches the pattern run the code and loop
    while let Some(val) = mingle.pop() {
        println!("{}", val);
    }
}

fn fore() {
    let mut mec = vec!['b', 'a', 't', 'c', 'a', 't'];

    mec.push('#');
    mec.push('$');

    for (i, v) in mec.iter().enumerate() {
        println!("{}, {}", i, v);
    }
}
