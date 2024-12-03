use rand::{Rng, thread_rng};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short='s', long="step_by_step", help="Enable step-by-step mode")]
    step_by_step: bool,
}

struct SegmentTree {
    node: usize,
    data: Vec<i32>,
    default: i32,
}

impl SegmentTree {
    fn new(size: usize, default: i32) -> Self {
        let node = size.next_power_of_two()-1;
        SegmentTree {
            node,
            data: vec![default; 2*node+1],
            default,
        }
    }

    fn build(&mut self, vals: &[i32]) { // val: &slice <i32>
        for (i, &v) in vals.iter().enumerate() {
            self.data[self.node+i] = v;
        }
        for i in (0..self.node).rev() {
            self.data[i] = self.data[2*(i+1)-1].max(self.data[2*(i+1)]);
        }
    }

    fn update(&mut self, mut idx: usize, val: i32) {
        idx += self.node;
        self.data[idx] = val;
        while idx > 0 {
            idx /= 2;
            self.data[idx] = self.data[2*(idx+1)-1].max(self.data[2*(idx+1)]);
        }
    }
}

fn main() {
    let mut rng = thread_rng();
    let args = Args::parse();

    let node: u32 = rng.gen_range(9..=16);
    let mut vec: Vec<i32> = Vec::new();
    
    for _ in 0..node {
        vec.push(rng.gen_range(1..100));
    }

    let mut segment_tree = SegmentTree::new(16, 0);

    segment_tree.build(&vec);

    for i in 0..1 as usize {
        print!("                               {:>2}                               ", segment_tree.data[i]);
    }
    println!();
    for i in 1..3 as usize {
        print!("               {:>2}               ", segment_tree.data[i]);
    }
    println!();
    for i in 3..7 as usize {
        print!("       {:>2}       ", segment_tree.data[i]);
    }
    println!();
    for i in 7..15 as usize {
        print!("   {:>2}   ", segment_tree.data[i]);
    }
    println!();
    for i in 15..31 as usize {
        print!(" {:>2} ", segment_tree.data[i]);
    }
    println!();
}