pub fn convert(s: String, num_rows: i32) -> String {
    let sb = s.as_bytes();

    let mut i:usize = 0;
    let mut row:usize = 0;
    let mut col:usize = 0;
    let mut dim1: Vec<String> = vec!["".to_string(); num_rows as usize];
    loop {
        if i > sb.len()-1 {
            break;
        }
        println!("PRE->row:{}, col:{}, char:{}", row, col, (sb[i] as char));
        if num_rows == 1 || col % (num_rows as usize-1) == 0 {
            if row < num_rows as usize {
                println!("R->row:{}, col:{}, char:{}", row, col, (sb[i] as char));
                dim1[row].push_str((sb[i] as char).to_string().as_str());
                if num_rows - 1 != 0 {
                    row+=1;
                }
            } else {
                col += 1;
                row -= 2;
                println!("RMID->row:{}, col:{}, char:{}", row, col, (sb[i] as char));
                dim1[row].push_str((sb[i] as char).to_string().as_str());
                col+=1;
                if row > 0 {
                    row-=1;
                } else {
                    row = 1;
                }
            }
        }
         else {
            dim1[row].push_str((sb[i] as char).to_string().as_str());
            println!("C->row:{}, col:{}, char:{}", row, col, (sb[i] as char));
            col+=1;
            row-=1;
        }
        println!("POST->row:{}, col:{}, char:{}", row, col, (sb[i] as char));
        i+=1;
    }
    let mut retval = "".to_string();
    for i in 0..dim1.len() {
        retval = retval + dim1[i].as_str();
    }

    println!("{:?}", dim1);
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
    }

    #[test]
    fn ex2() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
    }

    #[test]
    fn ex3() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 5), "PHASIYIRPLIGAN".to_string());
    }
    
    #[test]
    fn ex4() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 6), "PRAIIYHNPSGAIL".to_string());
    }

    #[test]
    fn ex5() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 7), "PNAIGYRPIAHLSI".to_string());
    }

    #[test]
    fn ex6() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 2), "PYAIHRNAPLSIIG".to_string());
    }

    #[test]
    fn ex7() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 1), "PAYPALISHIRING".to_string());
    }

    #[test]
    fn ex8() {
        assert_eq!(convert("A".to_string(), 1), "A".to_string());
    }
}
