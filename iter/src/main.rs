// iterators implement the Iterator trait which has one method next which either returns Some(next value) or None once iteration is over
use rand::prelude::*;

struct Miter {
    value: u16,
}

impl Miter {
    fn new() -> Miter {
        Miter {
            value: random(),
        }
    }
}

// iterator keeps going until it generates 7
impl Iterator for Miter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value != 7 {
            self.value = random();
            Some(self.value)
        } else {
            None
        }
    }
}

fn main() {
    // iterator adaptors are methods defined on the iterator trait that allow you to change iterators into different  kinds of iterators
    let my_vec: Vec<i32> = vec![1, 2, 7, 3, 8, 0, 0, 8];

    // turns my_vec into an iterator and then doubles each value
    // again, have to specify that the collection is a vector
    let your_vec: Vec<_> = my_vec.iter().map(|x| 2 * x).collect();

    println!("{:?}", &your_vec);

    let miter = Miter::new();
    let mut count = 0;

    for m in miter {
        println!("{}", m);
        count += 1;
    }

    println!("It took {} tries to get to 7", count);

    let vort: Vec<i32> = vec![1,2,3,4,5,6,7,8];
    let virt: Vec<bool> = vec![false, false, true, false, true, true, true, false, true];
    // zip combines two iterators to make one iterator of the length of the shorter iterator 
    let vert = vort.iter().zip(virt.iter());
    for v in vert {
        println!("{:?}", &v);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_test() {
        let v1 = vec![1, 2, 3];
        // creates an iterator from v1
        // .into_iter() takes ownership of the data while .iter_mut uses mutable references
        let mut v1_iter = v1.iter();

        // every iteration "eats up" a value from the iterator
        // the values wrapped in Some() are immutable references
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn summation() {
        let vect = vec![2.7, 6.5, 12.1, 5.5];
        let viter = vect.iter();
        // sum, like next, is a consuming adaptor, calling it uses up the iterator
        let total: f64 = viter.sum();
        // floating points are fun
        assert_eq!(total, 26.799999999999997);
    }

    #[test]
    fn filtre() {
        let v = vec![1, 7, 2, 21, 3, -3, 7, 90, 12, 5, 64, 28, 1, -14];
        // filter uses a closuse to filter the items that satisfy an expression
        // remember, into_iter takes ownership where iter doesn't
        let w: Vec<_> = v.into_iter().filter(|x| x <= &20).collect();

        assert_eq!(w, vec![1, 7, 2, 3, -3, 7, 12, 5, 1, -14]);
    }
}
