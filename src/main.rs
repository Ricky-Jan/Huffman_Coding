use std::collections::HashMap;
use std::fs;

pub const NULL_PTR: usize = usize::MAX;

#[derive(Clone)]
pub struct Node {
    pub char: char,
    pub freq: u32,
    pub left: usize,
    pub right: usize,
    pub parent: usize,
}
impl Node {
    pub fn new(char: char, freq: u32) -> Self {
        Self {
            char: char,
            freq: freq,
            left: NULL_PTR,
            right: NULL_PTR,
            parent: NULL_PTR,
        }
    }
}

pub struct Compressor {
    pub nodes: Vec<Node>,
    pub active_nodes: Vec<usize>,
    pub leaf_nodes: Vec<usize>,
    pub token_map: HashMap<char, usize>,
    pub code_map: HashMap<char, String>,
    pub root: usize,
}
impl Compressor {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            active_nodes: Vec::new(),
            leaf_nodes: Vec::new(),
            token_map: HashMap::new(),
            code_map: HashMap::new(),
            root: NULL_PTR,
        }
    }

    pub fn compress_text(&mut self, text: &str) -> String {
        self.count_token(text);
        self.build_tree();
        self.generate_code();
        self.encode(text)
    }

    fn count_token(&mut self, text: &str) {
        for char in text.chars() {
            if let Some(&node_idx) = self.token_map.get(&char) {
                self.nodes[node_idx].freq += 1;
            } else {
                let new_node = Node::new(char, 1);
                let new_idx = self.nodes.len();
                self.nodes.push(new_node);
                self.token_map.insert(char, new_idx);
                self.active_nodes.push(new_idx);
                self.leaf_nodes.push(new_idx);
            }
        }
    }

    fn build_tree(&mut self) {
        while self.active_nodes.len() >= 2 {
            let min_a = self.find_min_node();
            let min_b = self.find_min_node();
            let mut parent = Node::new('\0', self.nodes[min_a].freq + self.nodes[min_b].freq);
            parent.left = min_a;
            parent.right = min_b;

            let parent_idx = self.nodes.len();
            self.nodes.push(parent);
            self.active_nodes.push(parent_idx);

            self.nodes[min_a].parent = parent_idx;
            self.nodes[min_b].parent = parent_idx;
        }
        self.root = self.active_nodes[0];
    }

    fn find_min_node(&mut self) -> usize {
        let mut min_freq = u32::MAX;
        let mut min_idx = NULL_PTR;
        let mut pop_idx = 0;
        for (i, &node_idx) in self.active_nodes.iter().enumerate() {
            let freq = self.nodes[node_idx].freq;
            if freq < min_freq {
                min_freq = freq;
                min_idx = node_idx;
                pop_idx = i;
            }
        }
        self.active_nodes.remove(pop_idx);
        min_idx
    }

    fn generate_code(&mut self) {
        for &leaf_idx in self.leaf_nodes.iter() {
            let mut curr_code = String::new();
            let mut curr_idx = leaf_idx;
            while curr_idx != self.root {
                let parent = self.nodes[curr_idx].parent;
                if self.nodes[parent].left == curr_idx {
                    curr_code.push_str("0");
                } else {
                    curr_code.push_str("1");
                }
                curr_idx = parent;
            }
            let generated_code = curr_code.chars().rev().collect();
            let leaf_char = self.nodes[leaf_idx].char;
            self.code_map.insert(leaf_char, generated_code);
        }
    }

    fn encode(&mut self, text: &str) -> String {
        let mut encode = String::new();
        for char in text.chars() {
            let code = self.code_map.get(&char).unwrap();
            encode.push_str(code);
        }
        encode
    }

    pub fn decompress_bits(&mut self, bits: &str) -> String {
        let mut decode = String::new();
        let mut curr_idx = self.root;
        for bit in bits.chars() {
            if bit == '0' {
                curr_idx = self.nodes[curr_idx].left;
            } else {
                curr_idx = self.nodes[curr_idx].right;
            }
            if self.nodes[curr_idx].left == NULL_PTR {
                decode.push(self.nodes[curr_idx].char);
                curr_idx = self.root;
            }
        }
        decode
    }
}

fn main() {
    let text = fs::read_to_string(r".\src\input.txt").unwrap();
    let mut compressor = Compressor::new();
    let compressed_bits = compressor.compress_text(&text);
    let decompressd_text = compressor.decompress_bits(&compressed_bits);
    let ratio = compressed_bits.len() as f64 / (text.len() as f64 * 8.0);
    println!("{}", compressed_bits);
    println!("{}", decompressd_text);
    println!("Successfully compressed bit size by: {:.2}%", ratio * 100.0);
}
