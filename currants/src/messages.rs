// a channel has two halves: a transmitter and a receiver
// a channel transfers data between the transmitter and the receiver
// a channel is closed if either the transmitter or the receiver is dropped

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn mossage() {
    // mpsc stands for multiple producer single consumer
    // a channel can have multiple transmitters but only one receiver
    // tx is used to refer to the transmitter and rx to the receiver
    let (tx, rx) = mpsc::channel();

    // clone transmitter to make another transmitter
    let tx1 = mpsc::Sender::clone(&tx);

    // call move to move tx into the thread
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("there"),
            String::from("general"),
            String::from("kenobi"),
            String::from("you"),
            String::from("are"),
            String::from("a"),
            String::from("bold"),
            String::from("one"),
        ];
        // send returns a result, we don't care too much for dealing with it in this example so we'll just unwrap
        // if the receiver has been dropped then we'll get an error
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // we now have multiple transmitters going to one receiver rx 
    thread::spawn(move || {
        let vals = vec![
            String::from("a"),
            String::from("simple"),
            String::from("spell"),
            String::from("but"),
            String::from("quite"),
            String::from("unbreakable"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // keep rx in the main thread
    // recv blocks the main thread's execution until it receives something
    // the alternative is try_recv which doesn't block but returns a Result
    // try_recv can be used when we want the thread to do other stuff as well
    for received in rx {
        println!("Received: {}", received);
    }
}
