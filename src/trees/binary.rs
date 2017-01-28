use std::cmp::Ordering;


pub type Edge<T> = Option<Box<Node<T>>>;


#[derive(PartialEq)]
pub struct Node<T: PartialOrd + Ord> {
  data: T,
  left: Edge<T>,
  right: Edge<T>
}


#[derive(PartialEq)]
pub struct Tree<T: PartialOrd + Ord> {
  root: Edge<T>
}


pub fn compute_ordering<T>(current: &Edge<T>, target: &T) -> Ordering
    where T: PartialOrd + Ord {
  match current {
    &Some(ref node) => target.cmp(&node.data),
    &None => Ordering::Equal
  }
}


pub enum TraversalOrder {
  Increasing,
  Decreasing
}


pub struct TreeIterator<'a, T: 'a + PartialOrd + Ord> {
  tree: &'a Tree<T>,
  path: Option<Vec<&'a Edge<T>>>
}


impl<'a, T: 'a + PartialOrd + Ord> IntoIterator for &'a Tree<T> {
  type Item = &'a Node<T>;
  type IntoIter = TreeIterator<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    TreeIterator { tree: self, path: None }
  }
}


impl<'a, T: 'a + PartialOrd + Ord> TreeIterator<'a, T> {
  fn initialize_path(&mut self, order: &TraversalOrder) {
    let mut path = Vec::new();
    TreeIterator::populate_path(&mut path, &self.tree.root, order);
    self.path = Some(path);
  }

  fn populate_path(path: &mut Vec<&'a Edge<T>>, root: &'a Edge<T>, order: &TraversalOrder) {
    let mut current = root;
    loop {
      current = match current {
        &Some(ref node) => {
          path.push(current);
          match *order {
            TraversalOrder::Increasing => &node.left,
            TraversalOrder::Decreasing => &node.right
          }
        },
        &None => break
      };
    }
  }

  fn iter_next(&mut self, order: &TraversalOrder) -> Option<&'a Node<T>> {
    match self.path {
      None => {
        self.initialize_path(order);
        self.iter_next(order)
      },
      Some(ref mut path) => {
        let next = path.pop();
        match next {
          Some(edge) => {
            match edge {
              &Some(ref node) => {
                let subtree = match *order {
                  TraversalOrder::Increasing => &node.right,
                  TraversalOrder::Decreasing => &node.left
                };
                match subtree {
                  &None => return Some(node),
                  subtree => {
                    TreeIterator::populate_path(path, subtree, order);
                    return Some(node);
                  }
                }
              },
              &None => panic!("Edge in iterator path is None")
            }
          },
          None => return None
        }
      }
    }
  }
}


impl<'a, T: 'a + PartialOrd + Ord> DoubleEndedIterator for TreeIterator<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.iter_next(&TraversalOrder::Decreasing)
  }
}


impl<'a, T: 'a + PartialOrd + Ord> Iterator for TreeIterator<'a, T> {
  type Item = &'a Node<T>;

  fn next(&mut self) -> Option<Self::Item> {
    self.iter_next(&TraversalOrder::Increasing)
  }
}


impl<T> Tree<T>
        where T: PartialOrd + Ord {
  fn insert(&mut self, data: T) {
    match self.root {
      Some(_) => {
        match self.find_mut(&data) {
          &mut Some(_) => return,
          location => {
            let edge = Some(Box::new(
              Node { data: data, left: None, right: None }));
            *location = edge
          }
        };
      },
      None => {
        self.root = Some(Box::new(
          Node { data: data, left: None, right: None }));
      }
    }
  }

  fn delete(&mut self, data: &T) {
    match self.find_mut(&data) {
      &mut None => return,
      edge => {
        let mut node = edge.take().unwrap();
        match (node.left.take(), node.right.take()) {
          (Some(left), None) => *edge = Some(left),
          (None, Some(right)) => *edge = Some(right),
          (None, None) => return,
          (mut left, right) => {
            let mut replacement_node = {
              let mut current = &mut left;

              loop {
                match {current} {
                  &mut Some(ref mut node) if node.right.is_some() =>
                    current = &mut node.right,
                  edge => {
                    current = edge;
                    break;
                  }
                }
              }

              let mut replacement_node = current.take().unwrap();
              *current = replacement_node.left.take();

              replacement_node
            };

            replacement_node.left = left;
            replacement_node.right = right;

            *edge = Some(replacement_node);
          }
        }
      }
    }
  }

  fn find<'a>(&'a self, data: &T) -> &Edge<T> {
    let mut current = &self.root;
    loop {
      current = match (compute_ordering(&current, &data), current) {
        (Ordering::Less, &Some(ref node)) => &node.left,
        (Ordering::Greater, &Some(ref node)) => &node.right,
        (Ordering::Equal, edge) => return edge,
        (_, &None) => panic!("Unexpected None edge while finding value")
      };
    }
  }

  fn find_mut<'a>(&'a mut self, data: &T) -> &'a mut Edge<T> {
    let mut current = &mut self.root;
    loop {
      current = match (compute_ordering(&current, &data), current) {
        (Ordering::Less, &mut Some(ref mut node)) => &mut node.left,
        (Ordering::Greater, &mut Some(ref mut node)) => &mut node.right,
        (Ordering::Equal, edge) => return edge,
        (_, &mut None) => panic!("Unexpected None edge while finding value")
      };
    }
  }
}


