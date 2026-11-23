/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn quick_sort<T: std::cmp::PartialOrd + Copy>(array: &mut [T], left: isize, right: isize) {
    if left >= right {
        return ;
    }
    
    // index必须为usize
    let pivot = array[(left + (right - left) / 2) as usize];
    let mut i = left - 1;
    let mut j = right + 1;
    
    loop {
        loop {
            i += 1;
            if array[i as usize] >= pivot {
                break;
            }
        }
        loop {
            j -= 1;
            if array[j as usize] <= pivot {
                break;
            }
        }
        if i >= j {
            break;
        }

        // swap的参数为usize类型
        array.swap(i as usize, j as usize);
    }

    quick_sort(array, left, j);
    quick_sort(array, j+1, right);
}

fn sort<T: std::cmp::PartialOrd + std::marker::Copy>(array: &mut [T]){
	quick_sort(array, 0, (array.len()-1) as isize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}