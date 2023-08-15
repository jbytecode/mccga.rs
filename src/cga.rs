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

pub fn update(probvector: &Vec<f64>, winner: &Vec<u8>, loser: &Vec<u8>, mutrate: f64) -> Vec<f64> {
    let n = probvector.len();
    let mut newvector = vec![0.0_f64; n];
    for i in 0..n {
        if winner[i] == loser[i] {
            newvector[i] = probvector[i];
        } else {
            if winner[i] == 0 {
                newvector[i] = (probvector[i] - mutrate).max(0.0_f64);
            }else {
                newvector[i] = (probvector[i] + mutrate).min(1.0_f64);
            }
        }
    }
    return newvector;
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