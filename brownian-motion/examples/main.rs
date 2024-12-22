use brownian_motion::BrownianMotion;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let motion = BrownianMotion::new(10., 0.5);
    let t = motion.times(0.1);
    let x = motion.simulate(0.1);
    println!("{:?}\n{:?}", t, x);
    Ok(())
}