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
mod ws2811;

use colour_functions::fade;
use apa106led::{ WARM_WHITE, OFF };
use cube::{ Cube4 };
use patterns::{ MAX_BRIGHTNESS };

fn create_spi() -> io::Result<Spidev> {
	let mut spi = try!(Spidev::open("/dev/spidev0.0"));
	let options = SpidevOptions::new()
		 .bits_per_word(8)
		 // .lsb_first(false)
		 .max_speed_hz(6_400_000)
		 // .max_speed_hz(1_920_000)
		 // .max_speed_hz(5_120_000)
		 .mode(spidev::SPI_MODE_0)
		 .build();
	try!(spi.configure(&options));
	Ok(spi)
}

fn main() {
	run();
}

fn run() {
	println!("Started\r\n");

	// let hw = ws2811::rpi_hw_t {
	// 	pub type_: u32,
	// 	pub hwver: u32,
	// 	pub periph_base: u32,
	// 	pub videocore_base: u32,
	// 	pub desc: *mut ::std::os::raw::c_char,
	// }

	let mut foo = vec![
		0x00200000u32,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
		0x00200000,
	];

	unsafe {
		let mut ledstring = ws2811::get_strings();

		println!("{:?}", ledstring);

		ws2811::ws2811_init(&mut ledstring);

		println!("{:?}", ledstring);

		loop {
			ledstring.channel[0].leds = foo.as_mut_ptr();

			ws2811::ws2811_render(&mut ledstring);
		}
	}

	// unsafe {
	// 	let hw = ws2811::rpi_hw_detect();

	// 	let mut rust_ledstring = ws2811::ws2811_t {
	// 		// pub render_wait_time: u64,
	// 		// pub device: *mut ws2811_device,
	// 		// pub rpi_hw: *const rpi_hw_t,
	// 		// pub freq: u32,
	// 		// pub dmanum: ::std::os::raw::c_int,
	// 		// pub channel: [ws2811_channel_t; 2usize],

	// 		freq: 800_000,
	// 		dmanum: 5,
	// 		device: &mut ws2811::ws2811_device { _unused: [] },
	// 		rpi_hw: hw,
	// 		render_wait_time: 0,

	// 		channel: [
	// 			ws2811::ws2811_channel_t {
	// 				gpionum: 18,
	// 				count: 256,
	// 				invert: 0,
	// 				brightness: 32,
	// 				strip_type: (ws2811::WS2811_STRIP_RGB as i32),
	// 				leds: &mut 0,
	// 				wshift: 0,
	// 				rshift: 0,
	// 				gshift: 0,
	// 				bshift: 0,
	// 				gamma: &mut 0,
	// 			},
	// 			ws2811::ws2811_channel_t {
	// 				gpionum: 0,
	// 				count: 0,
	// 				invert: 0,
	// 				brightness: 0,
	// 				strip_type: (ws2811::WS2811_STRIP_RGB as i32),
	// 				leds: &mut 0,
	// 				wshift: 0,
	// 				rshift: 0,
	// 				gshift: 0,
	// 				bshift: 0,
	// 				gamma: &mut 0,
	// 			}
	// 		]
	// 	};

	// 	ws2811::ws2811_init(&mut rust_ledstring);
	// }

	// let mut spi = create_spi().unwrap();

	// let mut cube = Cube4::new(&mut spi);

	// cube.fill(OFF);

	// cube.flush();
	// thread::sleep(time::Duration::from_millis(1));

	// let raindrop_colour = fade(WARM_WHITE, MAX_BRIGHTNESS as f32 / 255.0);

	// loop {
	// 	// Rainbow
	// 	for _ in 0..4 {
	// 		patterns::christmas_rainbow(&mut cube);
	// 	}

	// 	// Fadey slices thing
	// 	for _ in 0..4 {
	// 		patterns::animated_slices(&mut cube);
	// 	}

	// 	// Rain
	// 	for _ in 0..16 {
	// 		patterns::rain(&mut cube, raindrop_colour);
	// 	}

	// 	// Blender
	// 	for _ in 0..16 {
	// 		patterns::blender(&mut cube, raindrop_colour);
	// 	}
	// }
}
