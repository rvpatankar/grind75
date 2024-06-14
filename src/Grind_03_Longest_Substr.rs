use std::collections::HashSet;
use std::collections::VecDeque;
// solution 4: this is also slower. Sol1 is the best.
/*
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut maxlen = 0;
    let mut right = 0;
    let mut left = 0;
    let mut sset: VecDeque<char> = VecDeque::new();
    while right < s.len() {
        if !sset.contains(&s.chars().nth(right).unwrap()) {
            sset.push_back(s.chars().nth(right).unwrap());
            right += 1;
            maxlen = maxlen.max(right - left);
        } else {
            sset.pop_front();
            left += 1;
        }
    }
    maxlen.try_into().unwrap()
}
*/

//solution 3 - sliding window. Time wise this is worse than sol1 & sol2
/*
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut maxlen = 0;
    let mut right = 0;
    let mut left = 0;
    let    let mut sset = HashSet::new();
    while right < s.len() {
        if !sset.contains(&s.chars().nth(right).unwrap()) {
            sset.insert(s.chars().nth(right).unwrap());
            right += 1;
            maxlen = maxlen.max(right - left);
        } else {
            sset.remove(&s.chars().nth(left).unwrap());
            left += 1;
        }
    }
    maxlen.try_into().unwrap()
}
*/
// solution 1 takes less time than solution 2
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let svec:Vec<char> = s.chars().collect();
    let mut substr: String = svec[0].to_string(); 
    let mut maxlen: usize = substr.len();
    let mut iter: usize = 1;
    let mut i: usize = 1;
    //for i in 1..svec.len() {
    while i < svec.len() {
        match substr.find(svec[i]) {
            Some(indx) => {
                if substr.len() > maxlen {
                    maxlen = substr.len();
                }
                iter += 1;
                i = iter;
                substr = svec[i-1].to_string();
                continue;
            },
            None => substr.push(svec[i])
        }
        i += 1;
    }
    if substr.len() > maxlen {
        maxlen = substr.len();
    }
        
    maxlen.try_into().unwrap()
}

//Solution 2 Takes more time than solution 1
/*
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let mut svec:Vec<char> = s.chars().collect();
    let mut substr: String = svec[0].to_string(); 
    let mut maxlen: usize = substr.len();
    //println!("svec len = {}", svec.len());
    //println!("svec={:?}", svec);
    let mut iter: usize = 1;
    let mut i: usize = 1;
    //for i in 1..svec.len() {
    while i < svec.len() {
        //println!("substr:before={}", substr);
        //println!("i={}", i);
        //println!("iter={}", iter);
        match substr.find(svec[i]) {
            Some(indx) => {
                //println!("substr index={}", indx);
                if substr.len() > maxlen {
                    maxlen = substr.len();
                }
                //iter += 1;
                //i = iter;
                svec = svec.split_off(indx+1);
                //println!("svec after split={:?}", svec);
                substr = svec[0].to_string();
                i = 1;
                continue;
            },
            None => substr.push(svec[i])
        }
        //println!("substr:after={}", substr);
        i += 1;
    }
    if substr.len() > maxlen {
        maxlen = substr.len();
    }
    maxlen.try_into().unwrap()
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }
    
    #[test]
    fn ex2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }

    #[test]
    fn ex4() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn ex5() {
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
    }

    #[test]
    fn ex6() {
        assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    }
}
