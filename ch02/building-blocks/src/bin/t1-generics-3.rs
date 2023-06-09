use std::marker::PhantomData;

/// Represents a dog with a name and certain breed.
#[derive(Debug)]
struct Dog<Breed> {
  name: String,
  breed: PhantomData<Breed>,
}

#[derive(Debug)]
struct Poodle {}

#[derive(Debug)]
struct Labrador {}

#[derive(Debug)]
struct Retriever {}

#[derive(Debug)]
struct Dachshund {}

impl Dog<Poodle> {
  fn breed_name(&self) -> &str {
    "poddle"
  }
}

impl Dog<Labrador> {
  fn breed_name(&self) -> &str {
    "labrador"
  }
}

impl Dog<Retriever> {
  fn breed_name(&self) -> &str {
    "retriever"
  }
}

impl Dog<Dachshund> {
  fn breed_name(&self) -> &str {
    "dachshund"
  }
}

fn main() {
  println!("********** Dogs and breeds **********");

  let my_poodle: Dog<Poodle> = Dog {
    name: "Jeffrey".to_string(),
    breed: PhantomData,
  };
  println!("{my_poodle:?}");
  println!(
    "My dog {} is a '{}'.",
    my_poodle.name,
    my_poodle.breed_name()
  );

  let my_labrador: Dog<Labrador> = Dog {
    name: "Milo".to_string(),
    breed: PhantomData,
  };
  println!("{my_labrador:?}");
  println!(
    "My dog {} is a '{}'.",
    my_labrador.name,
    my_labrador.breed_name()
  );
}
