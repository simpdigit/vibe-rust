// This file contains functions and structures related to vector operations.

pub fn create_vector(size: i16) -> Vec<i32> {
    // Create a vector with the specified size
    // and fill it with the first `size` integers
    vec![0; size as usize]
        .iter()
        .enumerate()
        .map(|(i, _)| i as i32)
        .collect()
}

pub fn sum_vector(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

pub fn double_vector(vec: &Vec<i32>) -> Vec<i32> {
    let result1: Vec<i32> = vec.iter().map(|&x| x * 2).collect();
    let mut result = Vec::with_capacity(vec.len());
    vec.iter().for_each(|&x| result.push(x * 2));
    assert!(result1 == result);
    result1
}

pub fn get_prime_numbers(vec: &Vec<i32>) -> Vec<i32> {
    vec.iter()
        .cloned()
        .filter(|x| is_prime(*x))
        .collect()
}

pub fn is_prime(num: i32) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    let sqrt = (num as f64).sqrt() as i32;
    for i in (3..=sqrt).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}