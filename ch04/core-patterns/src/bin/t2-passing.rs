/// Takes the given string by value and gets a new string with its characters in reverse order.
fn reverse(string_value: String) -> String {
  let mut string_chars: Vec<char> = Vec::from_iter(string_value.chars());
  // Reverse in place
  string_chars.reverse();
  String::from_iter(string_chars.iter())
}

fn reverse_inplace(string_value: &mut String) {
  let mut string_chars: Vec<char> = Vec::from_iter(string_value.chars());
  string_chars.reverse();
  string_value.clear();
  string_chars
    .into_iter()
    .for_each(|char| string_value.push(char));
}

fn main() {
  let string_1 = "gfedcba".to_string();
  assert_eq!("abcdefg", reverse(string_1));
  // println!("{string_1}");

  let mut string_2 = "gfedcba".to_string();
  reverse_inplace(&mut string_2);
  assert_eq!("abcdefg", string_2);
}
