use std::cell::RefCell;
use std::rc::Rc;

/// Represents the data contained in a node.
type ItemData<T> = Rc<RefCell<T>>;
/// Represents a pointer to a node.s
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;

/// Represents a node in a list.
struct ListItem<T> {
  data: ItemData<T>,
  next: Option<ListItemPtr<T>>,
}

impl<T> ListItem<T> {
  fn new(t: T) -> Self {
    Self {
      data: Rc::new(RefCell::new(t)),
      next: None,
    }
  }
}

/// Represents a linked list data structure.
struct LinkedList<T> {
  /// Head node of the List.
  head: ListItemPtr<T>,
  /// Pointer to the current position of the iterator.
  current_iteration: Option<ListItemPtr<T>>,
}

impl<T> LinkedList<T> {
  fn new(t: T) -> Self {
    Self {
      head: Rc::new(RefCell::new(ListItem::new(t))),
      current_iteration: None,
    }
  }

  fn append(
    &mut self,
    t: T,
  ) {
    self
      .last()
      .expect("List was empty, but it should never be.")
      .as_ref()
      .borrow_mut()
      .next = Some(Rc::new(RefCell::new(ListItem::new(t))));
  }
}

impl<T> Iterator for LinkedList<T> {
  type Item = ListItemPtr<T>;

  fn next(&mut self) -> Option<Self::Item> {
    match &self.current_iteration.clone() {
      Some(pointer) => {
        // Move the iterator to the next node
        self.current_iteration = pointer.borrow().next.clone()
      }
      None => {
        // Here, the iterator is not initialized. So move the iterator to the head node
        self.current_iteration = Some(self.head.clone())
      }
    }
    self.current_iteration.clone()
  }
}

fn main() {
  println!("********** List with one item **********");

  // Create a linked list of elements of type &str
  let dinosaurs: LinkedList<&str> = LinkedList::new("Tyrannosaurus Rex");
  // Retrieve a pointer to the last item (node). An item is of type reference counter.
  let last_item: Rc<RefCell<ListItem<&str>>> =
    dinosaurs.last().expect("couldn't get the last item.");

  println!("last_item = {}", last_item.borrow().data.borrow());

  println!("********** List with many items **********");

  // Create a linked list of elements of type &str
  let mut dinosaurs: LinkedList<&str> = LinkedList::new("Tyrannosaurus Rex");

  // Append more items
  dinosaurs.append("Triceratops");
  dinosaurs.append("Velociraptor");
  dinosaurs.append("Stegosaurus");
  dinosaurs.append("Spinosaurus");

  // Print the data in each pointer to an item (node). An item is of type reference counter.
  dinosaurs.for_each(|pointer| println!("data = {}", pointer.borrow().data.borrow()));
}
