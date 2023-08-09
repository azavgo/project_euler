//https://projecteuler.net/problem=4

pub fn all_nines(n: usize) -> usize {
    let v: Vec<usize> = vec![9; n];
    let mut m: usize = 0;
    for i in 0..n {
        m += v[i] * power(10, i);
    }
    m
}

pub fn power(n: usize, m: usize) -> usize {
    let mut output: usize = 1; 
    for i in 0..m {
        output *= n; 
    }  
    output
}

pub fn is_palindromic_number(n: usize) -> bool {
    let n_s = n.to_string();
    let n_vec: Vec<char> = n_s.chars().collect();
    let n_ver_rev: Vec<char> = n_s.chars().rev().collect();
    if n_vec == n_ver_rev {
        true
    } else {
        false
    }
}

pub fn problem_4(n: usize) -> usize { 
    let mut a = all_nines(n); 
    let mut b = all_nines(n);
    let mut c = a * b; 
    //TODO
    while is_palindromic_number(c) != true {
        a -= 1;
        b -= 1; 
        c = a * b;  
    };
    unimplemented!();
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_nines_2() {
        assert_eq!(all_nines(2), 99);
    }

    #[test]
    fn test_all_nines_5() {
        assert_eq!(all_nines(5), 99999);
    }
    #[test]
    fn test_problem_4_9009() {
        assert_eq!(problem_4(2), 9009);
    }

    #[test]
    fn test_is_palindromic_number_808() {
        assert_eq!(is_palindromic_number(808), true);
    }

    #[test]
    fn test_is_palindromic_number_9009() {
        assert_eq!(is_palindromic_number(9009), true);
    }

    #[test]
    fn test_is_palindromic_number_35() {
        assert_eq!(is_palindromic_number(35), false);
    }

    #[test]
    fn test_power_10_4() {
        assert_eq!(power(10, 4), 10000);
    }

    #[test]
    fn test_power_2_3() {
        assert_eq!(power(2, 3), 8);
    }
}
