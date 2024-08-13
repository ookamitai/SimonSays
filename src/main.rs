#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
use rand::prelude::*;
use std::{fs, usize};

trait MarkovChainNodeProperties: Default + std::fmt::Debug + std::cmp::Eq + Hash + Copy + std::fmt::Display {}
impl<T> MarkovChainNodeProperties for T where T: Default + std::fmt::Debug + std::cmp::Eq + Hash + Copy + std::fmt::Display {}



#[derive(Debug)]
struct MarkovChain<T: MarkovChainNodeProperties> {
    nodes: Vec<MarkovChainNode<T>>
}

#[allow(unused_variables)]
impl<T: MarkovChainNodeProperties> MarkovChain<T> {
    fn new() -> Self {
        MarkovChain {
            nodes: Vec::new(),
        }
    }

    fn add_node(&mut self, label: T) {
        for i in self.nodes.iter() {
            if i.label == label {
                return;
            }
        }
        self.nodes.push(MarkovChainNode::new(label));
    }

    fn connect_node(&mut self, from: T, to: T) {
        let mut from_index = -1; let mut to_index = -1;

        for (i, n) in self.nodes.iter().enumerate() {
            if n.label == from {
                from_index = i as i32;
            }

            if n.label == to {
                to_index = i as i32;
            }
        }

        if from_index == -1 || to_index == -1 {
            return;
        }

        if let Some(v) = self.nodes[from_index as usize].neigbors.get_mut(&to) {
            *v += 1;
        } else {
            self.nodes[from_index as usize].neigbors.insert(to, 1);
        }
    }

    fn convert_to_weights(&mut self) {
        for i in &mut self.nodes {
            let mut sum: u32 = 0;
            for (_, v) in &i.neigbors {
                sum += *v as u32;
            }
            
            for (k, v) in &i.neigbors {
                i.probability.insert(*k, *v as f64 / sum as f64);
            }
        }
    }

    fn infer(&self, steps: usize, starting: T) { 
        let mut steps = steps;
        let mut starting = starting;
        let mut rd;
        while steps > 0 {
            if let Some(node) = self.nodes.iter().find(|n| n.label == starting) {
                print!("{} ", starting);
                let mut sum = 0;
                for (_, cnt) in node.neigbors.iter() {
                    sum += cnt;
                }
                if sum == 0 { return; }
                rd = rand::thread_rng().gen_range(1..=sum) as i64;
                for (nd, cnt) in node.neigbors.iter() {
                    rd -= *cnt as i64;
                    if rd <= 0 {
                        starting = *nd;
                        break;
                    }
                }
            } else { return; } 

            steps -= 1;
        }
    }


    fn print_weights(&self) {
        for node in self.nodes.iter() {
            println!("Node {:?}: {:?}", node.label, node.probability);
        }
    }
}

#[derive(Debug)]
struct MarkovChainNode<T: MarkovChainNodeProperties> {
    label: T,
    neigbors: HashMap<T, u32>,
    probability: HashMap<T, f64>,
}

impl<T: MarkovChainNodeProperties> MarkovChainNode<T> {
    fn new(label: T) -> Self {
        MarkovChainNode {
            label: label,
            neigbors: HashMap::new(),
            probability: HashMap::new(),
        }
    }
}

fn split_by(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).map(|s| s.trim()).filter(|s|!s.is_empty()).collect()
}

fn regularize_text(text: &String) -> String {
    let references = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 -'.";
    let mut result = String::new();

    for c in text.chars().into_iter() {
        if c == '\n' {
            result.push(' ');
            continue;
        }
        if references.contains(c) {
            result.push(c);
        }
    }

    result
}


fn main() {
    let contents = fs::read_to_string("train.txt").expect("Oh no file reading error!");
    let contents = regularize_text(&contents);
    let words: Vec<&str> = contents.split(' ').collect();
    let mut chain = MarkovChain::new();

    chain.add_node(words[0]);
    for index in 1..words.len() {
        chain.add_node(words[index]);
        chain.connect_node(words[index - 1], words[index]);
    }

    chain.convert_to_weights();

    // chain.print_weights();
    println!();

    chain.infer(50, "A");
}
