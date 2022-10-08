use std::collections::HashMap;

pub struct Answer {
    pub median: f32,
    pub mode: i32,
}

pub fn num_summary(nums: &Vec<i32>) -> Option<Answer> {
    if nums.len() == 0 {
        // can't find median/mode
        return None;
    }

    let mut sorted_num_copy = nums.to_owned();
    sorted_num_copy.sort();
    let median: f32 = match sorted_num_copy.len() % 2 {
        0 => {
            let left = sorted_num_copy.len() / 2 - 1;
            let right = sorted_num_copy.len() / 2;
            (sorted_num_copy[left] + sorted_num_copy[right]) as f32 / 2.0
        }
        _ => sorted_num_copy[sorted_num_copy.len() / 2] as f32,
    };

    let mut counts = HashMap::new();
    let mut mode: i32 = sorted_num_copy.first().unwrap().to_owned();
    for num in sorted_num_copy {
        // update count for num
        counts.entry(num).and_modify(|c| *c += 1).or_insert(0);

        // see if we need to update mode
        let count = counts.get(&num).unwrap();
        if count > counts.get(&mode).unwrap() {
            mode = num;
        }
    }

    Some(Answer { median, mode })
}

#[cfg(test)]
mod tests {
    use crate::{num_summary, Answer};

    #[test]
    fn it_works() {
        let Answer { median, mode } = num_summary(&vec![1, 2, 3]).unwrap();
        assert_eq!(median, 2f32);
        assert_eq!(mode, 1);
    }

    #[test]
    fn it_works_for_empty_vec() {
        let res = num_summary(&vec![]);
        assert!(res.is_none());
    }

    #[test]
    fn it_works_for_one_num() {
        let res = num_summary(&vec![1]).unwrap();
        assert_eq!(res.median, 1f32);
        assert_eq!(res.mode, 1);
    }

    #[test]
    fn it_works_with_negative_nums() {
        let res = num_summary(&vec![-10, -5, -5]).unwrap();
        assert_eq!(res.median, -5f32);
        assert_eq!(res.mode, -5);
    }

    #[test]
    fn it_works_for_even_len_vecs() {
        let res = num_summary(&vec![1, 1, 2, 2]).unwrap();
        assert_eq!(res.mode, 1);
        assert_eq!(res.median, 1.5);
    }
}
