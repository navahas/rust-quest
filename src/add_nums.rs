#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "List Node: {} {:?}", self.val, self.next)
    }
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    fn from_array(arr: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &arr in arr.iter().rev() {
            current = Some(Box::new(ListNode { val: arr, next: current }));
        }
        current
    }
}

fn convert_to_num(mut l: Option<Box<ListNode>>) -> i32 {
    let mut l_vec = Vec::new();
    while let Some(node) = l {
        l_vec.push(node.val);
        l = node.next
    };
    l_vec.reverse();
    let num = l_vec.iter().fold(0, |acc, x| acc * 10 + x);
    num
}

fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    let l1_num = convert_to_num(l1);
    let l2_num = convert_to_num(l2);
    let output = l1_num + l2_num;
    println!("{} + {} = {}", l1_num, l2_num, output);
    let output_vec: Vec<i32> = output.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    ListNode::from_array(output_vec)
}

fn main() {
    let l1 = ListNode::from_array(vec![2, 4, 3]);
    let l2 = ListNode::from_array(vec![5, 6, 4]);
    let output = add_two_numbers(l1, l2);
    println!("{:?}", output);
    //output = [7, 0, 8];
}