#[cfg(test)]
mod test {
  use super::*;

  #[derive(PartialOrd, Ord, PartialEq, Eq)]
  struct Data {
    value: u32
  }

  fn assert_edge<'a, T>(edge: &'a Edge<T>, data: T) -> &'a Box<Node<T>>
      where T: PartialOrd + Ord {
    match edge {
      &Some(ref node) if node.data == data => node,
      &Some(_) => panic!("node data does not match"),
      &None => panic!("edge is none")
    }
  }

  fn make_edge(data: u32, left: Edge<Data>, right: Edge<Data>) -> Edge<Data> {
    Some(Box::new(Node { data: Data { value: data }, left: left, right: right }))
  }

  fn make_tree() -> Tree<Data> {
    Tree {
      root: make_edge(10,
        make_edge(5,
          make_edge(4, None, None),
          make_edge(6,
            None,
            make_edge(7, None, None))),
        make_edge(15,
          make_edge(12,
            make_edge(11, None, None),
            make_edge(14, None, None)),
          make_edge(16, None, None)))
    }
  }

  #[test]
  fn test_find() {
    let tree = make_tree();

    assert_edge(tree.find(&Data { value: 7 }), Data { value: 7 });
    assert_edge(tree.find(&Data { value: 10 }), Data { value: 10 });
    assert_edge(tree.find(&Data { value: 12 }), Data { value: 12 });

    assert!(tree.find(&Data { value: 30 }).is_none());
  }

  #[test]
  fn test_insert() {
    let mut tree = Tree { root: None };

    tree.insert(Data { value: 10 });
    tree.insert(Data { value: 5 });
    tree.insert(Data { value: 15 });
    tree.insert(Data { value: 10 });
    tree.insert(Data { value: 16 });
    tree.insert(Data { value: 3 });

    assert!(tree == Tree {
      root: make_edge(10,
        make_edge(5,
          make_edge(3, None, None),
          None),
        make_edge(15,
          None,
          make_edge(16, None, None))) });
  }

  #[test]
  fn test_delete() {
    let mut tree = make_tree();

    tree.delete(&Data{ value: 14 }); // leaf node
    tree.delete(&Data{ value: 12 }); // 1 left child
    tree.delete(&Data{ value: 6 });  // 1 right child
    tree.delete(&Data{ value: 10 }); // 2 children

    let final_tree = Tree {
      root: make_edge(7,
        make_edge(5,
          make_edge(4, None, None),
          None),
        make_edge(15,
          make_edge(11, None, None),
          make_edge(16, None, None)))
    };

    assert!(tree == final_tree);
  }

  #[test]
  fn test_iterator() {
    let tree = make_tree();

    let values: Vec<u32> = tree.into_iter()
      .map(|n| n.data.value)
      .collect();
    assert!(values == [4, 5, 6, 7, 10, 11, 12, 14, 15, 16]);

    let values: Vec<u32> = tree.into_iter()
      .rev()
      .map(|n| n.data.value)
      .collect();
    assert!(values == [16, 15, 14, 12, 11, 10, 7, 6, 5, 4]);

    let empty_tree: Tree<u32> = Tree { root: None };
    let values: Vec<&Node<u32>> = empty_tree.into_iter()
      .collect();
    assert!(values.len() == 0);
  }
}
