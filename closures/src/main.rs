// a closure stores a function in a variable
// the parameters are held in pipes (||) and then the function definition is given
// the variable assigned to it doesn't hold the result but the function itself
// closures aren't exposed to other users so you don't have to annotate parameter or return types
// closures are short and only relevant in a narrow context

use std::thread;
use std::time::Duration;

// stores an expensive calculation that we want to call only when completely necessary
struct Cacher<T>
where
    // generic type implements Fn trait, that is we can store a function as a type for a variable
    T: Fn(&str, bool) -> i32,
{
    // our closure function is stored in calc
    calc: T,
    val: Option<i32>
}

impl<T> Cacher<T>
where
    T: Fn(&str, bool) -> i32,
{
    // cacher constructor, takes a function with the above signature and sets the calc field to that
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            calc,
            val: None,
        }
    }

    // if there already is a value return it, otherwise call the function to set it and return it
    fn val(&mut self, one: &str, two: bool) -> i32 {
        match self.val {
            Some(v) => v,
            None => {
                let v = (self.calc)(one, two);
                self.val = Some(v);
                v
            }
        }
    }
}

// there is no real concept here, but nevertheless it's helped me understand closures better
fn main() {
    let standard_option = 50.09;
    the_choice_is_yours(standard_option);

    let bar = 7;
    let check = |n| {
        if n > bar {
            true
        } else {
            false
        }
    };

    assert!(check(6));
}

fn the_choice_is_yours(opt: f64) {
    // function is a bit long for a closure, but it works regardless
    // variable stores the closure in an instance of Cacher and the expensive calculation is only called when it's needed
    let mut hard_calculation = Cacher::new(|nub, bun| {
        println!("Thinking...");
        thread::sleep(Duration::from_secs(2));
        println!("Feeling...");
        thread::sleep(Duration::from_secs(2));
        println!("Developing unhealthy attachments...");
        thread::sleep(Duration::from_secs(1));
        if bun {
            println!("{}", nub);
        };
        52
    });

    // because of Cacher the closure function only ever has to be called once, or not at all if not applicable
    if opt < 12.3 {
        println!("{}", hard_calculation.val("Bonus round", true));
    } else if opt < 51.1 {
        println!("{}", hard_calculation.val("Burger round", true));
        println!("{}", hard_calculation.val("Badger round", false));
    } else {
        println!("That's all folks");
    }
}
