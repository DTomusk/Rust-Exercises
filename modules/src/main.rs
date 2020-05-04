// another file is a module of itself 
mod mods;

// brings rand into scope so we can use its functions and traits
use rand::Rng;
// change the name if you want
use office::accounting as Accounts;
// list things like so to avoid repetition
use office::{sales, hr};

// this will mostly be note taking, but I'll write some stuff to put it into practice as well

fn main() {
    // Packages are a cargo feature that let you build, test, and share crates
    // Crates are trees of modules that produce libraries or executables
    // Modules let you control the organization, scope, and privacy of paths
    // Paths are ways of naming an item (such as structs, functions, and modules)

    // A crate is a binary or library
    // the crate is where the compiler starts, it makes up the root module of the crate
    // a package contains some number of crates that provides a set of functionality
    // think crates.io or external libraries in other languages
    // a package can have any number of binary crates placed in the src/bin directory
    // crates group related functions together

    // a binary is something you run will a library contains functionality (and can contain binaries)

    // rand contains a struct thread_rng which implements the function gen_range
    let random = rand::thread_rng().gen_range(1,50);
    Accounts::taxes();

    mods::nest::draw_stuff();
}

// modules can be nested inside other modules
// modules contain related functions and divide up the program into manageable parts
// need to use the pub keyword to expose modules and functions or else they can't be used externally
// privacy rules apply to structs, enums, functions, methods and modules
mod office {
    pub mod accounting {
        pub fn taxes() {
            solitaire();
            // use super to access the module above
            super::hr::handle_harassment();
        }

        fn solitaire() {}
    }

    pub mod sales {
        pub fn sell_product() {}

        fn record_sale() {}

        fn water_cooler_gossip() {}
    }

    pub mod hr {
        pub fn handle_harassment() {}

        fn discuss_things() {}
    }

    pub mod management {
        fn talk_to_corporate() {}

        pub fn host_meeting() {}
    }

    pub mod warehouse {
        pub fn load_stuff() {}

        pub fn ship() {}

        fn unload_stuff() {}

        fn play_ping_pong() {}
    }

    pub struct Employee {
        // you can know his name
        pub name: String,
        // but you can't know if he's happy
        happy: bool,
    }

    pub enum position {
        wage_slave,
        middle_management,
        temp,
    }
}

pub fn make_sale() {
    // absolute path starts from the crate root
    crate::office::management::host_meeting();
    crate::office::sales::sell_product();
    // relative path starts from current module, because office is in this file we can access it directly
    office::warehouse::load_stuff();
    office::warehouse::ship();
}
