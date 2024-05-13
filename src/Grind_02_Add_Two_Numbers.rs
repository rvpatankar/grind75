// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
 
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}


pub fn vec_to_node(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &num in v.iter() {
        let node = ListNode::new(num);
        *current = Some(Box::new(node));
        current = &mut current.as_mut().unwrap().next;
    }
    head
}

pub fn print_listnode(mut head: Option<Box<ListNode>>){
    while let Some(node) = head {
        print!("{} -> ", node.val);
        head = node.next;
    }
    println!("None");
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //let mut head = None;
    //let mut current = &mut head;

    let mut v1 = Vec::new();
    let mut res: i32 = 0;
    let mut carry: i32 = 0;
    let mut node1 = &l1;
    let mut node2 = &l2;
    loop {
        match (node1, node2) {
            (Some(n1), Some(n2)) => {
                res = n1.val + n2.val + carry;
                //let node = ListNode::new(res % 10);
                //*current = Some(Box::new(node));
                //current = &mut current.as_mut().unwrap().next;
                v1.push(res % 10);
                carry = res / 10;
                node1 = &n1.next;
                node2 = &n2.next;
            },
            (Some(n1), None) => {
                res = n1.val + carry;
                //let node = ListNode::new(res % 10);
                //*current = Some(Box::new(node));
                //current = &mut current.as_mut().unwrap().next;
                v1.push(res % 10);
                carry = res / 10;
                node1 = &n1.next;
            },
            (None, Some(n2)) => {
                res = n2.val + carry; 
                //let node = ListNode::new(res % 10);
                //*current = Some(Box::new(node));
                //current = &mut current.as_mut().unwrap().next;
                v1.push(res % 10);
                carry = res / 10;
                node2 = &n2.next;
            },
            (None, None) => {
                if carry > 0 {    
                    //let node = ListNode::new(carry);
                    //*current = Some(Box::new(node));
                    //current = &mut current.as_mut().unwrap().next;
                    v1.push(carry);
                }
                println!("BREAK");
                break;
            },
            _ => {
                println!("BREAK");
                break;
            }
        }
    }
    vec_to_node(v1)
    //head
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let l1 = vec_to_node(vec![2,4,3]);
        let l2 = vec_to_node(vec![5,6,4]);
        let r = vec_to_node(vec![7,0,8]);
        assert_eq!(add_two_numbers(l1, l2), r);
    }
    #[test]
    fn ex2() {
        let l1 = vec_to_node(vec![0]);
        let l2 = vec_to_node(vec![0]);
        let r = vec_to_node(vec![0]);
        assert_eq!(add_two_numbers(l1, l2), r);
    }
    #[test]
    fn ex3() {
        let l1 = vec_to_node(vec![9,9,9,9,9,9,9]);
        let l2 = vec_to_node(vec![9,9,9,9]);
        let r = vec_to_node(vec![8,9,9,9,0,0,0,1]);
        assert_eq!(add_two_numbers(l1, l2), r);
    }
}
