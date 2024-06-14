pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }
    /*
    if x % 11 == 0 {
        return true;
    }
    */
    let mut xvec = Vec::new();
    let mut num = x;
    while num / 10 > 0  {
        xvec.push(num % 10);
        num = num / 10;
        println!("num={}", num);
    }
    xvec.push(num);
    /*
    if xvec.len() % 2 == 0 {
        return false;
    }
    */
    println!("vec={:?}", xvec);
    println!("len={:?}", xvec.len());
    let mut retval = false;
    let mut till = (xvec.len()/2) - 1;
    for i in 0..=till {
        println!("F[{}]={}, E[{}]={}", i, xvec[i], (xvec.len()-1-i), xvec[xvec.len()-1-i] );
        if xvec[i] == xvec[xvec.len()-1-i] {
            retval = true;
        } else {
            return false;
        }
    }
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(is_palindrome(121), true);
    } 
    #[test]
    fn ex2() {
        assert_eq!(is_palindrome(-121), false);
    }
    #[test]
    fn ex3() {
        assert_eq!(is_palindrome(10), false);
    }
    #[test]
    fn ex4() {
        assert_eq!(is_palindrome(6543456), true);
    }
    #[test]
    fn ex5() {
        assert_eq!(is_palindrome(8), true);
    }
    #[test]
    fn ex6() {
        assert_eq!(is_palindrome(23677632), true);
    }
    #[test]
    fn ex7() {
        assert_eq!(is_palindrome(1122), false);
    }
}
