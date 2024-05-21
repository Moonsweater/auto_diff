use std::{ops, vec};

//TODO:

//Current state: need to implement copy trait for idiomatic expressions.
//That means, we need generic arrays in the struct. Implemented via a crate.

//Implement all ementary functions
//Turn into an actual library
//Get good idiom: make things feel as close as possible to just doing arithmetic on numbers
//Research different AD techniques. (multivariable? forward vs. backward?)
//Implement wackier functions (SI, EI, sinc, etc.)
//See if it works on complex-valued functions???
    //Esp. for functions which are meromorphic but holomorphic.

//#[derive(Clone, Copy)]
//Fails due to generics, or something. Implement manually.\
struct AutoDiffFunction<const N: usize> {
    taylor_coefficients: [f64; N],
    evaluated_at: f64,
    err: bool
}

impl<const N: usize> Clone for AutoDiffFunction<{N}> {
    fn clone(&self) -> Self {
        AutoDiffFunction {
            taylor_coefficients: self.taylor_coefficients.clone(),
            evaluated_at: self.evaluated_at,
            err: self.err
        }
    }
}

impl<const N: usize> Copy for AutoDiffFunction<{N}>{}

type ADF<const N: usize> = AutoDiffFunction<N>;

fn adf_print<const N: usize>(a: ADF<{N}>) {
    for k in 0..N {
        print!("{:<4}", a.taylor_coefficients[k]);
    }
    println!("");
    println!("taylor depth: {}", N);
    println!("evaluated_at: {}", a.evaluated_at);
    println!("err: {}", a.err);
}

fn adf_for_binary_operation<const N: usize>(lhs: ADF<{N}>, rhs: ADF<{N}>, coefficients: Vec<f64>) -> ADF<{N}> {

    //This one should still take references, actually.

    let err: bool = {
        if lhs.evaluated_at != rhs.evaluated_at {
            println!("Error: mismatched values for evaluated_at.");
            //Need actual error handling here.
            true
        }
        else {false}
    };
    AutoDiffFunction {
        taylor_coefficients: {
            let mut out = [0.0;N];
            for i in 0..coefficients.len() {
                out[i] = coefficients[i];
            }
            out
        },
        evaluated_at: lhs.evaluated_at,
        err: err
    }
}

//Operators

impl<const N: usize> ops::Add for ADF<{N}> {

    type Output = ADF<{N}>;

    fn add(self, rhs: ADF<{N}>) -> Self::Output {
        let mut out_coeffs = vec![];
        for k in 0..N {
            out_coeffs.push(self.taylor_coefficients[k] + rhs.taylor_coefficients[k]);
        }
        adf_for_binary_operation(self, rhs, out_coeffs)
    }
}

impl<const N: usize> ops::Mul for ADF<{N}>  {
    
    //SAFE.
    
    type Output = ADF<{N}>;
    
    fn mul(self, rhs: ADF<{N}>) -> Self::Output {
        let mut out_coeffs = vec![];
        let lhs_coeffs = self.taylor_coefficients.clone();
        for k in 0..N{
            let mut sum = 0.0;
            for i in 0..=k {
                sum += lhs_coeffs[k-i] * rhs.taylor_coefficients[i];
            }
            out_coeffs.push(sum);
        }
        adf_for_binary_operation(self, rhs, out_coeffs)
    }
}

impl<const N: usize> ops::Div for ADF<{N}>  {
    
    //SAFE.
    
    type Output = ADF<{N}>;
    
    fn div(self, rhs: ADF<{N}>) -> Self::Output {
        let mut out_coeffs = vec![];
        //let lhs_coeffs = self.taylor_coefficients.clone();
        let mut err = false;
        for k in 0..N {
            let mut denom: f64 = 0.0;
            for i in k..=0 {
                if rhs.taylor_coefficients[i] != 0.0 {
                    denom = rhs.taylor_coefficients[i];
                    break;
                }
            }
            if denom == 0.0 {
                //Make this an actual error message.
                println!("Division by a zero function.");
                err = true;
            }
            let mut sum = 0.0;
            for i in 0..k {
                sum += out_coeffs[i] * rhs.taylor_coefficients[k-i];
            }
            sum = self.taylor_coefficients[k] - sum;
            out_coeffs.push(sum / denom);
        }
        let mut out = adf_for_binary_operation(self, rhs, out_coeffs);
        out.err = err;
        out
    }
}

//Initial functions

fn autodiff_identity<const N: usize>(evaluated_at: f64) -> ADF<N> {

    let mut coeffs = [0.0; N];
    coeffs[0] = evaluated_at;
    coeffs[1] = 1.0;

    AutoDiffFunction {
        taylor_coefficients: coeffs,
        evaluated_at: evaluated_at,
        err: false
    }
}

fn autodiff_const<const N: usize>(evaluated_at: f64) -> ADF<{N}> {
    let mut coeffs = [0.0; N];
    coeffs[0] = 1.0;

    AutoDiffFunction {
        taylor_coefficients: coeffs,
        evaluated_at: evaluated_at,
        err: false
    }
}

fn main() {

    const DEPTH: usize = 10;
    let x = 3.0;

    let mut f = autodiff_identity::<{DEPTH}>(x);
    let g = autodiff_const::<{DEPTH}>(x);
    let mut h = autodiff_identity::<{DEPTH}>(x);
    
    adf_print(f);
    
    f = f+g;
    h = (h * h) + h; //x*2 + x

    adf_print(f);
    adf_print(h);

}

