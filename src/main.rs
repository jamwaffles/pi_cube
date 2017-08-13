extern crate spidev;
use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SPI_MODE_0};

fn create_spi() -> io::Result<Spidev> {
	let mut spi = try!(Spidev::open("/dev/spidev0.0"));
	let options = SpidevOptions::new()
		 .bits_per_word(8)
		 .max_speed_hz(4_678_362)
		 .mode(SPI_MODE_0)
		 .build();
	try!(spi.configure(&options));
	Ok(spi)
}

/// perform half duplex operations using Read and Write traits
fn half_duplex(spi: &mut Spidev) -> io::Result<()> {
	try!(spi.write(&[
		0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 
	 	0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 
	   	0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 
	]));
	Ok(())
}

fn main() {
	let mut spi = create_spi().unwrap();
	println!("{:?}", half_duplex(&mut spi).unwrap());
}

