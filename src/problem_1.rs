//https://projecteuler.net/problem=1

pub fn problem_1(lmt: usize) -> usize {
    let mut s = 0; 
    let mut n = 0; 

    loop{
        if n < lmt {
            if n % 3 == 0 || n % 5 == 0 {
                s += n;
                n += 1;
            } else {
                n += 1;
            }
        } else {
            break;
        }
    }
    s
}  