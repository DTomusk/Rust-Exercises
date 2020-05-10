fn main() {
    println!("Hello, world!");
}

struct Line {
    origin: (f32, f32),
    gradient: (f32, f32),
}

impl Line {
    fn parallel(&self, other: &Line) -> bool {
        self.gradient == other.gradient
    }
}

// use cargo test to run tests
#[cfg(test)]
mod tests {
    // use all functions from the above module
    use super::*;
    // show the following function is a test
    #[test]
    fn add() {
        // macro that checks the first argument is equivalent to the second
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn fail() {
        // can add a string literal as a custom error message
        assert_eq!(1 + 1, 1, "Sum failed");
    }

    #[test]
    fn pllel() {
        let one = Line {
            origin: (0.0, 2.3),
            gradient: (1.0, 1.7),
        };
        let two = Line {
            origin: (12.0, 5.2),
            gradient: (1.0, 1.7),
        };
        let three = Line {
            origin: (40.1, 0.8),
            gradient: (1.0, 1.0),
        };
        assert!(!two.parallel(&three), false);
        assert!(one.parallel(&two));
    }

    #[test]
    // if we want to check that an erroneous case will panic we can tell the test it should panic
    // can make test more precise by saying which error message you expect
    #[should_panic(expected = "You will never see this message")]
    fn panic() {
        panic!("You will never see this message");
    }

    // instead of using the assert macro to check panics, you can use the result type
    // if ok returned then test passed, if error returned then failed 
    #[test]
    fn results() -> Result<(), String> {
        if true {
            Ok(())
        } else {
            Err(String::from("False"))
        }
    }
}

/*
To test:
1) set up necessary data/states
2) run testing code
3) assert the results are as expected
*/
