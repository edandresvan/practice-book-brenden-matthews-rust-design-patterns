fn main() {
  println!("********** Bark closure **********");

  let bark = || println!("Bark, bark!");
  bark();

  println!("********** Increment closure **********");

  let increment = |value| value + 1;
  let incr = increment(1);
  println!("increment is now {incr}.");
  let incr = increment(1);
  println!("increment is now {incr}.");
  let incr = increment(2);
  println!("increment is now {incr}.");
  let incr = increment(-4);
  println!("increment is now {incr}.");

  println!("********** Increment closure 2 **********");

  let print_anc_increment = |value| {
    print!("{value} will be incremented.");
    value + 1
  };
  println!(" -> {} was returned.", print_anc_increment(4));
  println!(" -> {} was returned.", print_anc_increment(2));
  println!(" -> {} was returned.", print_anc_increment(-3));

  println!("********** Higher order functions **********");

  let left_value = || 1;
  let right_value = || 2;
  let adder = |left: fn() -> i32, right: fn() -> i32| left() + right();
  let result = adder(left_value, right_value);
  println!("Adder result is: {result}.");

  println!("********** Closure variable capture **********");

  let consumable = "cookie".to_string();
  let consumer = move || consumable + " and coffee";

  println!("{}", consumer());
  //println!("{}", consumer());
}
