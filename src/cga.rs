use rand::random;

pub fn sample(probvector: &Vec<f64>) -> Vec<u8> {
    let n = probvector.len();
    let mut newvector = vec![0_u8; n];
    for i in 0..n {
        if random::<f64>() < probvector[i] {
            newvector[i] = 1_u8;
        }
    }
    return newvector;
}

///
/// In-place update of probability vector.
/// If ith bit of the winner and loser are equal than does nothing.
/// If ith bit of the winner is one then increase the corresponding probability by a small number (mutrate).
/// If ith bit of the winner is zero then decrease the corresponding probability by a small number.
/// The function mutates the values in place.
pub fn update(probvector: &mut Vec<f64>, winner: &Vec<u8>, loser: &Vec<u8>, mutrate: f64) -> () {
    for i in 0..probvector.len() {
        if winner[i] != loser[i] {
            if winner[i] == 0 {
                probvector[i] = (probvector[i] - mutrate).max(0.0_f64);
            } else {
                probvector[i] = (probvector[i] + mutrate).min(1.0_f64);
            }
        }
    }
}

pub fn isequal(x: &f64, other: f64, mutrate: f64) -> bool {
    return (x - other).abs() <= mutrate;
}

pub fn isconverged(probvector: &Vec<f64>, mutrate: f64) -> bool {
    let n = probvector.len();
    let mut satisfied: usize = 0;
    for i in 0..n {
        let val = probvector[i];
        if isequal(&val, 0.0, mutrate) || isequal(&val, 1.0, mutrate) {
            satisfied += 1;
        } else {
            break;
        }
    }
    return satisfied == n;
}

pub fn single_iteration(probvector: &mut Vec<f64>, fcost: fn(&Vec<u8>) -> f64, mutrate: f64) -> () {
    let candidate1: Vec<u8> = sample(&probvector);
    let candidate2: Vec<u8> = sample(&probvector);

    let cost1: f64 = fcost(&candidate1);
    let cost2: f64 = fcost(&candidate2);

    let mut winner: &Vec<u8> = &candidate1;
    let mut loser: &Vec<u8> = &candidate2;

    if cost2 < cost1 {
        winner = &candidate2;
        loser = &candidate1;
    }

    update(probvector, winner, loser, mutrate);
}

pub fn cga(fcost: fn(&Vec<u8>) -> f64, mutrate: f64, bitlen: usize) -> Vec<u8> {
    let mut probvector: Vec<f64> = vec![0.5_f64; bitlen];

    while !isconverged(&probvector, mutrate) {
        single_iteration(&mut probvector, fcost, mutrate);
    }

    let result: Vec<u8> = sample(&probvector);

    return result;
}

#[cfg(test)]
mod tests {

    use super::cga;
    use super::isconverged;
    use super::isequal;
    use super::update;

    #[test]
    fn test_update() {
        let n: usize = 5;
        let mutrate: f64 = 0.1;
        let winner: Vec<u8> = vec![1_u8; n];
        let loser: Vec<u8> = vec![0_u8; n];
        let mut probvector: Vec<f64> = vec![0.5_f64; n];

        update(&mut probvector, &winner, &loser, mutrate);

        for i in 0..n {
            assert_eq!(probvector[i], 0.6_f64);
        }
    }

    #[test]
    fn test_isequal() {
        // Equal
        assert!(isequal(&10.0, 9.99, 0.01));

        // Not equal
        assert!(!isequal(&10.0, 9.9, 0.01));
    }

    #[test]
    fn test_isconverged() {
        // Converged
        let v1: Vec<f64> = vec![1.0, 0.0, 0.0, 1.0];
        assert!(isconverged(&v1, 0.0001));

        // Converged
        let v2: Vec<f64> = vec![0.9999999, 0.0, 0.0, 1.0];
        assert!(isconverged(&v2, 0.0001));

        // Converged
        let v3: Vec<f64> = vec![0.99999, 0.0, 0.0, 1.0];
        assert!(isconverged(&v3, 0.00001));

        // Not converged
        let v4: Vec<f64> = vec![0.9, 0.0, 0.0, 1.0];
        assert!(!isconverged(&v4, 0.0001));
    }

    #[test]
    fn test_cga_maximization() {
        fn cfmax(x: &Vec<u8>) -> f64 {
            let mut sum: f64 = 0.0;
            for i in 0..x.len() {
                sum += x[i] as f64;
            }
            return -sum;
        }
        let bitlen = 20;
        let cgaresult = cga(cfmax, 0.01, bitlen);
        for i in 0..bitlen {
            assert_eq!(cgaresult[i], 1_u8);
        }
    }

    #[test]
    fn test_cga_minimization() {
        fn cfmin(x: &Vec<u8>) -> f64 {
            let mut sum: f64 = 0.0;
            for i in 0..x.len() {
                sum += x[i] as f64;
            }
            return sum;
        }
        let bitlen = 20;
        let cgaresult = cga(cfmin, 0.01, bitlen);
        for i in 0..bitlen {
            assert_eq!(cgaresult[i], 0_u8);
        }
    }
}
