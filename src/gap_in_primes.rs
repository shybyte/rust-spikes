fn gap_slow(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let primes = (m..n + 1).filter(|&x| is_prime(x)).collect::<Vec<_>>();
    primes.windows(2)
        .find(|prim| prim[1] - prim[0] == g as u64)
        .and_then(|pair| Some((pair[0], pair[1])))
}

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let mut primes = (m..n + 1).filter(|&x| is_prime(x));
    primes.next().and_then(|mut prev| {
        primes.find(|&prim|
            if prim - prev == g as u64 {
                true
            } else {
                prev = prim;
                false
            }
        ).and_then(|second| Some((prev, second)))
    })
}

// assumption x > 2
fn is_prime(x: u64) -> bool {
    let sqrt_x = (x as f64).sqrt() as u64;
    (2..sqrt_x + 1).all(|t| x % t != 0)
}


fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(gap(g, m, n), exp)
}

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(100), false);
    assert_eq!(is_prime(101), true);
    assert_eq!(is_prime(105), false);
}


#[test]
fn basics_gap() {
    testing(2, 100, 100, None);
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(6, 100, 110, None);
    testing(8, 300, 400, Some((359, 367)));
}
