/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素 方法二 优化后的向中靠拢的双指针
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // 左开右闭
        let mut left: usize = 0;
        let mut right: usize = nums.len();
        while left < right {
            if nums[right - 1] == val {
                right -= 1;
            } else if nums[left] != val {
                left += 1;
            } else {
                nums[left] = nums[right - 1];
                left += 1;
                right -= 1;
            }
        }
        return left as i32;
    }
}

// @lc code=end
