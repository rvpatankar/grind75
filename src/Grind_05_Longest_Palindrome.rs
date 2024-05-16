pub fn longest_palindrome(s: String) -> String {
    // Manacher's algorithm
    // s: "cbcbccde"
    let n = s.len();

    // s inserts '^', '#' and '$', and becomes Vec<u8>
    let s: Vec<u8> = "^"
            .bytes()
            .chain(s.into_bytes())
            .flat_map(|u| [u, b'#'])
            .chain("$".bytes())
            .collect();
    // s: "^#c#b#c#b#c#c#d#e#$", length: 2 * n + 3

    // Initialization: first character 'c' is center.
    let mut center = 2;
    let mut right = 3;
    // The last two characters "#$" will not be the center, so it's 2 * n + 1.
    let mut p = vec![0; 2 * n + 1];
    p[2] = 1;
    let mut max_p = 1;
    let mut max_p_index = 2;

    // Main loop.
    for i in 3..=2 * n {
        println!("P={:?}", p);
        println!("center={:?}", center);
        println!("right={:?}", right);
        println!("i={:?}", i);
        println!("P[{:?}]={:?}", i, p[i]);
        println!("s[{:?}]={:?}", i, s[i]);
        // Take advantage of last palindrome.
        if i < right {
            let i_mirror = 2 * center - i;
            p[i] = p[i_mirror].min(right - i);
        }
        println!("Intermediary::P[{:?}]={:?}", i, p[i]);
        // It's a necessary effort to stretch the boundaries.
        while s[i + p[i] + 1] == s[i - p[i] - 1] {
            p[i] += 1;
        }
        println!("Intermediary::P[{:?}]={:?}", i, p[i]);
        // Update center and right if i + p[i] is beyond the border.
        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }

        // Update max_p and its index if p[i] is bigger.
        if p[i] > max_p {
            max_p = p[i];
            max_p_index = i;
        }
    }

    // Build result from map_p_index and map_p.
    s.into_iter()
        .skip(max_p_index - max_p + 1)
        .take(2 * max_p - 1)
        .filter(|u| *u != b'#')
        .map(|u| u as char)
        .collect()
}

