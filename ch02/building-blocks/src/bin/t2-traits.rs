/// Represents a rectangle in geometry.
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  /// Initializes a new rectangle.
  pub fn new(
    width: u32,
    height: u32,
  ) -> Self {
    Self { width, height }
  }
}

/// Represents a square in geometry.
struct Square {
  length: u32,
}

impl Square {
  /// Initializes a new square.
  pub fn new(length: u32) -> Self {
    Self { length }
  }

  /// Gets the length of the square.
  pub fn get_length(&self) -> u32 {
    self.length
  }
}

/// Trait for rectangular behaviours.
pub trait Rectangular {
  fn get_width(&self) -> u32;
  fn get_height(&self) -> u32;
  fn get_area(&self) -> u32;
}

impl Rectangular for Rectangle {
  fn get_width(&self) -> u32 {
    self.width
  }

  fn get_height(&self) -> u32 {
    self.height
  }

  fn get_area(&self) -> u32 {
    self.width * self.height
  }
}

impl Rectangular for Square {
  fn get_width(&self) -> u32 {
    self.length
  }

  fn get_height(&self) -> u32 {
    self.length
  }

  fn get_area(&self) -> u32 {
    self.length.pow(2)
  }
}

fn main() {
  println!("********** Rectangle and square **********");

  let rectangle_1 = Rectangle::new(2, 3);
  let square_1 = Square::new(5);

  println!(
    "rectangle has width {0}, height {1}, and area {2}",
    rectangle_1.get_width(),
    rectangle_1.get_height(),
    rectangle_1.get_area()
  );

  println!(
    "square has length {0}, and area {1}",
    square_1.get_length(),
    square_1.get_area()
  );
}
