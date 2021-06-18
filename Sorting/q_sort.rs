use std::time::{Instant};

fn swap(incoming_vec: &mut Vec<i128>, i: usize, j: usize) {
    let temp = incoming_vec[i];
    incoming_vec[i] = incoming_vec[j];
    incoming_vec[j] = temp;
}

fn strike(incoming_vec: &mut Vec<i128>, lo: usize, hi: usize) -> i32 {
    let pivot = incoming_vec[hi];
    let mut i = lo;
    let mut j = lo;

    while j < hi {
        if incoming_vec[j] < pivot {
            swap(incoming_vec, i, j);
            i += 1
        } 
        j += 1;
    }

    swap(incoming_vec, i, j);
    return i as i32
} 

fn qsort(incoming_vec: &mut Vec<i128>, lo: usize, hi: usize) {
    if lo < hi {
        let point = strike(incoming_vec, lo, hi);

        qsort(incoming_vec, lo, (point - 1) as usize);
        qsort(incoming_vec, (point + 1) as usize, hi);
    }
}

fn main() {
    generate();
}

fn generate() {   
    let sizes_of_arrays = vec![5000, 5000000];
    
    for events in sizes_of_arrays {
        let temp_to_not_lose_ownership = events;
        let mut vector = get_populated_vector(temp_to_not_lose_ownership);

        let start = Instant::now();
        let len: usize = vector.len() - 1;
        qsort(&mut vector, 0, len);
        let duration = start.elapsed();
        
        let is_correct = check(&vector);
        if is_correct {
            println!("Sorted!");
        } else {
            panic!("Not Sorted");
        }
        println!("Time taken to sort {} elements is {:?}", events, duration);
    }
}

fn check(incoming_vec: &Vec<i128>) -> bool {
    incoming_vec.windows(2).all(|w| w[0] <= w[1])
}

fn get_populated_vector(size: usize) -> Vec<i128> {
    let mut ret_vec: Vec<i128> = Vec::with_capacity(size);
    let mut track: usize = 0;

    while track < size {
        ret_vec.push(rand::random());
        track += 1;
    }

    ret_vec
}
