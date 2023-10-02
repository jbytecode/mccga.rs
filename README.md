# mccga.rs
Machine-coded compact genetic algorithm in Rust

## In-short

The package implements the Machine-coded compact genetic algorithm defined in 

Satman, M. H. & Akadal, E. (2020). Machine Coded Compact Genetic Algorithms for Real Parameter Optimization Problems . Alphanumeric Journal , 8 (1) , 43-58 . DOI: 10.17093/alphanumeric.576919 [Link](https://dergipark.org.tr/en/pub/alphanumeric/issue/55603/576919)

## Example

Suppose that the objective function is to minimize 

```rust
fn f(x: &Vec<f64>) -> f64 {
    return (x[0] - 3.14159265).powf(2.0) + (x[1] - 2.71828).powf(2.0);
}
```

so the package allows to type 

```rust 
let mins: Vec<f64> = vec![-10000.0_f64; 2];
let maxs: Vec<f64> = vec![10000.0_f64; 2];

let result = mccga(f, &mins, &maxs, 0.001, 100000);
```

to minimize the objective function where result is a 2-element vector. One can test the result using

```rust
assert!(isequal(&result[0], 3.14159265, 0.001));
assert!(isequal(&result[1], 2.71828, 0.001)); 
```

## Other implementations

- Python (https://github.com/jbytecode/mccga.py)
- Julia (https://github.com/jmejia8/Metaheuristics.jl)
- Java (https://github.com/jbytecode/mccga.java)

