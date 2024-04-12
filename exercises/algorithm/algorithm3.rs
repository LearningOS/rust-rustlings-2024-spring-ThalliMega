/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::Ord>(array: &mut [T]) {
    if array.is_empty() {
        return;
    }
    let mut start = 0;
    let mut end = array.len() - 1;

    while start < end {
        let stand = &array[0];
        while start < end && &array[end] >= stand {
            end -= 1;
        }
        while start < end && &array[start] <= stand {
            start += 1;
        }
        array.swap(start, end);
    }

    array.swap(0, start);
    sort(&mut array[..start]);
    sort(&mut array[start + 1..]);
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
