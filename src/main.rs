use ::std::cmp::Reverse;
use ::std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
extern crate huffman_coding;

struct CmpFile {
    header: Vec<u8>,
    data: String,
    freq_dict: Option<BinaryHeap<Reverse<(i32, char)>>>,
}

impl CmpFile {
    fn new(data: String) -> Self {
        Self {
            header: vec![0],
            data,
            freq_dict: None,
        }
    }

    fn char_freq(&mut self) {
        let mut counts = HashMap::<char, i32>::new();
        let char_vec: Vec<char> = self.data.to_lowercase().chars().collect();

        for chr in char_vec {
            *counts.entry(chr).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<Reverse<(i32, char)>> = BinaryHeap::new();

        for (chr, freq) in counts {
            heap.push(Reverse((freq, chr)));
        }

        self.freq_dict = Some(heap);
    }

    fn huffman_code(self) -> Vec<usize> {
        let mut code_map = HashMap::<char, u32>::new();
        let mut i = 0;

        let mut f_vec: HashMap<char, usize> = self
            .freq_dict
            .unwrap()
            .into_sorted_vec()
            .iter()
            .enumerate()
            .map(|(idx, x)| (x.0 .1, idx))
            .collect::<HashMap<_, _>>();

        let mut res = Vec::new();

        for ch in self.data.to_lowercase().chars() {
            res.push(*f_vec.get(&ch).unwrap());
        }

        res
    }
}

fn main() {
    let mut file = CmpFile::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit.".into());

    dbg!(file.char_freq());
    file.huffman_code();
}
