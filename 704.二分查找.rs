/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] 二分查找
 * 左闭右开
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: usize = 0;
        let mut h: usize = nums.len();
        let mut m = (l + h) >> 1;

        while l < h {
            // 如果中间元素小于 target, 说明 target 在中间元素的右边
            // 此时 m 下标的元素可以忽略了, 但是 m + 1 下标的元素不能忽略
            // 由于是左闭右开, 因此 l 更新为 m + 1, 这样区间 [l, h) 就能将 m + 1 下标的元素包含进去
            if nums[m] < target {
                l = m + 1;
            }
            // 如果中间元素大于 target, 说明 target 在中间元素的左边
            // 此时 m 下标的元素可以忽略了, 但是 m - 1 下标的元素不能忽略
            // 由于是左开右闭, 因此 h 更新为 m, 这样区间 [l, h) 才能将 m - 1 下标的元素包含进去
            else if nums[m] > target {
                h = m;
            }

            if nums[m] == target {
                return m as i32;
            }

            m = (l + h) >> 1;
        }

        -1
    }
}
// @lc code=end
