pub fn concatenate(list_1: Vec<u32>, list_2: &Vec<u32>) -> Vec<u32> {
    let mut new_list: Vec<u32> = Vec::new();
    for elem in list_1 {
        new_list.push(elem);
    }
    for elem in list_2 {
        new_list.push(*elem);
    }
    return new_list;
}

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

pub fn get_index(vec: &Vec<u32>, x: u32) -> Option<usize> {
    for i in 0..vec.len() {
        if vec[i] == x {
            return Some(i);
        }
    }
    return None;
}
