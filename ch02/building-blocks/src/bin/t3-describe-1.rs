/// Trait for providing a description.
trait SelfDescribing {
  /// Gets a description.
  fn describe(&self) -> String;
}

fn describe_type<T>(t: &T) -> String
where
  T: SelfDescribing,
{
  t.describe()
}

struct Dog();

impl SelfDescribing for Dog {
  fn describe(&self) -> String {
    "happy little dog".to_string()
  }
}

struct Cat();

impl SelfDescribing for Cat {
  fn describe(&self) -> String {
    "curious cat".to_string()
  }
}

fn main() {
  println!("********** Describing dog and cat **********");

  let cat = Cat();
  let dog = Dog();

  println!("I am a '{}'", describe_type(&cat));
  println!("I am a '{}'", describe_type(&dog))
}
