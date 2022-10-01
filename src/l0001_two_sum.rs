use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut checked_map: HashMap<i32, i32> = HashMap::new();
    checked_map.insert(nums[0], 0);
    for i in 1..nums.len() {
        if checked_map.contains_key(&(target - nums[i])) {
            result.push(i as i32);
            result.push(checked_map.get(&(target - nums[i])).unwrap().to_owned());
            break;
        }
        checked_map.insert(nums[i], i as i32);
    }
    result
}

/*
[2,7,11,15]
9
[3,2,4]
6
[3,3]
6
*/
#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_two_sum_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let mut result = two_sum(nums, target);
        result.sort();
        assert_eq!(vec![0, 1], result);
    }

    #[test]
    fn test_two_sum_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let mut result = two_sum(nums, target);
        result.sort();
        assert_eq!(vec![1, 2], result);
    }

    #[test]
    fn test_two_sum_3() {
        let nums = vec![3, 3];
        let target = 6;
        let mut result = two_sum(nums, target);
        result.sort();
        assert_eq!(vec![0, 1], result);
    }
}
