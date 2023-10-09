use ::std::cmp::Reverse;
use ::std::collections::{BinaryHeap, HashMap};
struct CmpFile {
    header: Vec<u8>,
    data: String,
}

impl CmpFile {
    fn new(data: String) -> Self {
        Self {
            header: vec![0],
            data,
        }
    }

    fn char_freq(self) -> BinaryHeap<Reverse<(i32, char)>> {
        let mut counts = HashMap::<char, i32>::new();
        let char_vec: Vec<char> = self.data.to_lowercase().chars().collect();

        for chr in char_vec {
            *counts.entry(chr).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();

        for (chr, freq) in counts {
            heap.push(Reverse((freq, chr)));
        }

        return heap;
    }
}

fn main() {
    let file = CmpFile::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit.".into());

    dbg!(file.char_freq());
}
