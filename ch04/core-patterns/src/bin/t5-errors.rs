use std::path::Path;

/// Represents the errors in this application.
#[derive(Debug)]
enum MyError {
  /// Input-Output error.
  Io(std::io::Error),
  /// Error when the line number is invalid.
  BadLineArgument(usize),
}

impl From<std::io::Error> for MyError {
  fn from(value: std::io::Error) -> Self {
    Self::Io(value)
  }
}

fn read_nth_line(
  path: &Path,
  line_number: usize,
) -> Result<String, MyError> {
  use std::fs::File;
  use std::io::{BufRead, BufReader};

  // The minimal line number to read should be 1
  if line_number < 1 {
    return Err(MyError::BadLineArgument(0));
  }

  // Open the file
  let file: File = File::open(path)?;

  // Get a reader of lines
  let mut lines_reader = BufReader::new(file).lines();

  let line_result = lines_reader
    // In this lines collection the first line is at index zero
    .nth(line_number - 1)
    // Convert the io error into my error
    .map(|result| result.map_err(|err| err.into()))
    // Here, we have Option<Result<String, MyError>>, so try to unwrap the Option Some. Otherwise, a None value was returned, and a custom error should be returned
    .unwrap_or_else(|| Err(MyError::BadLineArgument(line_number)));

  line_result
}

fn read_nth_line_v2(
  path: &Path,
  line_number: usize,
) -> Result<String, MyError> {
  use std::fs::File;
  use std::io::{BufRead, BufReader};

  // The minimal line number to read should be 1
  if line_number < 1 {
    return Err(MyError::BadLineArgument(0));
  }

  // Open the file
  let file: File = File::open(path)?;

  // Get a reader of lines
  let mut lines_reader = BufReader::new(file).lines();

  // More verbose ...

  // Try to read the requested line
  let lines_result: Option<Result<String, std::io::Error>> =
    lines_reader.nth(line_number - 1);

  let single_line_result: Result<String, MyError> = match lines_result {
    // The line number is within range and a result was returned
    Some(value) => {
      // Now, return the string value or an IO error converted to MyError IO
      value.map_err(|err| err.into())
    }
    // Here, the line number was not valid, so return MyError Bad Line
    None => return Err(MyError::BadLineArgument(line_number)),
  };

  single_line_result
}

fn main() -> Result<(), MyError> {
  println!("********** Reading file **********");

  let path: &Path = Path::new("files/poem.txt");
  let line_number: usize = 12;
  println!(
    "The line number {0} contains: {1:?}",
    line_number,
    read_nth_line(path, line_number)?
  );

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_can_read_cargotoml() {
    let third_line = read_nth_line(Path::new("Cargo.toml"), 3)
      .expect("unable to read third line from Cargo.toml");
    assert_eq!("version = \"0.1.0\"", third_line);
  }

  #[test]
  fn test_not_a_file() {
    let err =
      read_nth_line(Path::new("not-a-file"), 1).expect_err("file should not exist");
    assert!(matches!(err, MyError::Io(_)));
  }

  #[test]
  fn test_bad_arg_0() {
    let err = read_nth_line(Path::new("Cargo.toml"), 0).expect_err("0th line is invalid");
    assert!(matches!(err, MyError::BadLineArgument(0)));
  }

  #[test]
  fn test_bad_arg_too_large() {
    let err =
      read_nth_line(Path::new("Cargo.toml"), 500).expect_err("500th line is invalid");
    assert!(matches!(err, MyError::BadLineArgument(500)));
  }
}
