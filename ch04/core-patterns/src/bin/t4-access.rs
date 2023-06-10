use std::vec;

#[derive(Debug, Clone)]
struct Pizza {
  toppings: Vec<String>,
}

impl Pizza {
  fn new(toppings: Vec<String>) -> Self {
    Self { toppings }
  }

  /// Gets a read-only list of toppings.
  fn toppings(&self) -> &[String] {
    &self.toppings
  }

  /// Gets a read-write list of toppings.
  fn toppings_mut(&mut self) -> &mut Vec<String> {
    &mut self.toppings
  }

  fn set_toppings(
    &mut self,
    toppings: Vec<String>,
  ) {
    self.toppings = toppings
  }

  fn replace_toppings(
    &mut self,
    toppings: Vec<String>,
  ) -> Vec<String> {
    std::mem::replace(&mut self.toppings, toppings)
  }
}

fn main() {
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

  println!("********** Pizza removing toppings **********");

  let mut pizza = Pizza {
    toppings: vec![String::from("sauce"), String::from("cheese")],
  };

  pizza.toppings.remove(1);
  println!("pizza = {:?}", pizza);

  println!("********** Pizza accessors and mutators **********");

  let toppings_list: Vec<String> = vec![
    String::from("tomato sauce"),
    String::from("mushrooms"),
    String::from("mozzarella"),
    String::from("pepperoni"),
  ];

  let pizza = Pizza::new(toppings_list.clone());
  let toppings: &[String] = pizza.toppings();
  println!("toppings read-only list = {:?}", toppings);

  let mut pizza: Pizza = Pizza::new(toppings_list.clone());
  let toppings: &mut Vec<String> = pizza.toppings_mut();
  println!("toppings read-write list before change = {:?}", toppings);
  toppings[1] = String::from("ricotta");
  toppings[2] = format!("double {}", toppings[2]);
  toppings.pop();
  println!("toppings read-write list after change = {:?}", toppings);
  println!("pizza toppings list after change = {:?}", toppings);

  let toppings_2: Vec<String> = vec![
    String::from("Mussels"),
    String::from("Garlic"),
    String::from("Olive oil"),
    String::from("Parsley"),
  ];
  let mut pizza = Pizza::new(Vec::new());
  println!("pizza before set = {:?}", pizza);
  pizza.set_toppings(toppings_2.clone());
  println!("pizza after set = {:?}", pizza);

  let toppings_3: Vec<String> = vec![
    String::from("Cheddar"),
    String::from("Macaroni"),
    String::from("Feta"),
  ];

  let mut pizza = Pizza::new(toppings_2.clone());
  println!("pizza before replace = {:?}", pizza);
  pizza.replace_toppings(toppings_3);
  println!("pizza after replace = {:?}", pizza);

}
