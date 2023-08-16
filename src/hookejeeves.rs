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
