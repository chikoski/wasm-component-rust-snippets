use std::thread;

fn main() {
    let th1 = thread::spawn(worker);
    //let th2 = thread::spawn(worker);

    match th1.join() {
        Ok(_) => (),
        Err(e) => println!("Error: {e:?}"),
    }
    // th2.join().unwrap();

    println!("Hello, world!");
}

fn worker() {
    let id = thread::current().id();
    println!("This is worker on thread ${id:?}!");
}