pub mod two_sum {
    use std::collections::HashMap;
    pub struct Solution;
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut map = HashMap::with_capacity(nums.len());
            for (index, num) in nums.iter().enumerate() {
                match map.get(&(target - num)) {
                    None => {
                        map.insert(num, index);
                    }
                    Some(sub_index) => {
                        return vec![*sub_index as i32, index as i32];
                    }
                }
            }
            vec![]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Solution;
        #[test]
        fn it_works() {
            let nums = vec![2, 7, 11, 15];
            let target = 9;
            assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
        }
    }
}
pub mod add_two_numbers {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    pub struct Solution;
    impl Solution {
        pub fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut result: Option<Box<ListNode>> = None;
            let (mut curr1, mut curr2, mut curr3) = (&l1, &l2, &mut result);
            let (mut flag1, mut flag2) = (false, false);
            let mut ten_num = 0;
            loop {
                let tmp1 = match curr1 {
                    Some(v) => {
                        let a = v.val;
                        curr1 = &v.next;
                        a
                    }
                    None => {
                        flag1 = true;
                        0
                    }
                };
                let tmp2 = match curr2 {
                    Some(v) => {
                        let a = v.val;
                        curr2 = &v.next;
                        a
                    }
                    None => {
                        flag2 = true;
                        0
                    }
                };

                if flag1 && flag2 && ten_num == 0 {
                    break;
                }
                *curr3 = Some(Box::new(ListNode::new((tmp1 + tmp2 + ten_num) % 10)));
                if let Some(v) = curr3 {
                    curr3 = &mut v.next
                }
                ten_num = (tmp1 + tmp2 + ten_num) / 10;
            }
            result
        }
    }
    pub fn num_to_listnode(num: i128) -> Option<Box<ListNode>> {
        if num != 0 {
            return Some(Box::new(ListNode {
                val: (num % 10) as i32,
                next: num_to_listnode(num / 10),
            }));
        } else {
            return None;
        }
    }
    #[cfg(test)]
    mod tests {
        use crate::leetcode::add_two_numbers::num_to_listnode;

        use super::Solution;
        #[test]
        fn it_works() {
            assert_eq!(
                Solution::add_two_numbers(num_to_listnode(10), num_to_listnode(20)),
                num_to_listnode(30)
            );
            assert_eq!(
                Solution::add_two_numbers(num_to_listnode(999999), num_to_listnode(999)),
                num_to_listnode(1000998)
            );
            assert_eq!(
                Solution::add_two_numbers(num_to_listnode(0), num_to_listnode(0)),
                num_to_listnode(0)
            );
            assert_eq!(
                Solution::add_two_numbers(num_to_listnode(20), num_to_listnode(0)),
                num_to_listnode(20)
            );
        }
    }
}
pub mod length_of_longest_substring {
    use std::collections::HashMap;

    //给定一个字符串s，请你找出其中不含有重复字符的最长子串的长度。
    pub struct Solution;
    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let mut result = 0;
            let sc: Vec<char> = s.chars().collect();
            let mut map: HashMap<char, usize> = HashMap::new();
            let (mut i, mut j) = (0usize, 0usize);
            while j < sc.len() {
                if map.contains_key(&sc[j]) {
                    if let Some(v) = map.get(&sc[j]) {
                        if v < &i {
                            map.insert(sc[j], j);
                        } else {
                            i = v.clone() + 1;
                            map.insert(sc[j], j);
                        }
                    }
                    result = result.max(j as i32 - i as i32 + 1);
                    j += 1;
                } else {
                    map.insert(sc[j], j);
                    result = result.max(j as i32 - i as i32 + 1);
                    j += 1;
                }
            }
            result
        }
    }
    #[cfg(test)]
    mod test {
        use crate::leetcode::length_of_longest_substring::Solution;

        #[test]
        fn test1() {
            assert_eq!(
                Solution::length_of_longest_substring("tmmzuxt".to_string()),
                5
            );
            assert_eq!(
                Solution::length_of_longest_substring("abcabcbb".to_string()),
                3
            );
            assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
            assert_eq!(
                Solution::length_of_longest_substring("bbbbbb".to_string()),
                1
            );
        }
    }
}
