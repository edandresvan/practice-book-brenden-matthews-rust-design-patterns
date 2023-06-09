trait MyTrait {
  fn trait_hello(&self);
}

struct MyStruct1 {}

impl MyStruct1 {
  fn struct_hello(&self) {
    println!("Hello from MyStruct1");
  }
}

impl MyTrait for MyStruct1 {
  fn trait_hello(&self) {
    self.struct_hello();
  }
}

struct MyStruct2 {}

impl MyStruct2 {
  fn struct_hello(&self) {
    println!("Hello from MyStruct2");
  }
}

impl MyTrait for MyStruct2 {
  fn trait_hello(&self) {
    self.struct_hello();
  }
}

fn main() {
  println!("********** Creating trait objects **********");

  let mut objects: Vec<Box<dyn MyTrait>> = Vec::new();

  objects.push(Box::new(MyStruct1 {}));
  objects.push(Box::new(MyStruct2 {}));

  objects.iter().for_each(|object| object.trait_hello());
}
