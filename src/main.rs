use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use std::{cmp::Ordering, str::FromStr};
use itertools::Itertools;

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let sum = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|s| !s.is_empty())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let chunk: [String; 2] = chunk.take(2).collect_vec().try_into().unwrap();
            chunk.map(|l| Node::<i32>::from_str(l.as_str()).unwrap())
        })
        .enumerate()
        .filter_map(
            |(i, signals)| match signals_are_ordered(&signals[0], &signals[1]) {
                Ordering::Less => Some(i),
                _ => None,
            },
        )
        .sum::<usize>();
    println!("part 1: {}", sum);
}

enum Node<T> {
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
                other => {
                    if let None = current_string {
                        current_string = Some(String::new())
                    }
                    if let Some(string) = current_string.as_mut() {
                        string.push(other);
                    }
                    continue;
                }
            }
            if let Some(string) = current_string {
                let t = T::from_str(string.trim())?;
                if let Node::List(list) = stack.last_mut().unwrap() {
                    list.push(Node::Single(t));
                }
            }
            current_string = None;
        }
        panic!("Invalid string")
    }
}

fn signals_are_ordered<T: Ord + Copy>(left: &Node<T>, right: &Node<T>) -> Ordering {
    match left {
        Node::Single(left_single) => match right {
            Node::Single(right_single) => left_single.cmp(right_single),
            Node::List(_) => {
                signals_are_ordered(&Node::List(vec![Node::Single(*left_single)]), right)
            }
        },
        Node::List(left_list) => match right {
            Node::Single(right_single) => {
                signals_are_ordered(left, &Node::List(vec![Node::Single(*right_single)]))
            }
            Node::List(right_list) => {
                for i in 0..left_list.len().min(right_list.len()) {
                    match signals_are_ordered(&left_list[i], &right_list[i]) {
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
