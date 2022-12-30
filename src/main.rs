use std::{
    fs::File,
    io::{self, BufRead},
    mem::discriminant,
    path::Path,
    str::FromStr,
};

use itertools::{Chunk, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

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
            chunk.map(|l| Node::<u8>::from_str(l.as_str()).unwrap())
        })
        .enumerate()
        .filter_map(
            |(i, signals)| match signals_are_ordered(&signals[0], &signals[1]) {
                true => Some(i),
                false => None,
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
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\[(.*)\]$").unwrap();
        }
        Ok(match RE.captures(s) {
            Some(cap) => {
                let mat = cap.get(1).unwrap();
                let mut v = Vec::new();
                for s in mat.as_str().split(',') {
                    v.push(Node::from_str(s)?);
                }
                Node::List(v)
            }
            None => Node::Single(T::from_str(s)?),
        })
    }
}

fn signals_are_ordered<T: PartialOrd>(first: &Node<T>, second: &Node<T>) -> bool {
    
    let (first, second) = match first {
        Node::Single(first_single) => match second {
            Node::Single(second_single) => return first_single <= second_single,
            Node::List(_) => (, second),
        },
        Node::List(first) => match second {
            Node::Single(second) => (first, vec![second]),
            Node::List(second) => (first, second),
        },
    }
}
