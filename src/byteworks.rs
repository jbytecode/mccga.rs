pub trait ByteWorks {
    fn to_bitvector(c: f32) -> Vec<u8>;
    fn from_bitvector(v: Vec<u8>) -> f32;
}

impl ByteWorks for f32 {
    fn to_bitvector(c: f32) -> Vec<u8> {
        let u32value: u32 = c.to_bits();
        let mut result: Vec<u8> = Vec::<u8>::with_capacity(32);
        let mut currentbit: u8;
        for i in 0..32 {
            currentbit = (((u32value >> i) as u8) & 1) as u8;
            result.push(currentbit);
        }
        return result;
    }

    fn from_bitvector(v: Vec<u8>) -> f32 {
        let mut u32value: u32 = 0;
        for i in 0..32 {
            u32value = u32value + (v[i] as u32) * (2_u32).pow(i as u32);
        }
        return f32::from_bits(u32value);
    }
}

pub fn floats2bits(v: &Vec<f32>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::<u8>::with_capacity(v.len() * 32);
    for val in v {
        let mut bitsoffloat = f32::to_bitvector(*val);
        result.append(&mut bitsoffloat);
    }
    return result;
}

pub fn bits2floats(v: &Vec<u8>) -> Vec<f32> {
    let bitsize: usize = v.len();
    let floatssize: usize = bitsize.wrapping_div(32);
    let mut floatvector: Vec<f32> = Vec::<f32>::with_capacity(floatssize);
    let mut index: usize = 0;
    while index + 32 <= bitsize {
        let part: Vec<u8> = v[index..(index + 32)].iter().cloned().collect();
        floatvector.push(f32::from_bitvector(part));
        index = index + 32;
    }
    return floatvector;
}

pub fn floatstodoubles(x: &Vec<f32>) -> Vec<f64> {
    let n = x.len();
    let mut newvector: Vec<f64> = vec![0.0_f64; n];
    for i in 0..n {
        newvector[i] = x[i] as f64;
    }
    return newvector;
}


#[cfg(test)]
mod tests {
    use super::{bits2floats, floats2bits};
    use super::ByteWorks;

    #[test]
    fn bit_f32_conversation() {
        let val: f32 = 3.14159265_f32;
        let bits: Vec<u8> = f32::to_bitvector(val);
        let otherval: f32 = f32::from_bitvector(bits);
        assert_eq!(val.to_bits(), otherval.to_bits());
        assert_eq!(val, otherval);
    }

    #[test]
    fn test_floats2bits() {
        let v1: Vec<u8> = vec![
            1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 1, 0,
        ];
        let floatvector = vec![3.14159265_f32, 3.14159265_f32];
        let floatsbits: Vec<u8> = floats2bits(&floatvector);
        assert_eq!(floatsbits, v1);
    }

    #[test]
    fn test_bits2floats() {
        let v1: Vec<u8> = vec![
            1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 1, 0,
        ];
        let floats: Vec<f32> = bits2floats(&v1);
        assert_eq!(floats.len(), 2);
        assert_eq!(floats[0], 3.14159265);
        assert_eq!(floats[1], 3.14159265);
    }

    #[test]
    fn test_autotestforconversations() {
        let fvals: Vec<f32> = vec![1.2_f32, 5.6_f32, 78.9998_f32];
        let bits: Vec<u8> = floats2bits(&fvals);
        let floatsback: Vec<f32> = bits2floats(&bits);
        assert_eq!(floatsback.len(), 3);
        assert_eq!(fvals[0], floatsback[0]);
        assert_eq!(fvals[1], floatsback[1]);
        assert_eq!(fvals[2], floatsback[2]);
    }
}
