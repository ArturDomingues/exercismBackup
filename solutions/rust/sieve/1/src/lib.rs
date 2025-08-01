pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; (upper_bound + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt = (upper_bound as f64).sqrt() as u64;
    for i in 2..=sqrt {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= upper_bound {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }

    (2..=upper_bound)
        .filter(|&n| is_prime[n as usize])
        .collect()
}
