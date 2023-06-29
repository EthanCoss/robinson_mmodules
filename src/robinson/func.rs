pub fn max(x: u32, y: u32) -> u32 {
    if x >= y {
        return x;
    } else {
        return y;
    }
}

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
