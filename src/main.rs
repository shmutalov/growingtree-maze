extern crate rand;

pub mod growing_tree_maze;

use rand::{thread_rng, Rng};
use growing_tree_maze::GrowingTreeMaze;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let width: usize = if args.len() > 1  {args[1].parse::<usize>().unwrap_or(40)} else {40};
    let height: usize = if args.len() > 2  {args[2].parse::<usize>().unwrap_or(20)} else {20};

    let mut maze = GrowingTreeMaze::new(width, height);

    let mut rng = thread_rng();
    let x_start = rng.gen_range::<usize>(0, width-1);
    let y_start = rng.gen_range::<usize>(0, height-1);

    maze.generate(x_start, y_start, 0_f64);
    maze.print();
}
