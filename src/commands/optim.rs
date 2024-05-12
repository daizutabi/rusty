use anyhow::Result;
use clap::Args;
use peroxide::fuga::*;
use peroxide::hstack;

/// Optimization
#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        println!("optim");
        fit();
        Ok(())
    }
}

fn fit() {
    println!("fit");

    let normal = Normal(0f64, 0.1f64);
    let normal2 = Normal(0f64, 0.1f64);

    let x = seq(1, 19, 1);
    let x = zip_with(|a, b| (a + b).abs(), &x, &normal.sample(x.len()));

    let y = x.fmap(|t| 5.1734 / t + 3.15 * t);
    let y = zip_with(|a, b| a + b, &y, &normal2.sample(y.len()));

    let n_init = vec![1f64, 1f64];
    let data = hstack!(x.clone(), y.clone());
    println!("{:?}", n_init);
    println!("{:?}", data);

    let mut opt = Optimizer::new(data, quad);
    let p = opt
        .set_init_param(n_init)
        .set_max_iter(50)
        // .set_method(LevenbergMarquardt)
        // .set_lambda_init(1e-3) // Optional: Set initial value of lambda (Only for `LevenbergMarquardt`)
        // .set_lambda_max(1e+3) // Optional: Set maximum bound of lambda (Only for `LevenbergMarquardt`)
        .optimize();
    p.print();
}

fn quad(x: &Vec<f64>, n: Vec<AD>) -> Option<Vec<AD>> {
    Some(
        x.clone()
            .into_iter()
            .map(|t| AD1(t, 0f64))
            .map(|t| n[0] * t + n[1] / t)
            .collect(),
    )
}
