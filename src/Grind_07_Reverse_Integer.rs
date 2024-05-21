pub fn reverse(x: i32) -> i32 {
    //let mut i:Vec<_> = (0..32).map( |n| (x >> n) ).collect();
    println!("x={:?}", x);
    let mut num = x;
    let mut retval:i32 = 0;
    let mut i:i32 = 10;
    let mut overflow;
    while num / 10 != 0 {
        (retval, overflow) = retval.overflowing_mul(i);
        println!("overflow={}", overflow);
        if overflow {
            retval = 0;
            break;
            println!("post: i={}, x={}, retval={}", i, num, retval);
        } else {
            //retval = (retval * i) + (num % 10);
            retval = retval + (num % 10);
            num = num / 10;
            println!("post: i={}, x={}, retval={}", i, num, retval);
        }
    }
    (retval, overflow) = retval.overflowing_mul(i);
    if overflow {
        retval = 0;
    } else {
        retval = retval + (num % 10);
    }
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(reverse(123), 321);
    }
    
    #[test]
    fn ex2() {
        assert_eq!(reverse(-123), -321);
    }
    
    #[test]
    fn ex3() {
        assert_eq!(reverse(120), 21);
    }
    
    #[test]
    fn ex4() {
        assert_eq!(reverse(i32::MAX), 0);
    }
    
    #[test]
    fn ex5() {
        assert_eq!(reverse(i32::MIN), 0);
    }
}
