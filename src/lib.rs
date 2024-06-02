use std::fmt::Display;
use std::ops::Rem;

pub trait Zero {
    const ZERO: Self;
}
impl Zero for u8 {
    const ZERO: Self = 0;
}
impl Zero for u16 {
    const ZERO: Self = 0;
}
impl Zero for u32 {
    const ZERO: Self = 0;
}
impl Zero for u64 {
    const ZERO: Self = 0;
}
impl Zero for u128 {
    const ZERO: Self = 0;
}
impl Zero for usize {
    const ZERO: Self = 0;
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + PartialOrd + Zero + Copy + Display,
{
    let mut ax = a;
    let mut bx = b;
    let mut tx = b;
    loop {
        let check = ax % bx;
        if check != T::ZERO {
            tx = check;
            ax = bx;
            bx = tx;
        } else {
            break;
        }
    }
    tx
}

fn array_zero_check<T>(array: &[T]) -> bool
where
    T: Rem<Output = T> + PartialOrd + Zero + Copy + Display + Clone,
{
    for v in array {
        if *v == T::ZERO {
            return true;
        }
    }
    false
}

pub fn gcdx<T>(array: &[T]) -> Option<T>
where
    T: Rem<Output = T> + PartialOrd + Zero + Copy + Display + Clone,
{
    if array.len() > 0 {
        if array_zero_check(array) {
            return Some(T::ZERO);
        } else {
            let mut m = array[0];
            for i in 1..array.len() {
                m = gcd(m, array[i]);
            }
            Some(m)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcdx() {
        let v: Vec<usize> = vec![0, 1, 2, 3, 4];
        let g = gcdx(&v).unwrap();
        assert_eq!(g, 0);

        let v: Vec<usize> = vec![10];
        let g = gcdx(&v).unwrap();
        assert_eq!(g, 10);

        let v: Vec<usize> = vec![10, 9];
        let g = gcdx(&v).unwrap();
        assert_eq!(g, 1);

        let v: Vec<usize> = vec![10, 9, 8, 7];
        let g = gcdx(&v).unwrap();
        assert_eq!(g, 1);

        let v: Vec<usize> = vec![120, 168, 328, 624, 320];
        let g = gcdx(&v).unwrap();
        assert_eq!(g, 8);

        let v: Vec<usize> = vec![2228668932, 825805579, 1955783521, 1173124319, 1234171242];
        let g = gcdx(&v).unwrap();
        assert_eq!(g, 1);
    }
}
