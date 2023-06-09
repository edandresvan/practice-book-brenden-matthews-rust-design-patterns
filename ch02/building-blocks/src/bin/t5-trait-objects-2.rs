use std::any::Any;

trait MyTrait {
  fn trait_hello(&self);
  fn as_any(&self) -> &dyn Any;
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

  fn as_any(&self) -> &dyn Any {
    self
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

  fn as_any(&self) -> &dyn Any {
    self
  }
}

fn main() {
  println!("********** Downcasting trait objects **********");

  let mut objects: Vec<Box<dyn MyTrait>> = Vec::new();

  objects.push(Box::new(MyStruct1 {}));
  objects.push(Box::new(MyStruct2 {}));

  objects.iter().for_each(|object| {
    // Try to downcast to MyStruct1
    if let Some(o) = object.as_any().downcast_ref::<MyStruct1>() {
      o.struct_hello();
    }

    // Try to downcast to MyStruct2
    if let Some(o) = object.as_any().downcast_ref::<MyStruct2>() {
      o.struct_hello();
    }
  });
}
