extern crate spidev;
extern crate rand;

use std::io;
use spidev::{ Spidev, SpidevOptions, SPI_MODE_0 };
use std::{thread, time};

mod cube;
mod apa106led;
mod colour_functions;
mod patterns;
mod tables;

use colour_functions::fade;
use apa106led::{ WARM_WHITE, OFF };
use cube::{ Cube4 };
use patterns::{ MAX_BRIGHTNESS };

fn create_spi() -> io::Result<Spidev> {
	let mut spi = try!(Spidev::open("/dev/spidev0.0"));
	let options = SpidevOptions::new()
		 .bits_per_word(8)
		 .max_speed_hz(6_400_000)
		 .mode(SPI_MODE_0)
		 .build();
	try!(spi.configure(&options));
	Ok(spi)
}

fn main() {
	run();
}


fn run() {
	println!("Started\r\n");

	let mut spi = create_spi().unwrap();

	let mut cube = Cube4::new(&mut spi);

	cube.fill(OFF);

	cube.flush();
	thread::sleep(time::Duration::from_millis(1));

	let raindrop_colour = fade(WARM_WHITE, MAX_BRIGHTNESS as f32 / 255.0);

	loop {
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
