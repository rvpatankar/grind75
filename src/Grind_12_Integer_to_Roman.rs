pub fn int_to_roman(num: i32) -> String {
    let mut retval = "".to_string();
    if num < 0 && num > 4000 {
        return retval;
    }
    let mut val = num;
    //let mut num_vec = vec![];
    let mut pos = 1;
    let mut pos_val = 0;
    let mut rom_val: String;
    loop {
        pos_val = val % 10;
        retval = pos_roman(pos_val, pos) + &retval;
        /*
        match pos {
            1 => {
                println!("ones={}", pos_val);
                println!("ones_r={}", pos_roman(pos_val, pos));
            }, 
            2 => {
                println!("tens={}", pos_val);
                println!("tens_r={}", pos_roman(pos_val, pos));
            }, 
            3 => {
                println!("hundreds={}", pos_val);
                println!("hundreds_r={}", pos_roman(pos_val, pos));
            }, 
            4 => {
                println!("thousands={}", pos_val);
                println!("thousands_r={}", pos_roman(pos_val, pos));
            },
            _ => println!("out of bounds")
        }
        */
        val = val / 10;
        pos += 1;
        if val == 0 {
            break;
        }
    }
    //println!("numvec = {:?}", num_vec);

    retval
}

fn pos_roman(num: i32, pos: i32) -> String {
    match num {
        4 => {
            match pos {
                1 => return "IV".to_string(),
                2 => return "XL".to_string(),
                3 => return "CD".to_string(),
                _ => return "".to_string()
            }
        },
        9 => {
            match pos {
                1 => return "IX".to_string(),
                2 => return "XC".to_string(),
                3 => return "CM".to_string(),
                _ => return "".to_string()
            }
        },
        5 => {
            match pos {
                1 => return "V".to_string(),
                2 => return "L".to_string(),
                3 => return "D".to_string(),
                _ => return "".to_string()
            }
        },
        1|2|3 => {
            match pos {
                1 => return (0..num).map(|_| "I").collect::<String>(),
                2 => return (0..num).map(|_| "X").collect::<String>(),
                3 => return (0..num).map(|_| "C").collect::<String>(),
                4 => return (0..num).map(|_| "M").collect::<String>(),
                _ => return "".to_string()
            }
        },
        6|7|8 => {
            match pos {
                1 => return "V".to_string() + &(0..(num-5)).map(|_| "I").collect::<String>(),
                2 => return "L".to_string() + &(0..(num-5)).map(|_| "X").collect::<String>(),
                3 => return "D".to_string() + &(0..(num-5)).map(|_| "C").collect::<String>(),
                _ => return "".to_string()
            }
        },
        _ => return "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
    }
    
    #[test]
    fn ex2() {
        assert_eq!(int_to_roman(58), "LVIII");
    }
    
    #[test]
    fn ex3() {
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}
