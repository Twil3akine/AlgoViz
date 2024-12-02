use rand::{Rng, thread_rng};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short='s', long="step_by_step", help="Enable step-by-step mode")]
    step_by_step: bool,
}

fn segment_tree(vec: &mut Vec<u32>, node: u32) -> () {
    let mut rng = thread_rng();
    let mut cnt: u32 = node;
    let mut idx: u32 = 0;
    while cnt>1 {
        cnt /= 2;
        idx += 1;
    }
    cnt = 2u32.pow(idx.into())-1u32;

    for _i in cnt..(2u32.pow(node.into())-1) {
        vec.push(
            if _i < cnt+node {
                rng.gen_range(1..100)
            } else {
                0
            }
        );
        let mut i: usize = _i as usize;
        while i>0 {
            if i%2==0 {
                i -= 1;
            }
            i /= 2;
            if vec[i] < vec[_i as usize] {
                vec[i] = vec[_i as usize]
            } else {
                break;
            }
        }
    }
}

fn main() {
    let mut rng = thread_rng();
    let args = Args::parse();

    let node: u32 = rng.gen_range(4..=16);
    let mut vec: Vec<u32> = vec!(0; 15);

    segment_tree(&mut vec, node);

    for i in 0..1 as usize {
        print!("                               {:>2}                               ", vec[i]);
    }
    println!();
    for i in 1..3 as usize {
        print!("               {:>2}               ", vec[i]);
    }
    println!();
    for i in 3..7 as usize {
        print!("       {:>2}       ", vec[i]);
    }
    println!();
    for i in 7..15 as usize {
        print!("   {:>2}   ", vec[i]);
    }
    println!();
    for i in 15..31 as usize {
        print!(" {:>2} ", vec[i]);
    }
    println!();
}