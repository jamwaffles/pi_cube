extern crate rand;
extern crate spidev;

use spidev::{SPI_MODE_0, Spidev, SpidevOptions};
use std::io;

mod apa106led;
mod colour_functions;
mod cube;
mod patterns;

use apa106led::{OFF, WARM_WHITE};
use cube::Cube4;

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

    let mut cube = Cube4::new(&mut spi, 25);

    cube.fill(OFF);

    cube.flush();

    let raindrop_colour = WARM_WHITE;

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
