extern crate rand;

use rand::thread_rng;

use rand::distributions::{Bernoulli, Distribution};

use rand::seq::SliceRandom;

pub fn get_empty_distance_matrice(n: u32) -> Vec<Vec<u32>> {
    let mut matrice: Vec<Vec<u32>> = Vec::new();
    let mut empty_line: Vec<u32> = Vec::new();

    for _i in 0..n {
        empty_line.push(0);
    }
    for _i in 0..n {
        matrice.push(empty_line.clone());
    }
    return matrice;
}

#[test]
fn test_get_empty_distance_matrice() {
    assert_eq!(
        get_empty_distance_matrice(3),
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
    );
}

pub fn get_random_permutation(n: u32) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut vec: Vec<u32> = (1..n as u32 + 1).collect();
    vec.shuffle(&mut rng);
    return vec;
}

pub fn fill_matrice(mut matrice: Vec<Vec<u32>>, increase_probability: f64) -> Vec<Vec<u32>> {
    let bernou = Bernoulli::new(increase_probability);

    for decal in 1..matrice.len() {
        for i in 0..matrice.len() - decal {
            let j = i + decal;
            let base_value = max(max(matrice[i + 1][j], matrice[i][j - 1]), 1);
            let new_value =
                base_value + (bernou.expect("REASON")).sample(&mut rand::thread_rng()) as u32;
            matrice[i][j] = new_value;
        }
    }

    return matrice;
}

fn max(x: u32, y: u32) -> u32 {
    if x >= y {
        return x;
    } else {
        return y;
    }
}

#[test]
fn test_max() {
    assert_eq!(max(32, 326), 326);
}
