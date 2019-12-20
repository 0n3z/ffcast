#[cfg(test)]

#[macro_use]
extern crate lazy_static;

extern crate ffcast;
use std::collections::HashMap;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::rc::Rc;
//use std::sync::Arc;
use ffcast::transcode_store::TranscodeStore;
use ffcast::transcode_task::TranscodeTask;
use ffcast::ffjob::FfJob;
//#[derive(PartialEq, Eq, Hash)]
use rand;

lazy_static! {
    static ref STORE: TranscodeStore = TranscodeStore::new();
}


#[test]
fn test_map_race() {
    let mut stderr = std::io::stderr();
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    let dani = contacts.get(&"Daniel");

    //contacts.remove(&"Daniel");
    match dani {
        Some(&number) => writeln!(&mut stderr, "Calling Daniel: {}", number),
        _ => writeln!(&mut stderr, "Don't have Daniel's number."),
    }.expect("woa");


/*

    match contacts.get(&"Daniel") {
        Some(&number) => writeln!(&mut stderr, "Calling Daniel: {}", number),
        _ => writeln!(&mut stderr, "Don't have Daniel's number."),
    }.expect("woa");

    contacts.remove(&"Ashley");

    // `HashMap::iter()` returns an iterator that yields
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in contacts.iter() {
        writeln!(&mut stderr, "Calling {}: {}", contact, number);
    }

    //assert_eq!(2 + 1, 4);
*/
}

#[test]
fn test_rc() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a is cloned to rc_b ---");

            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }

    // Error! `rc_examples` already moved into `rc_a`
    // And when `rc_a` is dropped, `rc_examples` is dropped together
    // println!("rc_examples: {}", rc_examples);
    // TODO ^ Try uncommenting this line
    let _task = TranscodeTask::new();
}

#[test]
fn test_store_concurrency() {
    //let store = TranscodeStore::new();
    let j = FfJob::new();
    for _i in 0..10 {
        let j1 = j.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(rand::random::<u64>()));
            let t1 = STORE.get_or_create_task(j1);
            println!("djkfsdkjfk");
        });
    }
//    let j1 = j.clone();
  //  let t = store.get_or_create_task(j);
    //let t1 = store.get_or_create_task(j1);

}
