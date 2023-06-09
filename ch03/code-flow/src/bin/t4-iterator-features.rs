use std::collections::LinkedList;

fn main() {
  println!("********** map() **********");

  let vec_i32: Vec<i32> = vec![1, 2, 3, 4];
  println!("{vec_i32:?}");
  let vec_string: Vec<String> = vec_i32.iter().map(|v| v.to_string()).collect();
  println!("{vec_string:?}");

  println!("********** list to i32 **********");

  let linked_list: LinkedList<i32> =
    vec_string.iter().flat_map(|v| v.parse::<i32>()).collect();
  println!("{linked_list:?}");

  println!("********** list with partition **********");

  let vec_mix: Vec<&str> = vec!["duck", "1", "2", "goose", "3", "4"];
  let (successes, failures): (Vec<_>, Vec<_>) = vec_mix
    .iter()
    .map(|v| v.parse::<i32>())
    .partition(Result::is_ok);

  println!("successes = {successes:?}");
  println!("failures = {failures:?}");

  let successes: Vec<i32> = successes.into_iter().map(Result::unwrap).collect();

  let failures: Vec<_> = failures.into_iter().map(Result::unwrap_err).collect();

  println!("successes = {successes:?}");
  println!("failures = {failures:?}");

  println!("********** list with enumerate **********");

  let popular_dog_breeds: Vec<&str> = vec![
    "Labrador",
    "French Bulldog",
    "Golden Retriever",
    "German Shepherd",
    "Poodle",
    "Bulldog",
    "Beagle",
    "Rottweiler",
    "Pointer",
    "Dachshund",
  ];
  println!("- Original list of breeds.");
  println!("{:?}", &popular_dog_breeds);

  // Get a list of breeds with a zero-based index.
  println!("- Show list of breeds with a zero-based index:");
  let ranked_breeds: Vec<(usize, &&str)> =
    popular_dog_breeds.iter().enumerate().collect();
  println!("{:?}", &ranked_breeds);

  // Get a list of breeds with a one-based index.
  println!("- Show list of breeds with a one-based index:");
  let ranked_breeds: Vec<(usize, &&str)> = popular_dog_breeds
    .iter()
    .enumerate()
    .map(|(index, breed)| (index + 1, breed))
    .collect();
  println!("{:?}", &ranked_breeds);

  // Get a list of breeds with a one-based index in reverse order.
  println!("- Show list of breeds with a one-based index in reverse order:");
  let ranked_breeds: Vec<(usize, &&str)> = popular_dog_breeds
    .iter()
    .enumerate()
    .map(|(index, breed)| (index + 1, breed))
    .rev()
    .collect();
  println!("{:?}", &ranked_breeds);
  
}
