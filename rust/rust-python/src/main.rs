use std::fs::File;

use ndarray::prelude::*;
use ndarray_npy::{ReadNpyExt, WriteNpyExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("./before.npy")?;
    let mut xs = Array1::<f64>::read_npy(input)?;

    // For numpy-style dynamic dimension:
    // let mut xs = ArrayD::<64>::read_npy(input)?;

    xs += 1.0;

    let output = File::create("./after.npy")?;
    xs.write_npy(output)?;
    Ok(())
}
