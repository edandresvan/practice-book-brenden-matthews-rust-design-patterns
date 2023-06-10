use std::{
  sync::{Arc, Condvar, Mutex, MutexGuard},
  thread,
};

fn main() {
  println!("********** RAII status **********");

  let status: String = String::from("Active");
  let statuses: Vec<String> = vec![status];
  // println!("{:?}", status);
  println!("{:?}", statuses);

  println!("********** RAII Mutex **********");

  // Tuple with:
  //   - a mutex containing an integer
  //   - a condition variable
  // The Arc is a thread-safe reference counter to the mutex and condition variable.
  let outer: Arc<(Mutex<i32>, Condvar)> = Arc::new((Mutex::new(0), Condvar::new()));
  // Clone the tuple
  let inner: Arc<(Mutex<i32>, Condvar)> = outer.clone();

  thread::spawn(move || {
    // Move the inner tuple and unpack it to a new mutex and variable
    let (mutex, cond_var) = &*inner;
    // Lock the mutex in order to create a guard value
    let mut guard: MutexGuard<i32> = mutex.lock().unwrap();
    // Increase the containe integer
    *guard += 1;

    println!("inner guard = {guard}");
    // Notify that changes has been made
    cond_var.notify_one();
  });

  let (mutex, cond_var) = &*outer;
  // Lock the mutex in order to create a guard value
  let mut guard: MutexGuard<i32> = mutex.lock().unwrap();

  println!("outer before wait guard = {guard}.");

  // Loop in the outer thread until the guard changes its zero value
  while *guard == 0 {
    // Request for a new guard value when notified
    guard = cond_var.wait(guard).unwrap();
  }

  println!("outer after wait guard = {guard}.");
}
