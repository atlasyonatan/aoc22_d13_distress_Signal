use itertools::Itertools;
use std::str::FromStr;
use std::{cmp::Ordering};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub mod node;
use node::Node;

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let sum = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|s| !s.is_empty())
        .chunks(2)
        .into_iter()
        .map(|chunk| -> [String; 2] { chunk.take(2).collect_vec().try_into().unwrap() })
        .map(|pair| pair.map(|l| Node::<u8>::from_str(l.as_str()).unwrap()))
        .enumerate()
        .filter_map(
            |(i, signals)| match signals_are_ordered(&signals[0], &signals[1]) {
                Ordering::Less => Some(i + 1),
                _ => None,
            },
        )
        .sum::<usize>();
    println!("part 1: {}", sum);
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
