#[derive(Debug, Clone)]
struct Pizza {
  toppings: Vec<String>,
}

impl Pizza {
  // fn new() -> Self {
  //   Self { toppings: vec![] }
  // }

  fn new(toppings: Vec<String>) -> Self {
    Self { toppings }
  }
}

fn main() {
  //println!("********** Creating pizza **********");

  // let pizza = Pizza::new();
  // println!("pizza = {pizza:?}");

  println!("********** Creating pizza with toppings **********");

  let toppings: Vec<String> = vec![
    String::from("tomato sauce"),
    String::from("mushrooms"),
    String::from("mozzarella"),
    String::from("pepperoni"),
  ];

  let pizza = Pizza::new(toppings);
  println!("pizza = {:?}", pizza);
  println!("pizza toppings = {:?}", pizza.toppings);

}
