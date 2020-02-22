use std::time::Instant;

use plonky::{Bls12Scalar, G1_GENERATOR, msm_execute, msm_precompute, msm_execute_parallel};

const DEGREE: usize = 1 << 17;

fn main() {
    // Configure the main thread pool size.
    rayon::ThreadPoolBuilder::new().num_threads(24).build_global().unwrap();

    // Here's a quick Python snippet to calculate optimal window sizes:
    //     degree = 2**17
    //     parallelism = 8
    //     field_bits = 253
    //     group_ops = lambda w: 2**w + degree * ceil(field_bits / w) / parallelism
    //     min(range(1, 50), key=group_ops)
    let w = 15;

    let mut generators = Vec::with_capacity(DEGREE);
    let mut scalars = Vec::with_capacity(DEGREE);
    for _i in 0..DEGREE {
        generators.push(G1_GENERATOR);
        scalars.push(Bls12Scalar::rand());
    }

    let start = Instant::now();
    println!("Precomputing...");
    let precomputation = msm_precompute(&generators, w);
    println!("Finished in {}s", start.elapsed().as_secs_f64());
    println!();

    let start = Instant::now();
    println!("Computing MSM with one thread...");
    let result = msm_execute(&precomputation, &scalars, w);
    println!("Finished in {}s", start.elapsed().as_secs_f64());
    println!("Result: {:?}", result.to_affine());
    println!();

    let start = Instant::now();
    println!("Computing MSM in parallel...");
    let result = msm_execute_parallel(&precomputation, &scalars, w);
    println!("Finished in {}s", start.elapsed().as_secs_f64());
    println!("Result: {:?}", result.to_affine());
}