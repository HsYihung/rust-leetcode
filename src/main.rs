mod problems;
mod test_verify;
mod utils;

fn main() {
    problems::medium::p3025_find_the_number_of_ways_to_place_people_i::Solution::number_of_pairs(
        vec![vec![0, 2], vec![2, 2], vec![0, 0], vec![2, 0]],
    );
    println!("LeetCode solutions");
}
