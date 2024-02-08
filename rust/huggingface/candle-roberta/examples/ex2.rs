use anyhow::Result;
use candle_core::{Device, Error, Tensor};

fn main() -> Result<(), Error> {
    let data: [u32; 3] = [1u32, 2, 3];
    let tensor = Tensor::new(&data, &Device::Cpu)?;

    let zero_tensor = tensor.zeros_like()?;
    println!("zero_tensor: {:?}", zero_tensor.to_vec1::<u32>()?);

    let ones_tensor = tensor.ones_like()?;
    println!("ones_tensor: {:?}", ones_tensor.to_vec1::<u32>()?);

    let random_tensor = tensor.rand_like(0.0, 1.0)?;
    println!("random_tensor: {:?}", random_tensor.to_vec1::<f64>()?);

    Ok(())
}
