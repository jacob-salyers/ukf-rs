
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate nalgebra;


use nalgebra::base::{DMatrix,DVector};
use nalgebra::linalg::{Cholesky};

struct UKF {

}

struct MerweScaledPoints{

    n: u32,
    alpha: f64,
    beta: f64,
    kappa: f64,
}    

impl MerweScaledPoints {

    fn num_sigmas(&self) -> u32 {
        2*self.n+1
    }

    fn sigma_points(&self, x: f64, P: DMatrix<f64>) {
        
        let lambda = self.alpha.powi(2) * (f64::from(self.n) + self.kappa) - f64::from(self.n);
        let u = (lambda + f64::from(self.n)).sqrt();
    }
}

//TODO Create test suite in bash
fn unscented_transform(sigmas: DMatrix<f64>,
                       Wm: DVector<f64>,
                       Wc: DVector<f64>) -> (f64,DMatrix<f64>)  {

    let x = sigmas.dot(&Wm);

    let y = sigmas.add_scalar(-&x);

    let P = &y * DMatrix::from_diagonal(&Wc).dot(&y);

    (x,P)
}

