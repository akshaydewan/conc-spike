use rand::{thread_rng, Rng};
use dashmap::DashMap;
use rayon::prelude::*;
use rand::rngs::ThreadRng;
use std::time::Instant;
use std::ops::Range;

const AGENTS: Range<i32> = (0..20_000_000);

fn main() {
    println!("Agents: {}", AGENTS.end);
    single_threaded();
    multi_threaded();
}

fn single_threaded() {
    let start = Instant::now();
    let mut rng = thread_rng();
    let mut grid = DashMap::with_capacity(AGENTS.end as usize);
    AGENTS.for_each(|i| update_location(&mut rng, &mut grid, i));
    println!("Single threaded completed in {:?} seconds", start.elapsed());
}

fn multi_threaded() {
    let start = Instant::now();
    let grid = DashMap::with_capacity(AGENTS.end as usize);
    AGENTS.into_par_iter().for_each(|i| update_location_par(&grid, i));
    println!("Multi threaded completed in {:?} seconds", start.elapsed());
}


fn update_location(rng: &mut ThreadRng, grid: &mut DashMap<(i32, i32), i32>, i: i32) -> () {
    // if i % 1_000_000 == 0 {
    //     println!("count: {}", i);
    // }
    let mut point = random_point(rng);
    while grid.get(&point).is_some() {
        point = random_point(rng)
    }
    // println!("insert for {} at {:?}", i, point);
    grid.insert(point, i);
}

fn random_point(rng: &mut ThreadRng) -> (i32, i32) {
    let x = rng.gen_range(0, 5660);
    let y = rng.gen_range(0, 5660);
    (x, y)
}

fn update_location_par(grid: &DashMap<(i32, i32), i32>, i: i32) -> () {
    // if i % 1_000_000 == 0 {
    //     println!("count: {}", i);
    // }

    //thread_rng is not thread safe, so creating it locally
    let mut rng = thread_rng();
    let mut point = random_point(&mut rng);
    while grid.get(&point).is_some() {
        point = random_point(&mut rng)
    }
    // println!("insert for {} at {:?}", i, point);
    grid.insert(point, i);
}
