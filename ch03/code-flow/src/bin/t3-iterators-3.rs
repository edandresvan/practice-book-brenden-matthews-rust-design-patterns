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
  // The return object will be the data itself.
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.next.clone() {
      //
      Some(pointer) => {
        // Move the next attribute to the next item
        self.next = pointer.as_ref().borrow().next.clone();

        // Extract the refcell from the reference counter
        let list_item = Rc::try_unwrap(pointer).map(|refcell| refcell.into_inner());

        match list_item {
          Ok(item) => {
            // Extract the data from the refcell
            let a: Option<T> = Rc::try_unwrap(item.data)
              .map(|refcell| refcell.into_inner())
              .ok();
            a
          }
          Err(_) => {
            println!("Error ocurred. Returning a None value");
            None
          }
        }
      }
      // Here, return None because there are no more entries
      None => None,
    }
  }
}

fn main() {
  // Create a linked list of elements of type &str
  let mut dinosaurs: LinkedList<&str> = LinkedList::new("Tyrannosaurus Rex");

  // Append more items
  dinosaurs.append("Triceratops");
  dinosaurs.append("Velociraptor");
  dinosaurs.append("Stegosaurus");
  dinosaurs.append("Spinosaurus");

  println!("********** Show list with into_iter() **********");
  // Print the data in each pointer to an item (node). An item is of type reference counter.
  dinosaurs
    .into_iter()
    .for_each(|data| println!("data = {data}"));
}
