use rand::{Rng, thread_rng};
use clap::Parser;
use std::io;

#[derive(Parser)]
struct Args {
    #[clap(short='s', long="step_by_step", help="Enable step-by-step mode")]
    step_by_step: bool,
}

fn print_line() -> () {
    println!("\n--------------------\n");
}

struct SegmentTree {
    // 葉以外
    node: usize,
    data: Vec<i32>,
    default: i32,
}

// RMQ-SegmentTree
impl SegmentTree {
    fn new(size: usize, default: i32) -> Self {
        let node: usize = size.next_power_of_two()-1;
        SegmentTree {
            node,
            data: vec![default; 2*node+1],
            default,
        }
    }

    fn show(&self, idx: Option<usize>) -> () {
        // 何個目のノードを示すか
        let mut start: usize = 0;
        let mut end: usize;
        // ノード幅 15 7 3 1
        let mut width: usize = 2 * self.node + 1;
        let mut spaces: String;
        
        // 2の累乗の深さを計算（log2(self.node + 1) の結果）
        let mut cnt: u32 = ((self.node + 1) as f64).log2() as u32;
    
        while start < self.data.len() {
            end = start + (start + 1).next_power_of_two();
            spaces = " ".repeat(width);
            for i in start..end.min(self.data.len()) {
                // idxv = node内のindex
                if let Some(idxv) = idx {
                    // ノードとその親要素をハイライト
                    // idxv + self.node は対象のノードのインデックス
                    // cnt はノードの深さ（log2に基づく）
                    let mut parent_idx: usize = idxv + self.node;
                    for _ in 0..cnt {
                        parent_idx = (parent_idx-1)/2
                    }
    
                    // iが対象のノードまたは親ノードであれば色を付ける
                    if self.data[i] == self.data[idxv + self.node] && i == parent_idx {
                        print!("{}\x1b[32m{:>2}\x1b[0m{}", spaces, self.data[i], spaces);
                        continue;
                    }
                }
                print!("{}{:>2}{}", spaces, self.data[i], spaces);
            }
            println!();
            
            width /= 2;
            start = end;
            cnt = cnt.saturating_sub(1); // cntが負にならないようにsaturating_subを使用
            if width == 0 {
                break;
            }
        }
        println!("\n");
    }
    
    fn build(&mut self, vals: &[i32], step_by_step: bool) -> () { // vals: &slice <i32>
        println!("Initial Segment Tree:\n");
        self.show(None);

        for (i, &v) in vals.iter().enumerate() {
            self.update(i, v);

            self.show(Some(i));
            if step_by_step {
                println!("please push 'Enter' for next.");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("please push 'Enter'");
            }
        }

        println!("Finish Building Segment Tree:\n");
        self.show(None);
    }

    fn update(&mut self, mut idx: usize, val: i32) -> () {
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

    // RMQ
    fn query(&mut self, lidx: usize, ridx: usize, index: Option<usize>, left: Option<usize>, right: Option<usize>) -> i32 {
        if self.data.len() <= ridx {
            panic!("Invaild index.")
        }

        let index: usize = index.unwrap_or(0);
        let left: usize = left.unwrap_or(0);
        let right: usize = right.unwrap_or(self.node);
        
        if lidx == ridx {
            return self.data[self.node+lidx];
        }

        if ridx < left || right < lidx {
            return self.default;
        }
        if lidx <= left && right <= ridx {
            return self.data[index];
        }

        let mid: usize = (left+right)/2;
        let lval = self.query(lidx, ridx, Some(2*index+1), Some(left), Some(mid));
        let rval = self.query(lidx, ridx, Some(2*index+2), Some(mid+1), Some(right));

        lval.max(rval)
    }
}

fn main() {
    let mut rng = thread_rng();
    let args = Args::parse();

    let node: u32 = rng.gen_range(5..=16);
    let mut vec: Vec<i32> = vec![0; node.next_power_of_two() as usize];
    
    for i in 0..node as usize {
        vec[i] = rng.gen_range(1..100);
    }

    let mut segment_tree = SegmentTree::new(vec.len(), 0);

    segment_tree.build(&vec, args.step_by_step);

    for _i in 0..rng.gen_range(2..=5) {
        let left: usize = rng.gen_range(0..node.next_power_of_two() as usize);
        let right: usize = rng.gen_range(left..node.next_power_of_two() as usize);
        let rlt: i32 = segment_tree.query(left, right, None, None, None);

        print_line();

        println!(" Left: {}\nRight: {}\n\nQuery Value: {}", left+1, right+1, rlt);
    }
    print_line();
}