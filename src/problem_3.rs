//https://projecteuler.net/problem=3

pub fn is_prime_number(n: usize) -> bool {
    let mut output = true;
    for i in 2 .. (n/2 + 1) { 
        match n%i {
            0 => {
                    output = false; 
                    break; 
                }, 
            _ => {},
        }      
    }
    output
}

pub fn problem_3(n: usize) -> usize { 
    let mut m = n; 
    let mut i = 2; 
    
    while m > 1 {
        while is_prime_number(i) != true {
            i += 1;
        }
        while m % i == 0 {
            m /= i;
        }
        i += 1;
    }
    i - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_number_7() {
        assert_eq!(is_prime_number(7), true);
    }

    #[test]
    fn test_prime_number_10() {
        assert_ne!(is_prime_number(10), true);
    }

    #[test]
    fn test_problem_3_29 () {
        assert_eq!(problem_3(29), 29);
    }

    #[test]
    fn test_problem_3_10 () {
        assert_eq!(problem_3(10), 5);
    }

    #[test]
    fn test_problem_3_13195 () {
        assert_eq!(problem_3(13195), 29);
    }

}
