use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

/// Represents the data contained in a node.
type ItemData<T> = Rc<RefCell<T>>;
/// Represents a pointer to a node.s
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;

/// Represents a node in a list.
///
/// Note: This list structure is not thread-safe.
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
}

impl<T> LinkedList<T> {
  fn new(t: T) -> Self {
    Self {
      head: Rc::new(RefCell::new(ListItem::new(t))),
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
      data: None,
      phantom: PhantomData,
    }
  }

  fn iter_mut(&self) -> IterMut<T> {
    IterMut {
      next: Some(self.head.clone()),
      data: None,
      phantom: PhantomData,
    }
  }

  fn into_iter(self) -> IntoIter<T> {
    IntoIter {
      next: Some(self.head.clone()),
    }
  }
}

struct Iter<'a, T> {
  /// Pointer to next item in the list.
  next: Option<ListItemPtr<T>>,
  /// Copy of the pointer to the data.
  data: Option<ItemData<T>>,
  /// Field to capture the lifetime 'a in the struct.
  phantom: PhantomData<&'a T>,
}

struct IterMut<'a, T> {
  /// Pointer to next item in the list.
  next: Option<ListItemPtr<T>>,
  /// Copy of the pointer to the data.
  data: Option<ItemData<T>>,
  /// Field to capture the lifetime 'a in the struct.
  phantom: PhantomData<&'a T>,
}

struct IntoIter<T> {
  /// Pointer to next item in the list.
  next: Option<ListItemPtr<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.next.clone() {
      Some(pointer) => {
        // Make the next attribute points to the next item
        self.next = pointer.as_ref().borrow().next.clone();
        // Extract the data of this pointer
        self.data = Some(pointer.as_ref().borrow().data.clone());
        // Return the data as a reference to a deferenced pointer
        unsafe { Some(&*self.data.as_ref().unwrap().as_ptr()) }
      }
      // Here, return None because there are no more entries
      None => None,
    }
  }
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.next.clone() {
      Some(pointer) => {
        // Make the next attribute points to the next item
        self.next = pointer.as_ref().borrow().next.clone();
        // Extract the data of this pointer
        self.data = Some(pointer.as_ref().borrow().data.clone());
        // Return the data as a reference to a deferenced pointer
        unsafe { Some(&mut *self.data.as_ref().unwrap().as_ptr()) }
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

impl<'a, T> IntoIterator for &'a LinkedList<T> {
  type IntoIter = Iter<'a, T>;
  type Item = &'a T;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
  type IntoIter = IterMut<'a, T>;
  type Item = &'a mut T;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<'a, T> IntoIterator for LinkedList<T> {
  type IntoIter = IntoIter<T>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    self.into_iter()
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

  println!("********** Show list with for loop **********");

  for data in &dinosaurs {
    println!("data = {data}");
  }

  println!("********** Show list with iter() **********");

  dinosaurs.iter().for_each(|data| println!("data = {data}"));

  println!("********** Show list with iter_mut() **********");

  dinosaurs
    .iter_mut()
    .for_each(|data| println!("data = {data}"));

  println!("********** Show list with into_iter() **********");

  // Print the data in each pointer to an item (node). An item is of type reference counter.
  dinosaurs
    .into_iter()
    .for_each(|data| println!("data = {data}"));
}
