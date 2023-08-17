pub mod byteworks;
pub mod genprobs;
pub mod cga;
pub mod hookejeeves;


use crate::byteworks::{bits2floats, floatstodoubles};
use crate::cga::{isconverged, sample, update};
use crate::genprobs::generate_probability_vector;
use crate::hookejeeves::hj;

fn binarycost(fcost: fn(&Vec<f64>) -> f64, candidate: &Vec<u8>) -> f64 {
    let doubles = floatstodoubles(&bits2floats(candidate));
    let result = fcost(&doubles);
    return result;
}

pub fn mccga_singleiter(probvect: &mut Vec<f64>, fcost: fn(x: &Vec<f64>)->f64, mutrate: f64) -> (){
    let candidate1: Vec<u8> = sample(&probvect);
    let candidate2: Vec<u8> = sample(&probvect);
    let cost1: f64 = binarycost(fcost, &candidate1);
    let cost2: f64 = binarycost(fcost, &candidate2);
    let mut winner: &Vec<u8> = &candidate1;
    let mut loser: &Vec<u8> = &candidate2;
    if cost2 < cost1 {
        winner = &candidate2;
        loser = &candidate1;
    }
    update(probvect, winner, loser, mutrate);
}

/// Optimize f: R^n -> R type functions in n-dimensional real space.
/// The direction of optimization is supposed to be a minimization.
/// Maximization-typed objective functions can return the negative of the output value.
/// Suppose that the objective function is 
/// ```
/// fn f(x: &Vec<f64>) -> f64 {
///    return (x[0] - 3.14159265).powf(2.0) + (x[1] - 2.71828).powf(2.0);
/// }
/// ```
/// The function is minimized for `x[0] = 3.14159265` and `x[1] = 2.71828`.
/// The function mccga (Machine-coded compact genetic algorithms) is designed to find
/// a solution to that kind of problems.
/// The method initially generates a probability vector using the range of decision variables.
/// Each single element of this probability vector defines the probability of p(b) = 1.0 where
/// b is the corresponding bit. 
/// The algorithm feeds this initial vector to a standard compact algorithm (CGA). In CGA stage
/// of the algorithm, bitwise operations are performed on the IEEE-754 representations of the 
/// 32-bit real values (f32). 
/// In the final stage, the output is fed to a Hooke-Jeeves algorithm for a direct search and 
/// fine-tuning. Finally the algorithm outputs a good approximation for the minimum of the function.
/// Here is the documentation for the details:
/// 
/// Satman, M. H. & Akadal, E. (2020). Machine Coded Compact Genetic Algorithms for Real Parameter Optimization Problems . Alphanumeric Journal , 8 (1) , 43-58 . DOI: 10.17093/alphanumeric.576919
/// 
/// 
/// Here is a example of optimizing the function defined above:
/// ```
/// use mccga::mccga;
/// fn f(x: &Vec<f64>) -> f64 {
///    return (x[0] - 3.14159265).powf(2.0) + (x[1] - 2.71828).powf(2.0);
/// }
/// let mins: Vec<f64> = vec![-10000.0_f64; 2];
/// let maxs: Vec<f64> = vec![10000.0_f64; 2];
/// let result = mccga(f, &mins, &maxs, 0.001, 100000);
/// ```
/// 
pub fn mccga(
    fcost: fn(&Vec<f64>) -> f64,
    mins: &Vec<f64>,
    maxs: &Vec<f64>,
    mutrate: f64,
    maxiter: usize,
) -> Vec<f64> {
    let mut probvect = generate_probability_vector(mins, maxs, 500000);

    let mut iter = 0;

    while iter < maxiter {
        mccga_singleiter(&mut probvect, fcost, mutrate);

        if isconverged(&probvect, mutrate) {
            break;
        }

        iter = iter + 1;
    }

    let firstresult =  floatstodoubles(&bits2floats(&sample(&probvect)));
    
    let seconresult = hj(fcost, firstresult, 1000, 5.0, 0.00001);
    
    return seconresult;
}

#[cfg(test)]
mod tests {
    use super::mccga;
    use crate::cga::isequal;

    #[test]
    fn test_mccga() {
        fn f(x: &Vec<f64>) -> f64 {
            return (x[0] - 3.14159265).powf(2.0) + (x[1] - 2.71828).powf(2.0);
        }
        let mins: Vec<f64> = vec![-10000.0_f64; 2];
        let maxs: Vec<f64> = vec![10000.0_f64; 2];
        let result: Vec<f64> = mccga(f, &mins, &maxs, 0.001, 100000);
        assert!(isequal(&result[0], 3.14159265, 0.0001));
        assert!(isequal(&result[1], 2.71828, 0.0001)); 
    }


    #[test]
    fn test_mccga_5_paramateres() {
        const VALS: [f64; 5]= [7.0, 70.0, 700.0, 7000.0, 70000.0];
        fn f(x: &Vec<f64>) -> f64 {
            let mut sum = 0.0;
            for i in 0..x.len(){
                sum += (x[i] - VALS[i]).powf(2.0);
            }
            return sum;
        }
        let mins: Vec<f64> = vec![-100000.0_f64; 5];
        let maxs: Vec<f64> = vec![100000.0_f64; 5];
        let result: Vec<f64> = mccga(f, &mins, &maxs, 0.001, 100000);
        let delta: f64 = 0.01;
        println!("{:?}", result);
        for i in 0..result.len(){
            println!("Testing {} ?= {}", &result[i], VALS[i]);
            assert!(isequal(&result[i], VALS[i], delta));
        }
    }
}
