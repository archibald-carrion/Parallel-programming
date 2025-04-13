use std::thread;

fn main() {
    let new_thread = thread::spawn(|| {
        println!("Hello from a spawned thread!");
    });

    println!("Hello from the main thread!");

    /*
    alternative would be to use let _ to ignore the result
    let _ = new_thread.join(); // Wait for the spawned thread to finish
     */

    let res = new_thread.join(); // Wait for the spawned thread to finish

    match res {
        Ok(_) => println!("Spawned thread has finished successfully."),
        Err(e) => println!("Spawned thread has panicked: {:?}", e),
    }
}