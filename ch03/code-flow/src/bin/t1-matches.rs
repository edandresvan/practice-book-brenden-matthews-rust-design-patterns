/// Shows a message depending on the given option parameter.
fn some_or_none<T>(value: &Option<T>) {
  match value {
    Some(_v) => println!("Option has some value."),
    None => println!("Option does not value."),
  }
}

/// Shows a message depending on the range the given value belongs to.
fn what_type_of_integer_is_this(value: i32) {
  match value {
    1 => println!("{value} -> The number is one"),
    2 | 3 => println!("{value} -> The number is either two or three."),
    4..=10 => println!("{value} -> The number is between 4 and 10 inclusive."),
    _ => println!("{value} -> The number is less than 1 or greater than 10."),
  }
}

/// Shows the elements of the given tuple gradually.
fn destructure_tuple(tuple: &(i32, i32, i32)) {
  match tuple {
    (first, ..) => println!("First tuple element is: {first}."),
  }

  match tuple {
    (.., last) => println!("Last tuple element is: {last}."),
  }

  match tuple {
    (_, middle, _) => println!("Middle tuple element is: {middle}."),
  }

  match tuple {
    (first, middle, last) => {
      println!("Whole tuple is: ({first}, {middle}, {last}).")
    }
  }
}

/// Shows a message depending if the given value matches a condtional.
fn match_with_guard(value: i32) {
  match value {
    v if v == 1 => println!("{value} => Value is equal to 1."),
    v if v < 10 => println!("{value} => Value is less than 10."),
    _ => println!("{value} is greater or equal to 10."),
  }
}

/*
fn invalid_matching<T>(value: &T) {
  match value {
    "is a string" => println!("This is a string value."),
    1 => println!("This is an integer value.")
  }
}
*/

/// Represents an enumeration of distinct types.
enum DistinctTypes {
  Name(String),
  Count(i32),
}

/// Shows a message according to the type of the given value.
fn match_enum_types(value_type: &DistinctTypes) {
  match value_type {
    DistinctTypes::Name(name) => println!("name = {name}."),
    DistinctTypes::Count(count) => println!("count = {count}."),
  }
}

/// Represents the color of a cat.
enum CatColour {
  Black,
  Red,
  Chocolate,
  Cinnamon,
  Blue,
  Cream,
  Cheshire,
}

/// Represents a cat.
struct Cat {
  name: String,
  colour: CatColour,
}

/// Shows a description of the given cat.
fn match_on_black_cats(cat: &Cat) {
  match cat {
    Cat {
      name,
      colour: CatColour::Black,
    } => println!("{name} is a black cat."),
    Cat { name, colour: _ } => println!("{name} is not a black cat."),
  }
}

/// Write a note to a file returning a result.
fn write_to_file() -> std::io::Result<()> {
  use std::fs::File;
  use std::io::prelude::*;

  let mut file = File::create("files/my-note.txt")?;
  file.write_all(b"A simple note.")?;

  Ok(())
}

/// Write a list to file without returning a result.
fn write_to_file_without_result() {
  use std::fs::File;
  use std::io::prelude::*;

  let create_result = File::create("files/my-list.txt");
  match create_result {
    // The Ok file must be mutable
    Ok(mut file) => match file.write_all(b"- Buy coffee\n- Buy milk") {
      Ok(()) => println!("Writing list succeeded."),
      Err(err) => {
        println!("There was an error writing the list: {err}")
      }
    },
    Err(err) => println!("There was an error opening the list: {err}"),
  }
}

/// Represents different kinds of errors.
enum ErrorTypes {
  IoError(std::io::Error),
  FormatError(std::fmt::Error),
}

/// Represents a wrapper for errors.
struct ErrorWrapper {
  source: ErrorTypes,
  message: String,
}

impl From<std::io::Error> for ErrorWrapper {
  fn from(value: std::io::Error) -> Self {
    Self {
      source: ErrorTypes::IoError(value),
      message: "An IO error ocurred!".to_string(),
    }
  }
}

/// Write a note to a file returning a result with a possible error type.
fn write_to_file_wrapper() -> Result<(), ErrorWrapper> {
  use std::fs::File;
  use std::io::prelude::*;

  let mut file = File::create("files/my-text.txt")?;
  file.write_all(b"A sample text \ngoes over here.")?;

  Ok(())
}

fn main() {
  println!("********** Some or None **********");

  some_or_none(&Some("ABC"));
  some_or_none(None);

  println!("********** What integer is **********");

  [1, 2, 3, 4, 5, 8, 10, 0, -1, 12, 14]
    .into_iter()
    .for_each(|value| what_type_of_integer_is_this(value));

  println!("********** Destructure tuple **********");

  let tuple: (i32, i32, i32) = (-2, 5, 14);
  destructure_tuple(&tuple);

  println!("********** Matching with guard **********");

  [-6, 0, 1, 3, 6, 10, 7, -4, 20, 1, 8, 12, 15]
    .into_iter()
    .for_each(|value| match_with_guard(value));

  println!("********** Matching with enum type **********");

  [
    DistinctTypes::Name("Wattermellon".to_string()),
    DistinctTypes::Count(0),
    DistinctTypes::Count(-14),
    DistinctTypes::Name("Coconut".to_string()),
  ]
  .into_iter()
  .for_each(|value| match_enum_types(&value));

  println!("********** Destructing a struct **********");

  let black_cat = Cat {
    name: "Henry".to_string(),
    colour: CatColour::Black,
  };

  let cheshire_cat = Cat {
    name: "Penelope".to_string(),
    colour: CatColour::Cheshire,
  };

  match_on_black_cats(&black_cat);
  match_on_black_cats(&cheshire_cat);

  println!("********** Writing to a file **********");

  match write_to_file() {
    Ok(()) => println!("Writing note succeded."),
    Err(err) => println!("Failed to write note: {err}"),
  }

  write_to_file_without_result();

  match write_to_file_wrapper() {
    Ok(()) => println!("Writing note with wrapper succeded."),
    Err(err) => println!("Failed to write note with wrapper: {0}", err.message),
  }
}
