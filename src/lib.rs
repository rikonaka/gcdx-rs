fn gcd_euclidean(a: usize, b: usize) -> usize {
    let mut ax = a;
    let mut bx = b;
    let mut tx = b;
    loop {
        let check = ax % bx;
        if check != 0 {
            tx = check;
            ax = bx;
            bx = tx;
        } else {
            break;
        }
    }
    tx
}

fn gcd_stein(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    let shift = (a | b).trailing_zeros();

    a >>= a.trailing_zeros();
    b >>= b.trailing_zeros();

    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }

    a << shift
}

fn gcd_recursion(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd_recursion(b, a % b)
    }
}

enum GCDMethods {
    Euclidean,
    Stein,
    Recursion,
}

fn gcd_multiple(array: &[usize], method: GCDMethods) -> Option<usize> {
    let zero_check = |inputs: &[usize]| -> bool {
        for v in inputs {
            if *v == 0 {
                return true;
            }
        }
        false
    };

    if array.len() > 0 {
        if zero_check(array) {
            Some(0)
        } else {
            let mut m = array[0];
            for i in 1..array.len() {
                m = match method {
                    GCDMethods::Euclidean => gcd_euclidean(m, array[i]),
                    GCDMethods::Stein => gcd_stein(m, array[i]),
                    GCDMethods::Recursion => gcd_recursion(m, array[i]),
                };
            }
            Some(m)
        }
    } else {
        None
    }
}

/// Calculate the greatest common divisor using the Stein method.
pub fn gcdx_stein(array: &[usize]) -> Option<usize> {
    gcd_multiple(array, GCDMethods::Stein)
}

/// Calculate the greatest common divisor using the Euclidean method.
pub fn gcdx_euclidean(array: &[usize]) -> Option<usize> {
    gcd_multiple(array, GCDMethods::Euclidean)
}

/// Calculate the greatest common divisor using the Recursion method.
pub fn gcdx_recursion(array: &[usize]) -> Option<usize> {
    gcd_multiple(array, GCDMethods::Recursion)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcdx_recursion() {
        let v: Vec<usize> = vec![0, 1, 2, 3, 4];
        let g = gcdx_recursion(&v).unwrap();
        assert_eq!(g, 0);

        let v: Vec<usize> = vec![120, 168, 328, 624, 320];
        let g = gcdx_recursion(&v).unwrap();
        assert_eq!(g, 8);
    }
    #[test]
    fn test_gcdx_stein() {
        let v: Vec<usize> = vec![0, 1, 2, 3, 4];
        let g = gcdx_stein(&v).unwrap();
        assert_eq!(g, 0);

        let v: Vec<usize> = vec![120, 168, 328, 624, 320];
        let g = gcdx_stein(&v).unwrap();
        assert_eq!(g, 8);
    }
    #[test]
    fn test_gcdx_euclidean() {
        let v: Vec<usize> = vec![0, 1, 2, 3, 4];
        let g = gcdx_euclidean(&v).unwrap();
        assert_eq!(g, 0);

        let v: Vec<usize> = vec![10];
        let g = gcdx_euclidean(&v).unwrap();
        assert_eq!(g, 10);

        let v: Vec<usize> = vec![10, 9];
        let g = gcdx_euclidean(&v).unwrap();
        assert_eq!(g, 1);

        let v: Vec<usize> = vec![10, 9, 8, 7];
        let g = gcdx_euclidean(&v).unwrap();
        assert_eq!(g, 1);

        let v: Vec<usize> = vec![120, 168, 328, 624, 320];
        let g = gcdx_euclidean(&v).unwrap();
        assert_eq!(g, 8);

        let v: Vec<usize> = vec![2228668932, 825805579, 1955783521, 1173124319, 1234171242];
        let g = gcdx_euclidean(&v).unwrap();
        assert_eq!(g, 1);
    }
}
