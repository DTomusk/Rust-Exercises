// iterators implement the Iterator trait which has one method next which either returns Some(next value) or None once iteration is over

fn main() {
    println!("Hello, world!");
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
}
