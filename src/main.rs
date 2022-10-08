/**
* Given a list of integers, use a vector and return the median (when sorted,
* the value in the middle position) and mode (the value that occurs most often;
* a hash map will be helpful here) of the list.
*/
use find_median::{num_summary, Answer};

fn main() -> () {
    if let Some(Answer { median, mode }) = num_summary(&vec![1, 2, 2, 3]) {
        return println!("Median was {median} and mode was {mode}");
    }

    panic!("Could not compute number summary!")
}
