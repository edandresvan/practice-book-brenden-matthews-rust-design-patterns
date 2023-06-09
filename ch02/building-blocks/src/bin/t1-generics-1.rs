/// Represents a container for any generic type.
struct Container<T> {
  value: T,
}

impl<T> Container<T> {
  /// Initializes a new instance of the container.
  fn new(value: T) -> Self {
    Self { value: value }
  }
}

fn main() {
  println!("********** Basic container **********");

  let str_container: Container<&str> = Container {
    value: "watermelon",
  };

  println!("{}", str_container.value);

  let ambiguos_container: Container<Option<String>> = Container { value: None };

  println!("{:?}", ambiguos_container.value);

  let short_alt_ambiguos_container = Container::<Option<String>>::new(None);

  println!("{:?}", short_alt_ambiguos_container.value);
}
