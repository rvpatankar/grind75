use grind75::*;
//Grind_02_Add_Two_Numbers::*;

fn main() {
    //let l1 = vec_to_node(vec![2, 4, 3]);
    //let l2 = vec_to_node(vec![5, 6, 4]);
    //let l1 = vec_to_node(vec![9,9,9,9,9,9,9]);
    //let l2 = vec_to_node(vec![9,9,9,9]);
    //let l3 = add_two_numbers(l1, l2);
    //print_listnode(l3);
    //let mut v = vec!['a', 'b', 'c', 'd', 'e'];
    //println!("before:{:?}", v);
    //v = v.split_off(2);
    //println!("after:{:?}", v);

    //let s ="abcdskaas".to_string();
    //println!("{}", length_of_longest_substring(s));
   
    let v1 = vec![1,2];
    let v2 = vec![3];
    println!("sol:{}", find_median_sorted_arrays(v1,v2));
    /*
    let mut i = 0;
    let mut j = 0;
    loop {
        if i<v1.len() && j<v2.len() {
            println!("v1:{},v2:{}",v1[i],v2[j]);
            i+=1;
            j+=1;
        } else if i<v1.len() && j>=v2.len() {
            println!("v1:{},v2:__",v1[i]);
            i+=1;
        } else if i>=v1.len() && j<v2.len() {
            println!("v1:__,v2:{}",v2[j]);
            j+=1;
        } else {
            break;
        }
    }
    */
}
