use std::str::FromStr;

#[derive(Debug)]
pub enum Node<T> {
    Single(T),
    List(Vec<Node<T>>),
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
