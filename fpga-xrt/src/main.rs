use fpga_xrt::Xrt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let device = Xrt::new()?;
    for i in 0..3 {
        let device = Xrt::from_device_index(i)?;
        println!("{}", device.name());
        println!("{}", device.uuid());
    }
    Ok(())
}
