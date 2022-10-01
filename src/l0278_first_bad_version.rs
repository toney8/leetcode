// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
use crate::Solution;

impl Solution {
    fn isBadVersion(&self, version: i32) -> bool {
        false
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        // [g, g, .., g, b, ... ,b]
        // 1. init two pointers: `start = 1, end = n`
        // 2. check isBadVersion(start) and isBadVersion(end)
        // 2.1 isBadVersion(start) == true, isBadVersion(end) == true: return start
        // 2.2 isBadVersion(start) == true, isBadVersion(end) == false: not a valid case
        // 2.3 isBadVersion(start) == fase, isBadVersion(end) == false: not a valid case
        // 2.4 isBadVersion(start) == fase, isBadVersion(end) == true: need to search [start, end]. The `start` and `end` are checked already.
        // 3. search from [start, end]
        // 3.1 `mid = start + (end - start) / 2;`
        // 3.1 isBadVersion(mid) == true, need to search [start, mid]. Update `end = mid;`
        // 3.2 isBadVersion(mid) == false, need to search [mid, end]. Update `start = mid;`
        // The `start` and`end` are checked and guaranteed to be [good, bad].
        // 3.3 exit condition: `start + 1 == end`
        // 3.4 return `end`
        let mut start = 1;
        let mut end = n;
        if self.isBadVersion(start) {
            start
        } else {
            while start + 1 < end {
                let mid = start + (end - start) / 2;
                if self.isBadVersion(mid) {
                    end = mid;
                } else {
                    start = mid;
                }
            }
            end
        }
    }
}
