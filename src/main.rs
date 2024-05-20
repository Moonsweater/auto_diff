use std::boxed::Box;
use std::{ops, vec};
use std::dbg;

//TODO:
//Implement all ementary functions
//Turn into an actual library
//Get good idiom: make things feel as close as possible to just doing arithmetic on numbers
//Research different AD techniques. (multivariable? forward vs. backward?)
//Implement wackier functions (SI, EI, sinc, etc.)
//See if it works on complex-valued functions???
    //Esp. for functions which are meromorphic but holomorphic.

struct AutoDiffFunction {
    taylor_coefficients: Vec<f64>,
    taylor_depth: usize,
    evaluated_at: f64,
    err: bool
}

type ADF = AutoDiffFunction;

fn adf_print(a: &ADF) {
    for k in 0..a.taylor_depth {
        print!("{:<4}", a.taylor_coefficients[k]);
    }
    println!("");
    println!("taylor_depth: {}", a.taylor_depth);
    println!("evaluated_at: {}", a.evaluated_at);
    println!("err: {}", a.err);
}

fn adf_for_binary_operation(lhs: &ADF, rhs: &ADF, coefficients: Vec<f64>) -> ADF {

    let err: bool = {
        if lhs.evaluated_at != rhs.evaluated_at {
            println!("Error: mismatched values for evaluated_at.");
            //Need actual error handling here.
            true
        }
        else {false}
    };

    let n:usize = {
        if lhs.taylor_depth <= rhs.taylor_depth {
            lhs.taylor_depth
        } else {
            rhs.taylor_depth
        }
    }; 
    ADF {
        taylor_coefficients: coefficients,
        taylor_depth: n,
        evaluated_at: lhs.evaluated_at,
        err: err
    }
}

//Operators

impl ops::Add<ADF> for ADF {
    
    //SAFE.

    type Output = ADF;

    fn add(self, rhs: ADF) -> Self::Output {

        let n:usize = {
            if self.taylor_depth <= rhs.taylor_depth {
                self.taylor_depth
            } else {
                rhs.taylor_depth
            }
        }; 

        let mut coeffs = vec![];
        for k in 0..n {
            coeffs.push(self.taylor_coefficients[k] + rhs.taylor_coefficients[k]);
        }
        adf_for_binary_operation(&self, &rhs, coeffs)
    }
}

impl ops::Mul<ADF> for ADF {
    
    //SAFE.
    
    type Output = ADF;
    
    fn mul(self, rhs: ADF) -> Self::Output {
        let n:usize = {
            if self.taylor_depth <= rhs.taylor_depth {
                self.taylor_depth
            } else {
                rhs.taylor_depth
            }
        }; 
        let mut coeffs = vec![];
        let lhs_coeffs = self.taylor_coefficients.clone();
        for k in 0..n {
            coeffs.push(lhs_coeffs[n-k] * rhs.taylor_coefficients[k]);
        }
        adf_for_binary_operation(&self, &rhs, coeffs)
    }
}



//Initial functions

fn autodiff_identity(evaluated_at: f64, taylor_depth: usize) -> ADF {

    let mut coeffs: Vec<f64> = vec![0.0; taylor_depth];
    coeffs[0] = evaluated_at;
    coeffs[1] = 1.0;

    ADF {
        taylor_coefficients: coeffs,
        taylor_depth: taylor_depth,
        evaluated_at: evaluated_at,
        err: false
    }
}

fn autodiff_const(evaluated_at: f64, taylor_depth: usize) -> ADF {
    let mut coeffs: Vec<f64> = vec![0.0; taylor_depth];
    coeffs[1] = 1.0;

    ADF {
        taylor_coefficients: coeffs,
        taylor_depth: taylor_depth,
        evaluated_at: evaluated_at,
        err: false
    }
}

fn main() {
    let depth: usize = 10;
    let x = 3.0;
    let mut f = autodiff_identity(x, depth);
    let g = autodiff_const(x, depth);
    adf_print(&f);
    f = f+g;
    adf_print(&f);
}

