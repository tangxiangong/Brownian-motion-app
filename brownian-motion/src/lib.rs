use wasm_bindgen::prelude::*;
use rand_distr::{Normal, Distribution};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
pub struct BrownianMotion {
    length: f64,
    diffusivity: f64,
}

#[wasm_bindgen]
impl BrownianMotion {
    pub fn new(length: f64, diffusivity: f64) -> Self {
        if (length <= 0.) | (diffusivity <= 0.) {
            alert("参数不能为非负数");
            // return Err(ArgError::NegArgErr);
        }
        Self { length, diffusivity, }
    }

    pub fn times(&self, tau: f64) -> Vec<f64> {
        let num = (self.length/ tau).ceil() as usize;
        (0..num+1).map(|idx| tau * idx as f64).collect::<Vec<_>>()
    }
    
    pub fn simulate(&self, tau: f64) -> Vec<f64> {
        let num = (self.length/ tau).ceil() as usize;
        let normal = Normal::new(0.0, (2.0 * self.diffusivity * tau).sqrt()).unwrap();
        let mut rng = rand::thread_rng();
        let mut position = vec![0.0; num+1];
        let mut noise;
        for k in 1..num+1 {
            noise = normal.sample(&mut rng);
            position[k] = position[k-1] + noise;
        }
        position
    }
}