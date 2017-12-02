extern crate spidev;
extern crate rand;
extern crate ctrlc;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::io;
use spidev::{ Spidev, SpidevOptions, SPI_MODE_0 };
use std::{thread, time};

mod cube;
mod apa106led;
mod colour_functions;
mod patterns;
mod ws2811;

use colour_functions::fade;
use apa106led::{ WARM_WHITE, OFF };
use cube::{ Cube4 };
use patterns::{ MAX_BRIGHTNESS };

// fn create_spi() -> io::Result<Spidev> {
// 	let mut spi = try!(Spidev::open("/dev/spidev0.0"));
// 	let options = SpidevOptions::new()
// 		 .bits_per_word(8)
// 		 // .lsb_first(false)
// 		 .max_speed_hz(6_400_000)
// 		 // .max_speed_hz(1_920_000)
// 		 // .max_speed_hz(5_120_000)
// 		 .mode(spidev::SPI_MODE_0)
// 		 .build();
// 	try!(spi.configure(&options));
// 	Ok(spi)
// }

fn main() {
	println!("Started\r\n");

	let mut foo: Vec<u32> = (1..64).map(|_| 0x00201000u32).collect();

	let mut empty: Vec<u32> = (1..64).map(|_| 0x00000000u32).collect();

	let raindrop_colour = fade(WARM_WHITE, MAX_BRIGHTNESS as f32 / 255.0);

	unsafe {
		// let mut ledstring = ws2811::get_strings();

		// println!("{:?}", ledstring);

		// ws2811::ws2811_init(&mut ledstring);

		// println!("{:?}", ledstring);

		let running = Arc::new(AtomicBool::new(true));
	    let r = running.clone();
	    ctrlc::set_handler(move || {
	        r.store(false, Ordering::SeqCst);
	    }).expect("Error setting Ctrl-C handler");

	    println!("Waiting for Ctrl-C...");

	    // while running.load(Ordering::SeqCst) {
	    // 	ledstring.channel[0].leds = foo.as_mut_ptr();

	    // 	ws2811::ws2811_render(&mut ledstring);

	    // 	thread::sleep(time::Duration::from_millis(200));
	    // }

	    let mut cube = Cube4::new();

		cube.fill(OFF);

		cube.flush();

	    while running.load(Ordering::SeqCst) {
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

	    println!("Cleaning up...");

	    cube.close();

	    // ledstring.channel[0].leds = empty.as_mut_ptr();

	    // ws2811::ws2811_render(&mut ledstring);

		// ws2811::ws2811_fini(&mut ledstring);
	}

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