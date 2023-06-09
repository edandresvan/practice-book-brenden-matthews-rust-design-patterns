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
    let mut next = self.head.clone();

    while next.as_ref().borrow().next.is_some() {
      let n: Rc<RefCell<ListItem<T>>> =
        next.as_ref().borrow().next.as_ref().unwrap().clone();
      next = n;
    }

    next.as_ref().borrow_mut().next = Some(Rc::new(RefCell::new(ListItem::new(t))));
  }

  fn iter(&self) -> Iter<T> {
    Iter {
      next: Some(self.head.clone()),
    }
  }

  fn iter_mut(&self) -> IterMut<T> {
    IterMut {
      next: Some(self.head.clone()),
    }
  }

  fn into_iter(self) -> IntoIter<T> {
    IntoIter {
      next: Some(self.head.clone()),
    }
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

struct Iter<T> {
  /// Pointer to next item in the list.
  next: Option<ListItemPtr<T>>,
}

struct IterMut<T> {
  /// Pointer to next item in the list.
  next: Option<ListItemPtr<T>>,
}

struct IntoIter<T> {
  /// Pointer to next item in the list.
  next: Option<ListItemPtr<T>>,
}

impl<T> Iterator for Iter<T> {
  type Item = ItemData<T>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.next.clone() {
      Some(pointer) => {
        // Make the next attribute points to the next item
        self.next = pointer.as_ref().borrow().next.clone();
        // Return the pointer to the data within the item
        Some(pointer.as_ref().borrow().data.clone())
      }
      // Here, return None because there are no more entries
      None => None,
    }
  }
}

impl<T> Iterator for IterMut<T> {
  type Item = ItemData<T>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.next.clone() {
      Some(pointer) => {
        // Make the next attribute points to the next item
        self.next = pointer.as_ref().borrow().next.clone();
        // Return the pointer to the data within the item
        Some(pointer.as_ref().borrow().data.clone())
      }
      // Here, return None because there are no more entries
      None => None,
    }
  }
}

impl<T> Iterator for IntoIter<T> {
  // The return object will be a reference counter containing the item data.
  type Item = ItemData<T>;
  fn next(&mut self) -> Option<Self::Item> {
    match self.next.clone() {
      // Make the next attribute points to the next item
      Some(pointer) => {
        self.next = pointer.as_ref().borrow().next.clone();
        // Return the pointer to the data within the item
        Some(pointer.as_ref().borrow().data.clone())
      }
      // Here, return None because there are no more entries
      None => None,
    }
  }
}

fn main() {

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
