// hash maps store key value pairs, use a key to access a value
use std::collections::HashMap;

pub fn hash_stuff() {
    // hashmaps, like vectors, are homogeneous, the keys are all of one type and the values are all of one type
    let mut hashish = HashMap::new();
    // of course having a hashmap with values of bool might not be the most useful thing ever
    hashish.insert(String::from("Wildcats"), false);
    hashish.insert(String::from("Thunderdog"), true);

    let cities = vec![String::from("Manchester"), String::from("Hull"), String::from("Copenhagen"), String::from("Branson")];
    let ratings = vec![12, 32, -1, 6];

    // .zip creates a vector of tuples of cities and their ratings
    // .collect moves them into the hashmap yelp
    let mut yelp: HashMap<_, _> = cities.into_iter().zip(ratings.into_iter()).collect();
    yelp.insert(String::from("Ulanbaatar"), 33);

    // gets the rating associated with the key Hull
    // returns Some() if that key exists, None if not
    let mine = yelp.get(&String::from("Hull"));
}
