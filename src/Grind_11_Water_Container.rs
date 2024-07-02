pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_a = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut h = 0;
    let mut w = 0;
    let mut area = 0;
    while left < right {
        //print!("height[{}]={}, height[{}]={}, ", left, height[left], right, height[right]);
        w = right -left;
        if height[left] < height[right] {
            h = height[left];
            left += 1;
        } else {
            h = height[right];
            right -= 1;

        }
        area = h as i32 * w as i32;
        if max_a < area {
            max_a = area;
        }
        //println!("h={}, w={}, area={}, max_a={}", h, w, area, max_a);
    }
    max_a
}
// time complexity too high
/*
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_a:i32 = 0;
    let mut a:i32 = 0;
    let mut len:i32 = 0;
    for i in 0..height.len() {
        for j in i+1 .. height.len(){
            if height[i] > height[j]{
                len = height[j] as i32;
            } else {
                len = height[i] as i32;
            }
            a = (j - i) as i32 * len;
            println!("height[{}]={}, height[{}]={}, height-vec={}, a={}", i, height[i], j, height[j], len, a);
            if a > max_a {
                max_a = a;
            }
            println!("max_a={}", max_a);
        }
    }
    max_a
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
    
    #[test]
    fn ex2() {
        assert_eq!(max_area(vec![1,1]), 1);
    }
}
