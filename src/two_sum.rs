use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = Vec::from([2, 7, 15, 11]);
    let target: i32 = 9;
    let solution_bf = two_sum_bf(nums.clone(), target);
    let solution_hm = two_sum_hm(nums, target);
    println!("{:?}", solution_bf);
    println!("{:?}", solution_hm);
}

// method with brute force
fn two_sum_bf(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, number) in nums.iter().enumerate() {
        for n in 0..nums.len() {
            if n == index { continue };
            if nums[n] + number == target {
                println!("We found a solution!");
                return vec![index as i32, n as i32];
            }
            println!("Complement not found, searching...");
        }
    }
    unreachable!();
}

// method with hashmap (faster)
fn two_sum_hm(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hmap: HashMap<i32, i32> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = hmap.get(&complement) {
            return vec![i as i32, index as i32];
        }
        hmap.insert(num, i as i32);
    }
    unreachable!();
}
