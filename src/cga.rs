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
pub fn update(probvector: &mut Vec<f64>, winner: &Vec<u8>, loser: &Vec<u8>, mutrate: f64) -> () {
    for i in 0..probvector.len() {
        if winner[i] != loser[i] {
            if winner[i] == 0 {
                probvector[i] = (probvector[i] - mutrate).max(0.0_f64);
            }else {
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
        }else{
            break;
        }
    }
    return satisfied == n;
}