use find_median::summary_generator::get_num_summary;

/**
* From the Rust book:
* Given a list of integers, use a vector and return the median (when sorted,
* the value in the middle position) and mode (the value that occurs most often;
* a hash map will be helpful here) of the list.
*/

fn main() {
    if let Some(summary) = get_num_summary(&vec![1, 2, 2, 3]) {
        println!("Here's your number summary: {:?}", summary);
    } else {
        println!("Could not compute the result!")
    }
}
