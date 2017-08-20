extern crate spidev;
extern crate rand;

use std::io;
use std::io::prelude::*;
use spidev::{ Spidev, SpidevOptions, SPI_MODE_0 };
use std::{thread, time};

mod cube;
mod apa106led;
mod tables;
mod colour_functions;
mod patterns;

use colour_functions::{ christmas_wheel, fade };
use apa106led::{ Apa106Led, WARM_WHITE, OFF };
use cube::{ Cube4 };
use patterns::{ MAX_BRIGHTNESS };

fn create_spi() -> io::Result<Spidev> {
	let mut spi = try!(Spidev::open("/dev/spidev0.0"));
	let options = SpidevOptions::new()
		 .bits_per_word(8)
		 // .max_speed_hz(4_678_362)
		 .max_speed_hz(6_400_000)
		 .mode(SPI_MODE_0)
		 .build();
	try!(spi.configure(&options));
	Ok(spi)
}

/// perform half duplex operations using Read and Write traits
/*fn half_duplex(spi: &mut Spidev) -> io::Result<()> {
	try!(spi.write(&[
		0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC,
	 	0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC,
	   	0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC, 0xFC,
	]));
	Ok(())
}*/

fn main() {

	//println!("{:?}", half_duplex(&mut spi).unwrap());
	run();
}


fn run() {
	println!("Started\r\n");

	let mut spi = create_spi().unwrap();

	let mut cube = Cube4::new(&mut spi);

	//cube.fill(Apa106Led { red: 0, green: 5, blue: 0 });
	cube.fill(OFF);

	cube.flush();
	thread::sleep(time::Duration::from_millis(1));

	let raindrop_colour = fade(WARM_WHITE, MAX_BRIGHTNESS as f32 / 255.0);

	let wait_time = time::Duration::from_millis(500);

	loop {
		/*cube.fill(Apa106Led { red: 0, green: 5, blue: 0 });
			cube.flush();
		thread::sleep(wait_time);
		cube.fill(OFF);
			cube.flush();
		thread::sleep(wait_time);
		
		cube.fill(Apa106Led { red: 5, green: 0, blue: 0 });
			cube.flush();
		thread::sleep(wait_time);
		cube.fill(OFF);
			cube.flush();
		thread::sleep(wait_time);
		
		cube.fill(Apa106Led { red: 0, green: 0, blue: 5 });
			cube.flush();
		thread::sleep(wait_time);
		cube.fill(OFF);
			cube.flush();
		thread::sleep(wait_time);*/
		
		// Rainbow
		for _ in 0..4 {
			patterns::christmas_rainbow(&mut cube);
		}

		// Fadey slices thing
		for _ in 0..4 {
			patterns::animated_slices(&mut cube);
		}

		// Rain
		for _ in 0..16 {
			patterns::rain(&mut cube, raindrop_colour);
		}

		// Blender
		for _ in 0..16 {
			patterns::blender(&mut cube, raindrop_colour);
		}
	}
}
