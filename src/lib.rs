pub mod summary_generator {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct NumberSummary {
        median: f32,
        mode: i32,
    }

    pub fn get_num_summary(nums: &Vec<i32>) -> Option<NumberSummary> {
        if nums.len() == 0 {
            // can't find median/mode
            return None;
        }

        // make a copy of nums and sort them
        let mut nums_copy = nums.to_owned();
        nums_copy.sort();

        // find the median
        let median: f32 = match nums_copy.len() % 2 {
            0 => {
                // even-length list
                let left = nums_copy.len() / 2 - 1;
                let right = nums_copy.len() / 2;
                (nums_copy[left] + nums_copy[right]) as f32 / 2.0
            }
            _ => {
                // odd-length list
                nums_copy[nums_copy.len() / 2] as f32
            }
        };

        // find the mode by counting all the nums up
        let mut counts = HashMap::new();
        // initialize mode to the first num in the list to start with
        let mut mode: i32 = nums_copy
            .first()
            .expect("First vector element should exist at this point since we checked its length.")
            .to_owned();

        for num in nums_copy {
            // update the count for each num
            counts.entry(num).and_modify(|c| *c += 1).or_insert(1);

            // see if we need to update mode
            let count = counts.get(&num).expect(
                "Count for num should exist, because we've already inserted it into the map.",
            );
            if count > counts.get(&mode).expect("Count for mode should exist, because it should always be equal to a number that is already accounted for in the counter map.") {
                mode = num;
            }
        }

        Some(NumberSummary { median, mode })
    }

    #[cfg(test)]
    mod tests {
        use crate::summary_generator::{get_num_summary, NumberSummary};

        #[test]
        fn it_works_in_the_base_case() {
            let NumberSummary { median, mode } = get_num_summary(&vec![1, 2, 3]).unwrap();
            assert_eq!(median, 2f32);
            assert_eq!(mode, 1);
        }

        #[test]
        fn it_works_for_empty_vec() {
            let res = get_num_summary(&vec![]);
            assert!(res.is_none());
        }

        #[test]
        fn it_works_for_one_num() {
            let res = get_num_summary(&vec![1]).unwrap();
            assert_eq!(res.median, 1f32);
            assert_eq!(res.mode, 1);
        }

        #[test]
        fn it_works_with_negative_nums() {
            let res = get_num_summary(&vec![-10, -5, -5]).unwrap();
            assert_eq!(res.median, -5f32);
            assert_eq!(res.mode, -5);
        }

        #[test]
        fn it_works_for_even_len_vecs() {
            let res = get_num_summary(&vec![1, 1, 2, 2]).unwrap();
            assert_eq!(res.mode, 1);
            assert_eq!(res.median, 1.5);
        }

        #[test]
        fn mode_should_be_correct() {
            let res = get_num_summary(&vec![1, 1, 0, 0, 0]).unwrap();
            assert_eq!(res.mode, 0);
            assert_eq!(res.median, 0.0);
        }
    }
}
