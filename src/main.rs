mod problems;
mod test_verify;
mod utils;

fn main() {
    problems::medium::p16_3sum_closest::Solution::three_sum_closest(
        vec![4, 0, 5, -5, 3, 3, 0, -4, -5],
        -2,
    );
    println!("LeetCode solutions");
}