// solution 3 : 129ms
/*
pub fn longest_palindrome(s: String) -> String {
    let sb = s.as_bytes();
    let slen = sb.len();
    if slen == 0 {
        return "".to_string();
    }
    if slen == 1 {
        return s;
    }
    if slen == 2 {
        if sb[0] == sb[1] {
            return s; 
        } else {
            return String::from_utf8(sb[0..1].to_vec()).unwrap();
        }
    }

    let mut maxlen = 0;
    let mut retval = String::from_utf8(sb[0..1].to_vec()).unwrap();//     "".to_string();
    for i in 0..slen -1 {
        let tv = get_palindrome(sb, i);
        if tv.len() > maxlen { 
            maxlen = tv.len();
            retval = tv;
        }
    }
    retval
}

pub fn get_palindrome(str: &[u8], position: usize) -> String {
    println!("position:{}", position);
    let mut lefte = position;
    let mut righte = position + 1;
    let mut even_continue = true;
    let mut even_str = "".to_string();
    let mut odd_continue = true;
    let mut lefto;
    if position == 0 {
        lefto = 0;
        odd_continue = false;
    } else {
        lefto = position-1;
    }
    let mut righto = position + 1;
    let mut odd_str = "".to_string(); //String::from_utf8(str[position..position+1].to_vec()).unwrap();
    loop {
        println!("lefte:{}, righte: {}, even_continue: {}, even_str: {}", lefte, righte, even_continue, even_str);
        println!("lefto:{}, righto: {}, odd_continue: {}, odd_str: {}", lefto, righto, odd_continue, odd_str);
        if !even_continue && !odd_continue {
            break;
        }
        if lefte >= 0 && righte < str.len() && even_continue {
            if str[lefte] == str[righte] {
                even_str = String::from_utf8(str[lefte..righte+1].to_vec()).unwrap();
                if lefte > 0 {
                    lefte -= 1;
                } else {
                    even_continue = false;
                }
                righte += 1;
            } else {
                even_continue = false;
            }
        } else {
            even_continue = false;
        } 
        if lefto >= 0 && righto < str.len() && odd_continue { 
            if str[lefto] == str[righto] {
                odd_str = String::from_utf8(str[lefto..righto+1].to_vec()).unwrap();
                if lefto > 0 {
                    lefto -= 1;
                } else {
                    odd_continue = false;
                }
                righto += 1;
            } else {
                odd_continue = false;
            }
        } else {
            odd_continue = false;
        }
    }
    if even_str.len() > odd_str.len() {
        return even_str;
    } else {
        return odd_str;
    }
}
*/
//Solution 2 - 52ms
/*
pub fn longest_palindrome(s: String) -> String {
    let sb = s.as_bytes();
    let slen = sb.len();
    if slen == 0 {
        return "".to_string();
    }
    if slen == 1 {
        return s;
    }
    let mut maxlen = 0;
    let mut retval = "".to_string();
    for i in 0..slen-1 {
        let mut l = i;
        let mut r = slen - 1;
        loop {
            if l <= r && maxlen < (r - l + 1){
                if is_palindrome(&sb, l, r) {
                    maxlen = r - l + 1;
                    retval = s.chars().skip(l).take(r-l+1).collect();
                    break;
                } else {
                    if r > 0 {
                        r -= 1;
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
    retval
}

pub fn is_palindrome(str: &[u8], start: usize, end: usize) -> bool {
    let mut left = start;
    let mut right = end;
    let mut retval = false;
    //println!("str={}", str.chars().skip(left).take(right-left+1).collect::<String>());
    while left <= right {
        println!("left:{} == right:{}", str[left], str[right]);
        if str[left] == str[right] { 
            retval = true;
            left += 1;
            if right > 0 {
                right -= 1;
            } else {
                break;
            }
        } else {
            retval = false;
            break;
        } 
    }
    retval
}
*/
// Solution 1 - 84ms
/*
pub fn longest_palindrome(s: String) -> String {
    let mut svec:Vec<char> = s.chars().collect();
    let mut lp = "".to_string();
    let mut maxlen: usize = 0;
    //let mut resvec: &Vec<char>;
    println!("svec: {:?}", svec);
    for i in 0..svec.len() {
        //svec = s.chars().collect();
        println!("i={}", i);
        let mut tempvec = s.chars().collect::<Vec<_>>().split_off(i);
        println!("tempvec: {:?}", tempvec);
        let mut tvlen = 0;//tempvec.len(); 
        loop {
            tvlen = tempvec.len();
            println!("tempvec.len = {}", tvlen);
            if tvlen > 0 && maxlen < tvlen {
                if is_palindrome(&tempvec) {
                    maxlen = tvlen;
                    //resvec = &tempvec;
                    lp = tempvec.iter().collect();
                }
                tempvec.pop();
            } else {
                break;
            }
        }
    }
    //resvec.iter().collect()
    lp
}

pub fn is_palindrome(str: &Vec<char>) -> bool {
    let mut left = 0;
    let mut right = str.len()-1;
    let mut retval = false;
    while left <= right {
        println!("left:{} == right:{}", str[left], str[right]);
        if str[left] == str[right] { 
            retval = true;
            left += 1;
            if right > 0 {
                right -= 1;
            } else {
                break;
            }
        } else {
            retval = false;
            break;
        } 
    }
    retval
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
    }

    #[test]
    fn ex2() {
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
    }

    #[test]
    fn ex3() {
        assert_eq!(longest_palindrome("a".to_string()), "a".to_string());
    }

    #[test]
    fn ex4() {
        assert_eq!(longest_palindrome("ac".to_string()), "a".to_string());
    }

    #[test]
    fn ex5() {
        assert_eq!(longest_palindrome("bb".to_string()), "bb".to_string());
    }

    #[test]
    fn ex6() {
        assert_eq!(longest_palindrome("abcda".to_string()), "a".to_string());
    }
}
