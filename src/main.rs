extern crate robinson_mmodules;

use robinson_mmodules::{DistanceMatrice, Robin};

mod test;
use test::{fill_matrice, get_empty_distance_matrice, get_random_permutation};

use std::time::SystemTime;

use std::thread;

const SIZE: u32 = 5;

const STACK_SIZE: usize = 4 * 1024 * 1024;

const DISPLAY_INITIAL_MAT: bool = true;
const DISPLAY_FINAL_MAT: bool = true;
const DISPLAY_FOUND_PERMUTATION: bool = true;

fn main() {
    // A bigger stack size is required for large matrice
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();
    child.join().unwrap();
}
fn run() {
    let mut initial_matrice = get_empty_distance_matrice(SIZE);

    initial_matrice = fill_matrice(initial_matrice, 0.005);

    let initial_distance_matrice = DistanceMatrice {
        distance_mat: initial_matrice,
    };

    let shuffle_permut = get_random_permutation(SIZE);

    let shuffled_distance_matrice = initial_distance_matrice.permut_matrice(&shuffle_permut);

    if DISPLAY_INITIAL_MAT {
        shuffled_distance_matrice.display_mat()
    }

    let mut robin = Robin::new(shuffled_distance_matrice.distance_mat);

    let time_start = SystemTime::now();
    println!("{}", robin.resolve_robin()); //true since, by construction, the given matrice admit a compatible order
    let time_end = SystemTime::now();

    let time_diff = time_end
        .duration_since(time_start)
        .expect("Clock may have gone backwards");

    println!("It took {:?} for a {}x{} matrice", time_diff, SIZE, SIZE);

    if DISPLAY_FOUND_PERMUTATION {
        println!("{:?}", robin.solved_permut.clone().unwrap());
    }

    if DISPLAY_FINAL_MAT {
        let final_matrice = robin
            .dist
            .permut_matrice(&robin.solved_permut.clone().unwrap());
        final_matrice.display_mat();

        println!("{}", final_matrice.is_robinson()); // Is indeed robinson
    }
}
