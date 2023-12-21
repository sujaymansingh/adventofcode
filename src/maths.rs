/*
 * def gcd(a, b):
    """Return greatest common divisor using Euclid's Algorithm."""
    while b:
        a, b = b, a % b
    return a

def lcm(a, b):
    """Return lowest common multiple."""
    return a * b // gcd(a, b)

def lcmm(*args):
    """Return lcm of args."""
    return reduce(lcm, args)
[712, 157, 96, 591, 187, 100]
>>> import math
>>> math.lcm(*nums)
1235403232800
*/

use num::{integer, Integer};

pub fn lcm<T: Integer + Copy>(nums: &[T]) -> Option<T> {
    let mut num_iter = nums.iter();

    let mut result = *num_iter.next()?;

    for x in num_iter {
        result = integer::lcm(result, *x);
    }
    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_calculate_lcm() {
        let nums: Vec<u64> = vec![712, 157, 96, 591, 187, 100];
        assert_eq!(lcm(&nums).unwrap(), 1235403232800);
    }
}
