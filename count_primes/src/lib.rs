///Count the primes upto a given number n

pub fn count_primes(n: usize) -> usize {
    let mut primes = vec![true; n + 1];
    let mut i = 2;
    while i * i < primes.len() {
        if primes[i] {
            let mut j = i;
            while i * j < primes.len() {
                primes[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
    println!("{:?}", primes);
    primes[2..].iter().filter(|p| true == **p).count()
}
#[cfg(test)]
mod tests {
    use super::count_primes;
    #[test]
    fn test_count_primes_10() {
        assert_eq!(count_primes(10), 4);
    }
    #[test]
    fn test_count_primes_17() {
        assert_eq!(count_primes(17), 7);
    }
    #[test]
    fn test_count_primes_23() {
        assert_eq!(count_primes(23), 9);
    }
}
