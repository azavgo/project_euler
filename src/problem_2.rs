//https://projecteuler.net/problem=2

pub fn problem_2(lmt: usize) -> usize {
    let mut a: usize = 1;
    let mut b = 2;
    
    let mut c;
    let mut s = 2;

    loop {
        c = a + b;
        if c > lmt {
            break;
        } else {
            match c%2 {
                0 => {
                    s += c; 
                    a = b; 
                    b = c;
                }
                _ => {
                    a = b; 
                    b = c;
                }
            }
        } 
    }
    s
}
