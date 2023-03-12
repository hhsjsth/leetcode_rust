/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // 快慢指针即可
        let mut slowIndex: usize = 0;
        for pos in (0..nums.len()) {
            if nums[pos] != val {
                nums[slowIndex] = nums[pos];
                slowIndex += 1;
            }
        }
        return slowIndex as i32;
    }
}
// @lc code=end
