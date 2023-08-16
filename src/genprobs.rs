

use crate::byteworks::*;

use rand::random;

fn getrandomvectorbetween(mins: &Vec<f64>, maxs: &Vec<f64>) -> Vec<f32> {
    let n = mins.len();
    let mut result: Vec<f32> = vec![0.0_f32; n];
    for i in 0..n {
        result[i] = (mins[i] as f32) + random::<f32>() * (maxs[i] as f32 - mins[i] as f32);
    }
    return result;
}

pub fn generate_probability_vector(mins: &Vec<f64>, maxs: &Vec<f64>, ntries: usize) -> Vec<f64> {
    let nbits: usize = mins.len() * 32;
    let mutrate: f64 = (1.0_f64) / (ntries as f64);
    let mut probvector: Vec<f64> = vec![0.0_f64; nbits];
    for _ in 0..ntries {
        let floats: Vec<f32> = getrandomvectorbetween(mins, maxs);
        let floatbits: Vec<u8> = floats2bits(&floats);
        for k in 0..nbits {
            if floatbits[k] == 1 {
                probvector[k] = probvector[k] + mutrate;
            }
        }
    }

    return probvector;
}

#[cfg(test)]
mod tests {
    use super::generate_probability_vector;

    #[test]
    fn test_generateprobabilityvector() {
        let mins = vec![0.0_f64, 0.0_f64];
        let maxs = vec![10.0_f64, 10.0_f64];
        let prob = generate_probability_vector(&mins, &maxs, 10000);
        for i in 0..prob.len() {
            assert!(prob[i] <= 1.0);
            assert!(prob[i] >= 0.0);
        }
    }
}
