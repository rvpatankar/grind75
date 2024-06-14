// does not pass all cases.
/*
pub fn is_match(s: String, p: String) -> bool {
    let mut retval = false;
    if s == p {
        retval = true;
    } else {
        let svec:Vec<char> = s.chars().collect();
        let pvec:Vec<char> = p.chars().collect();
        let mut i = 0;
        let mut j = 0;
        loop {
            if i < svec.len() {
                if j+1 < pvec.len() && pvec[j+1] == '*' {
                    if svec[i] == pvec[j] || pvec[j] == '.' {
                        print!("svec={}, pvec={} - ", svec[i], pvec[j]);
                        retval = true;
                        i += 1;
                        println!("1 ... retval={}, i={}, j={}", retval, i, j);
                        continue;
                    } else {
                        print!("svec={}, pvec={} - ", svec[i], pvec[j]);
                        j += 2;
                        println!("2 ... retval={}, i={}, j={}", retval, i, j);
                        continue;
                    }
                } else {
                    if j < pvec.len(){
                        if svec[i] == pvec[j] || pvec[j] == '.' {
                            print!("svec={}, pvec={} - ", svec[i], pvec[j]);
                            retval = true;
                            i += 1;
                            j += 1;
                            println!("3 ... retval={}, i={}, j={}", retval, i, j);
                            continue;
                        } else {
                            print!("svec={}, pvec={} - ", svec[i], pvec[j]);
                            retval = false;
                            println!("3.1 ... retval={}, i={}, j={}", retval, i, j);
                            break;
                        }
                    } else {
                        retval = false;
                        println!("3.2 ... retval={}, i={}, j={}", retval, i, j);
                        break;
                    }
                }
            } else {
                if j < pvec.len() {
                    println!("pvec left = {:?}", &pvec[j..]);
                    if (j+1 < pvec.len()) && (pvec[j+1] == '*') {
                        println!("4 ... retval={}, i={}, j={}", retval, i, j);
                        j += 2;
                        continue;
                    }
                    if pvec[j] == '*' || (pvec[j] == svec[i-1] && pvec[j-1] == '*' && svec.len() > 1) {
                        j += 1;
                        println!("4.1 ... retval={}, i={}, j={}", retval, i, j);
                        continue;
                    } else {
                        retval = false;
                        println!("4.2 ... retval={}, i={}, j={}", retval, i, j);
                        break;
                    }
                } else {
                    println!("4.3 ... retval={}, i={}, j={}", retval, i, j);
                    break;
                }
            }
        }
    }
    retval    
}
*/
// watch video https://www.youtube.com/watch?v=s5Ts6Hbq_00
pub fn is_match(s: String, p: String) -> bool { 
    let svec:Vec<char> = s.chars().collect();
    let pvec:Vec<char> = p.chars().collect();
    let row = svec.len()+1;
    let col = pvec.len()+1;
    let mut dp = Vec::with_capacity(col);
    for i in 0..row {
        dp.push(vec![false; col]);
    }
    // E.g. start
    //	    _ 	. 	* 	. 	.
    // _ 	  f 	f 	f 	f 	f
    // a 	  f 	f 	f 	f 	f
    // b 	  f 	f 	f 	f 	f
    dp[0][0] = true;
    for j in 1..col {
        if pvec[j-1] == '*' {
            dp[0][j] = dp[0][j-2];
        }
    } 
    // after filling first row
    //	    _ 	. 	* 	. 	.
    // _ 	  t 	f 	t 	f 	f
    // a 	  f 	f 	f 	f 	f
    // b 	  f 	f 	f 	f 	f
    for i in 1..row {
        for j in 1..col {
            if pvec[j-1] == '*'{
                // if '*' two conditions char before * can match 0 times or 1 or more times.
                // for 0 match take value from same row two column before.
                // 			  _ 	 (c) 	  * 	  a 	* 	b
                // 	_ 		t 	  f 	  t 	  f 	t 	f
                // (a)	 <f>    f    <f>
                // 	b 		f
                // for 1 or more match take previous row same column.
                // 		  _ 	  c     * 	 (a) 	 * 		b
                // 	_ 	t 	  f     t 	  f 	<t> 	f
                // (a)	f     f     f 	  f 	<t>
                // 	b 	f
                //0 matches
                dp[i][j] = dp[i][j-2];

                // if 1+ match
                if svec[i-1] == pvec[j-2] || pvec[j-2] == '.' {
                    dp[i][j] = dp[i][j] || dp[i-1][j];
                }
            } else if pvec[j-1] == '.' || svec[i-1] == pvec[j-1] {
                // if pattern is . or pattern char matches string char tha 
                // take value from previous row and previous column.
                // 		  _ 	  c 	 *	   (a) 	 * 		b
                // 	_ 	t 	  f 	<t> 	  f 	 t 	  f
                // (a)	f     f    f 	   <t> 	
                // 	b 	f
                dp[i][j] = dp[i-1][j-1];
            }
        }
        println!("{:?}", dp);
    } 
    dp[row-1][col-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(is_match("aa".to_string(), "a".to_string()), false);
    }
    
    #[test]
    fn ex2() {
        assert_eq!(is_match("aa".to_string(), "a*".to_string()), true);
    }
    
    #[test]
    fn ex3() {
        assert_eq!(is_match("ab".to_string(), ".*".to_string()), true);
    }
    
    #[test]
    fn ex4() {
        assert_eq!(is_match("aab".to_string(), "c*a*b".to_string()), true);
    }
    
    #[test]
    fn ex5() {
        assert_eq!(is_match("mississippi".to_string(), "mis*is*p*".to_string()), false);
    }
    
    #[test]
    fn ex6() {
        assert_eq!(is_match("ab".to_string(), ".*c".to_string()), false);
    }
    
    #[test]
    fn ex7() {
        assert_eq!(is_match("aaa".to_string(), "aaaa".to_string()), false);
    }
    
    #[test]
    fn ex8() {
        assert_eq!(is_match("aaa".to_string(), "a*a".to_string()), true);
    }
    
    #[test]
    fn ex9() {
        assert_eq!(is_match("aaa".to_string(), "ab*a*c*a".to_string()), true);
    }
   
    #[test]
    fn ex10() {
        assert_eq!(is_match("a".to_string(), "ab*a".to_string()), false);
    }
    
    #[test]
    fn ex11() {
        assert_eq!(is_match("ab".to_string(), ".*..".to_string()), true);
    }
}
