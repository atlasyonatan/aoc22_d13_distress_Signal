use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use itertools::Itertools;
use std::cmp::Ordering;
use std::str::FromStr;

pub mod node;
use node::Node;

fn main() {
    let path = Path::new("../input.txt");
    //part 1
    {
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
            .filter_map(|(i, signals)| (signals[0] < signals[1]).then(|| i + 1))
            .sum::<usize>();
        println!("part 1: {}", sum);
    }
    //part 2
    {
        let mut packets = ["[[2]]", "[[6]]"]
            .map(Node::<u8>::from_str)
            .map(|r| r.unwrap());
        packets.sort();
        let packets = packets;
        let positions = packets
            .iter()
            .map(|packet| {
                let file = File::open(path).unwrap();
                io::BufReader::new(file)
                    .lines()
                    .map(|l| l.unwrap())
                    .filter(|s| !s.is_empty())
                    .map(|l| Node::<u8>::from_str(l.as_str()).unwrap())
                    .filter(|signal| signal < packet)
                    .count()
            })
            .enumerate()
            .map(|(i, position)| position + i + 1)
            .collect_vec();
        println!("positions: {:?}", positions);
        println!(
            "part 2: {}",
            positions.iter().fold(1, |acc, item| acc * item)
        );
    }
}
