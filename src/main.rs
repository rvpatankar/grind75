use grind75::Grind_02_Add_Two_Numbers::*;

fn main() {
    //let l1 = vec_to_node(vec![2, 4, 3]);
    //let l2 = vec_to_node(vec![5, 6, 4]);
    let l1 = vec_to_node(vec![9,9,9,9,9,9,9]);
    let l2 = vec_to_node(vec![9,9,9,9]);
    let l3 = add_two_numbers(l1, l2);
    print_listnode(l3);
}
