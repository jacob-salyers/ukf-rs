#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate nalgebra;

use nalgebra::base::{DMatrix,DVector,MatrixMN,Dim};
use nalgebra::linalg::{Cholesky};

struct UKF {

        x: DVector<f64>,
        P: DMatrix<f64>,
        Q: DMatrix<f64>,
        R: DMatrix<f64>,
        dt: f64,

        Wm: DVector<f64>,
        WC: DVector<f64>,

        sigmas_f: DVector<f64>,
        sigmas_h: DVector<f64>,

        x_prior: DVector<f64>,
        P_prior: DVector<f64>,

        x_post: DVector<f64>,
        P_post: DVector<f64>,

}

struct MerweScaledPoints{

    n: usize,
    alpha: f64,
    beta: f64,
    kappa: f64,
}    

impl MerweScaledPoints {

    fn num_sigmas(&self) -> usize {
        2*self.n+1
    }

    fn sigma_points(&self, x: DVector<f64>, P: DMatrix<f64>) {
        
        let lambda = self.alpha.powi(2) * (f64::from(self.n as u32) + self.kappa) - f64::from(self.n as u32);
        let u = (lambda + f64::from(self.n as u32)).sqrt() * P;

        let mut sigmas = DMatrix::from_element(2*self.n+1, self.n ,0.0);

        for j in 0..self.n {
            sigmas[(0,j)] = x[j];    

            for i in 0..self.n {
                sigmas[((i+1),j)] = x[j] - u[(i, j)];
                sigmas[((self.n + i+1),j)] = x[j] + u[(i, j)];
            }
        }
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

