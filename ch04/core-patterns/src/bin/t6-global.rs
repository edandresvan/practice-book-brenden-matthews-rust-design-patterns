/*
static POPULAR_BABY_NAMES_2021: Vec<String> = vec![
  String::from("Olivia"),
  String::from("Liam"),
  String::from("Emma"),
  String::from("Noah"),
];
 */

use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use static_init::dynamic;

thread_local! {
  static PUPULAR_BABY_NAMES_2021 : Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
}

// lazy_static
lazy_static! {
  static ref POPULAR_BABY_NAMES_2020: Vec<String> = {
    vec![
      String::from("Olivia"),
      String::from("Liam"),
      String::from("Emma"),
      String::from("Noah"),
    ]
  };
}

// once_cell
static POPULAR_BABY_NAMES_2019: Lazy<Vec<String>> = Lazy::new(|| {
  vec![
    String::from("Olivia"),
    String::from("Liam"),
    String::from("Emma"),
    String::from("Noah"),
  ]
});

// static_init
#[dynamic]
static POPULAR_BABY_NAMES_2018: Vec<String> = vec![
    String::from("Emma"),
    String::from("Liam"),
    String::from("Olivia"),
    String::from("Noah"),
];

fn main() {
  let arc: Arc<Mutex<Option<Vec<String>>>> =
    PUPULAR_BABY_NAMES_2021.with(|arc| arc.clone());

  let mut inner: std::sync::MutexGuard<Option<Vec<String>>> =
    arc.lock().expect("Tt was not possible to tock the mutex.");

  *inner = Some(vec![
    String::from("Olivia"),
    String::from("Liam"),
    String::from("Emma"),
    String::from("Noah"),
  ]);

  println!("********** Global with thread_local arc **********");
  println!("popular baby names of 2021: {:?}", *inner);

  println!("********** Global with lazy_static **********");
  println!("popular baby names of 2020: {:?}", *POPULAR_BABY_NAMES_2020);

  println!("********** Global with once_cell **********");
  println!("popular baby names of 2019: {:?}", *POPULAR_BABY_NAMES_2019);

println!("********** Global with static_init **********");
  println!("popular baby names of 2018: {:?}", *POPULAR_BABY_NAMES_2018);
  
}
