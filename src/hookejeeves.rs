
fn clone(x: &Vec<f64>) -> Vec<f64> {
    let n: usize = x.len();
    let mut result: Vec<f64> = vec![0.0_f64; n];
    for i in 0..n {
        result[i] = x[i];
    }
    return result;
}

fn mutate(par: &Vec<f64>, p: usize, d: f64) -> Vec<f64> {
    let mut newpar: Vec<f64> = clone(&par);
    newpar[p - 1] += d;
    return newpar;
}

pub fn hj(
    f: fn(&Vec<f64>) -> f64,
    parv: Vec<f64>,
    maxiter: usize,
    startstep: f64,
    endstep: f64,
) -> Vec<f64> {
    let p = parv.len();
    let mut currentstep: f64 = startstep;
    let mut iter: usize = 0;
    let mut par: Vec<f64> = clone(&parv);
    while iter < maxiter {
        let fold = f(&par);
        let mut fnow: f64 = fold;
        for currentp in 1..=p {
            let mutateleft: Vec<f64> = mutate(&par, currentp, -currentstep);
            let fleft: f64 = f(&mutateleft);
            let mutateright: Vec<f64> = mutate(&par, currentp, currentstep);
            let fright: f64 = f(&mutateright);
            if fleft < fold {
                par = mutateleft;
                fnow = fleft;
            } else if fright < fold {
                par = mutateright;
                fnow = fright;
            }
        }

        if fold <= fnow {
            currentstep = currentstep / 2.0;
        }

        if currentstep < endstep {
            break;
        }

        iter += 1;
    } // end of while

    return par;
}




#[cfg(test)]
mod tests {

    use super::hj;
    use rand;

    #[test]
    fn test_hj_minimization() {
        fn cfmin(x: &Vec<f64>) -> f64 {
            let mut sum: f64 = 0.0;
            for i in 0..x.len() {
                sum += x[i] * x[i];
            }
            return sum;
        }
        let eps = 0.00001;
        let parlen = 20;
        let mut starter = vec![0.0_f64; parlen];
        for i in 1..parlen {
            starter[i] = rand::random();
        }
        let hjresult = hj(cfmin, starter, 10000, 1000.0, 0.000001);
        for i in 0..parlen {
            assert!((hjresult[i] -  0.0_f64).powf(2.0) <= eps);
        }
    }
}