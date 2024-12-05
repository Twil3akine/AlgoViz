use rand::{Rng, thread_rng};
use clap::Parser;
use std::io;

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

    fn show(&self, idx: Option<usize>) -> () {
        let mut start: usize = 0;
        let mut end: usize;
        let mut width: usize = 2*self.node+1;
        let mut spaces: String;
        // 15 7 3 1
        while start < self.data.len() {
            end = start + (start+1).next_power_of_two();
            spaces= " ".repeat(width);

            for i in start..end.min(self.data.len()) {
                if let Some(idxv) = idx {
                    if self.data[i] == self.data[idxv+self.node] {
                        print!("{}\x1b[32m{:>2}\x1b[0m{}", spaces, self.data[i], spaces);
                        continue;
                    }
                }
                print!("{}{:>2}{}", spaces, self.data[i], spaces);
            }
            println!();
            
            width /= 2;
            start = end;
            if width == 0 {
                break;
            }
        }
        println!("\n");
    }

    fn build(&mut self, vals: &[i32], step_by_step: bool) { // vals: &slice <i32>
        println!("Initial Segment Tree:\n");
        self.show(None);

        for (i, &v) in vals.iter().enumerate() {
            self.update(i, v);

            if step_by_step {
                self.show(Some(i));
                println!("please push 'Enter' for next.");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("please push 'Enter'");
            }
        }

        println!("Finish Building Segment Tree:\n");
        self.show(None);
    }

    fn update(&mut self, mut idx: usize, val: i32) {
        idx += self.node;
        self.data[idx] = val;
        while idx > 0 {
            if idx%2==0 {
                idx = (idx-1)/2;
            } else {
                idx /= 2;
            }
            self.data[idx] = self.data[2*(idx+1)-1].max(self.data[2*(idx+1)]);

            if self.data[idx] != val {
                break;
            }
        }
    }
}

fn main() {
    let mut rng = thread_rng();
    let args = Args::parse();

    let node: u32 = rng.gen_range(5..=16);
    let mut vec: Vec<i32> = Vec::new();
    
    for _ in 0..node {
        vec.push(rng.gen_range(1..100));
    }

    let mut segment_tree = SegmentTree::new(vec.len(), 0);

    segment_tree.build(&vec, args.step_by_step);
}