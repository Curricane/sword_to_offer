fn main() {
    println!("Hello, world!");
}

trait Solution {
    fn find_dupicate(nums: Vec<i32>) -> i32;
}

// 快慢指针
struct Impl1;
impl Solution for Impl1 {
    fn find_dupicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        // 1. 快慢指针相遇
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        while slow != fast {
            // 1.1 慢指针走1步
            slow = nums[slow] as usize;
            // 1.2 快指针走2步
            fast = nums[nums[fast] as usize] as usize;
        }
        // 2. 找环的入口
        slow = 0;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }
        slow as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_impl1() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(Impl1::find_dupicate(nums), 3);
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(Impl1::find_dupicate(nums), 2);
    }
}
