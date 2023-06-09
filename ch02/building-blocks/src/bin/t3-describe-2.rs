/// Trait for providing a description.
trait SelfDescribing {
  /// Gets a description.
  fn describe() -> String;
}

fn describe_type<T>() -> String
where
  T: SelfDescribing,
{
  T::describe()
}

struct Dog();

impl SelfDescribing for Dog {
  fn describe() -> String {
    "happy little dog".to_string()
  }
}

struct Cat();

impl SelfDescribing for Cat {
  fn describe() -> String {
    "curious cat".to_string()
  }
}

fn main() {
  println!("********** Describing dog and cat **********");
  
  println!("I am a '{}'", describe_type::<Cat>());
  println!("I am a '{}'", describe_type::<Dog>())
}
