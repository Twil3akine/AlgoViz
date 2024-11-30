use rand::{Rng, thread_rng};
use clap::Parser;

#[derive(Parser)]
struct Args {
    sort_type: String,
}

// 線を引く
fn line(idx: usize) -> () {
    print!(" ");
    for _i in 0..=idx {
        print!("---- ");
    }
    println!();
}

// マスの出力
fn grid(vec: &Vec<u32>, idx: usize, highlight: Option<(usize, usize)>) -> () {

    line(idx);

    print!("|");
    for (i,v) in vec.iter().enumerate() {
        if let Some((x,y)) = highlight {
            if i == x {
                print!(" \x1b[32m{:>2}\x1b[0m |", v);
                continue;
            }
            else if i == y {
                print!(" \x1b[31m{:>2}\x1b[0m |", v);
                continue;
            }
        }
        print!(" {:>2} |", v);
    }

    println!();

    line(idx);
}

// ソート実行
fn sort(vec: &mut Vec<u32>, idx: usize, method: &str) -> () {
    match method {
        "Bubble" => {
            for i in 0..idx {
                for j in 0..idx-i {
                    if vec[j]>vec[j+1] {
                        vec.swap(j, j+1);
                        grid(vec, idx, Some((j,j+1)));
                    }
                    else { 
                        continue; 
                    }
                }
            }
        }

        "Choose" => {
            for i in 0..idx {
                let mut mx: usize = i;
                for j in i..idx+1 {
                    if vec[j] < vec[mx] {
                        mx = j;
                    }
                }
                vec.swap(i, mx);
                grid(vec, idx, Some((i,mx)));
            }
        }

        "Insert" => {
            for i in 1..idx+1 {
                let key: u32 = vec[i];
                let mut j: usize = i;
                while j > 0 && vec[j-1] > key {
                    vec[j] = vec[j-1];
                    j -= 1;
                }
                vec[j] = key;
                grid(vec, idx, Some((j,j)));
            }
        }

        _ => panic!("Invalid sorting method!"),
    }
}

fn main() {
    let mut rng = thread_rng();
    let args = Args::parse();

    let idx: usize = rng.gen_range(7..=10);
    let mut vec: Vec<u32> = (0..=idx).map(|_| rng.gen_range(0..=100)).collect();

    println!("\nInitial Array:");
    grid(&vec, idx, None);
    println!();

    sort(&mut vec, idx, &args.sort_type);

    println!("\nSorted Array:");
    grid(&vec, idx, None);
}