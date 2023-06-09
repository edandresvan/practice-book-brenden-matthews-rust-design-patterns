
/// Represents a pumpkin.
#[derive(Debug, Clone, Default)]
struct Pumpkin {
  /// Mass of the pumkin.
  mass: f64,
  /// Diameter of the pumkin.
  diameter: f64,
}

/* 
impl Default for Pumpkin {
  fn default() -> Self {
      Self { mass: 2.0, diameter: 5.0 }
  }
}
*/

fn main() {
  println!("********** Creating pumpkins **********");

  println!("Default pumpking: {:?}", Pumpkin::default());

  let big_pumpkin = Pumpkin {
    mass: 60.0, diameter: 50.0
  };

  println!("Big pumpkin: {:?}", big_pumpkin);
  println!("Cloned big pumpkin: {:?}", big_pumpkin.clone());
  
}