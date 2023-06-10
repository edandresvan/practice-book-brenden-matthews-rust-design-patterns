use std::fmt::Debug;

#[derive(Clone, Debug)]
/// Represents a recursive item in a list.
struct ListItem<T>
where
  T: Clone + Debug,
{
  data: Box<T>,
  next: Option<Box<&ListItem<T>>>,
}

/// Represents a recursive item in a list as an enumeration.
#[derive(Debug)]
enum Recursive<T> {
  Next(Box<Recursive<T>>),
  Boxed(Box<T>),
  Optional(Option<T>),
}

/// Represents the next item in a list.
#[derive(Clone, Debug)]
enum NextNode<T> {
  Next(Box<ListNode<T>>),
  End,
}

/// Represents an item in the list.
#[derive(Clone, Debug)]
struct ListNode<T> {
  data: Box<T>,
  next: NextNode<T>,
}

fn main() {
  println!("********** ListItem **********");

  let mut item_1: ListItem<&str> = ListItem {
    data: Box::new(&"plum"),
    next: None,
  };

  let mut item_2: ListItem<&str> = ListItem {
    data: Box::new(&"blackberry"),
    next: None,
  };

  let mut item_3: ListItem<&str> = ListItem {
    data: Box::new(&"blueberry"),
    next: None,
  };

  item_1.next = Some(Box::new(&item_2));
  item_2.next = Some(Box::new(&item_3));
  item_3.next = None;

  let items = vec![&item_1, &item_2, &item_3];
  items.iter().for_each(|item| println!("{item:?}"));

  println!("********** NextNode and ListNode **********");

  let item_1: Recursive<&str> = Recursive::Boxed(Box::new("mango"));

  let item_2: Recursive<&str> = Recursive::Boxed(Box::new("orange"));

  let item_3: Recursive<&str> = Recursive::Boxed(Box::new("banana"));

  let items = vec![&item_1, &item_2, &item_3];
  items.iter().for_each(|item| println!("{item:?}"));

  println!("********** NextNode and ListNode **********");

  let mut item_1: ListNode<&str> = ListNode {
    data: Box::new("lemoon"),
    next: NextNode::End,
  };

  let mut item_2: ListNode<&str> = ListNode {
    data: Box::new("lime"),
    next: NextNode::End,
  };

  let mut item_3: ListNode<&str> = ListNode {
    data: Box::new("kiwi"),
    next: NextNode::End,
  };

  item_1.next = NextNode::Next(Box::new(item_2.clone()));
  item_2.next = NextNode::Next(Box::new(item_3.clone()));
  item_3.next = NextNode::End;

  let items = vec![&item_1, &item_2, &item_3];
  items.iter().for_each(|item| println!("{item:?}"));
}
