pub fn my_atoi(s: String) -> i32 {
    let ss = s.trim();
    let mut sschars = ss.chars();
    let mut sign: i32 = 1;
    let mut first = true;
    let mut retval: i32 = 0;
    let mut overflow;
    println!("sschar={:?}", sschars);
    loop {
        match sschars.next() {
            Some(x)  => {
                match x {
                    '-' => {
                        if first {
                            sign = -1;
                            first = false;
                        } else {
                            break;
                        }
                    },
                    '+' => {
                        if first {
                            sign = 1;
                            first = false;
                        } else {
                            break;
                        }
                    },
                    '0'..='9' => {
                        println!("PRE:: retval={}", retval);
                        first = false;
                        (retval, overflow) = retval.overflowing_mul(10);
                        if overflow {
                            if sign == 1 {
                                return i32::MAX;
                            } else {
                                return i32::MIN
                            }
                        }
                        println!("INTER:: retval={}", retval);
                        let num = x.to_digit(10).unwrap() as i32;
                        (retval, overflow) = retval.overflowing_add(num);
                        if overflow {
                            if sign == 1 {
                                return i32::MAX;
                            } else {
                                return i32::MIN
                            }
                        }
                        println!("POST:: retval={}", retval);
                    },
                    _ => break
                }
            },
            None => break
        }
    }
    retval * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }
    #[test]
    fn ex2() {
        assert_eq!(my_atoi("-042".to_string()), -42);
    }
    #[test]
    fn ex3() {
        assert_eq!(my_atoi("1337c0d3".to_string()), 1337);
    }
    #[test]
    fn ex4() {
        assert_eq!(my_atoi("0-1".to_string()), 0);
    }
    #[test]
    fn ex5() {
        assert_eq!(my_atoi("words and 987".to_string()), 0);
    }
    
    #[test]
    fn ex6() {
        assert_eq!(my_atoi("2147483648".to_string()), 2147483647);
    }
}
