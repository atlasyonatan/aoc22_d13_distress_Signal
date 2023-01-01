use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum Node<T> {
    Single(T),
    List(Vec<Node<T>>),
}

impl<T> Ord for Node<T>
where
    T: Copy + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Node::Single(left_single) => match other {
                Node::Single(right_single) => left_single.cmp(right_single),
                Node::List(_) => Node::List(vec![Node::Single(*left_single)]).cmp(other),
            },
            Node::List(left_list) => match other {
                Node::Single(right_single) => {
                    self.cmp(&Node::List(vec![Node::Single(*right_single)]))
                }
                Node::List(right_list) => {
                    for i in 0..left_list.len().min(right_list.len()) {
                        match left_list[i].cmp(&right_list[i]) {
                            Ordering::Equal => {
                                continue;
                            }
                            ordering => {
                                return ordering;
                            }
                        }
                    }
                    left_list.len().cmp(&right_list.len())
                }
            },
        }
    }
}

impl<T> PartialOrd for Node<T>
where
    T: Copy + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: FromStr> FromStr for Node<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Node<T>> = Vec::new();
        let mut current_string: Option<String> = None;
        for char in s.chars() {
            match char {
                ']' | ',' => {
                    if let Some(string) = current_string {
                        let t = T::from_str(string.trim())?;
                        if let Node::List(list) = stack.last_mut().unwrap() {
                            list.push(Node::Single(t));
                        }
                        current_string = None;
                    }
                }
                _ => (),
            };
            match char {
                '[' => {
                    stack.push(Node::List(Vec::new()));
                }
                ']' => {
                    if stack.len() > 1 {
                        let node = stack.pop().unwrap();
                        if let Node::List(list) = stack.last_mut().unwrap() {
                            list.push(node);
                        }
                    } else if stack.len() == 1 {
                        return Ok(stack.pop().unwrap());
                    }
                }
                ',' => (),
                other_char => {
                    if let None = current_string {
                        current_string = Some(String::new())
                    }
                    if let Some(string) = current_string.as_mut() {
                        string.push(other_char);
                    }
                }
            }
        }
        panic!("Invalid string")
    }
}
