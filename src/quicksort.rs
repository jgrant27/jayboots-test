use rand::{thread_rng, Rng};

fn gen_random_vec(cnt: u64) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<u64> = Vec::new();
    for _ in 1..=cnt {
        nums.push(rng.gen_range(1, cnt+1));
    }
    nums
}

pub fn quicksort_rec(nums: Vec<u64>) -> Vec<u64> {
    match nums.len() {
        cnt if cnt <= 1 => nums,
        cnt => {
            let mut left = Vec::new();
            let mut right = Vec::new();
            let pivot_index = thread_rng().gen_range(0, cnt);
            let pivot = nums[pivot_index];
            for i in 0..cnt {
                if i != pivot_index {
                    match nums[i] {
                        num if num <= pivot => left.push(num),
                        num => right.push(num),
                    }
                }
            }
            let mut left_sorted = quicksort_rec(left);
            let mut right_sorted = quicksort_rec(right);
            left_sorted.push(pivot);
            left_sorted.append(&mut right_sorted);
            left_sorted
        }
    }
}

pub fn test() -> String {
    //let res = format!("{:?}", quicksort_rec(gen_random_vec(100)));
    //let res = format!("{:?}", gen_random_vec(100));
    //let res = format!("{:?}", "hey there!");
    //let res = format!("{:?}", quicksort_rec(vec![3, 4, 2, 1, 5]));
    let res = format!("{:?}", quicksort_rec(gen_random_vec(1000)));
    res
}
