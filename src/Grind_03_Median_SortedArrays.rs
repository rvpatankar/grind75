pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut res: Vec<i32> = Vec::new();
    let mut median: f64 = 0.0;
    loop {
        if i1<nums1.len() && i2<nums2.len() {
            if nums1[i1] <= nums2[i2] {
                res.push(nums1[i1]);
                i1+=1;
            } else {
                res.push(nums2[i2]);
                i2+=1;
            }
        } else if i1<nums1.len() && i2>=nums2.len() {
            res.push(nums1[i1]);
            i1+=1;
        } else if i1>=nums1.len() && i2<nums2.len() {
            res.push(nums2[i2]);
            i2+=1;
        } else {
            break;
        }    
    }
    println!("res={:?}", res);
    println!("x={}", res.len()/2);
    if res.len() % 2 == 0 {
        let x = res.len() / 2;
        let sum = res[x-1] as f64 + res[x] as f64;
        median = sum / 2.0;
    } else {
        let x = res.len() / 2;
        median = res[x] as f64;
    }
    median
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
    }

    #[test]
    fn ex2() {
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
    }
}
