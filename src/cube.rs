use spidev::Spidev;
use std::io::Write;
use std::{thread, time};
use std::time::Duration;

use apa106led::Apa106Led;

// const ON_BYTE: u8 = 0b1111_1100;
// const OFF_BYTE: u8 = 0b1100_0000;

const ON_BYTE: u8 = 0b1110_0000;
const OFF_BYTE: u8 = 0b1000_0000;

#[derive(Copy, Clone)]
pub struct Voxel {
	pub x: u8,
	pub y: u8,
	pub z: u8,
}

pub struct Cube4<'a> {
	spi: &'a mut Spidev,

	cube_frame: [Apa106Led; 64],
}

impl<'a> Cube4<'a> {
	pub fn new(spi: &mut Spidev) -> Cube4 {
		let blank_frame: [Apa106Led; 64] = [Apa106Led { red: 1, green: 0, blue: 0 }; 64];

		Cube4 {
			spi: spi,

			cube_frame: blank_frame
		}
	}

	fn coord_to_index(&self, coord: Voxel) -> usize {
		let index = match coord.z {
			0|2 => {
				match coord.y {
					0|2 => (4 * coord.y) + coord.x,
					1|3 => (4 * coord.y) + 3 - coord.x,
					_ => 64
				}
			},
			1|3 => {
				match coord.y {
					0 => 15 - coord.x,
					2 => 7 - coord.x,
					1 => coord.x + 7 + coord.y,
					3 => coord.x + 3 - coord.y,
					_ => 64
				}
			},
			_ => 64
		};

		// Z coord is easy, just offset n * (num voxels in layer)
		(index + (coord.z * 16)) as usize
	}

	pub fn set_at_index(&mut self, index: usize, colour: Apa106Led) {
		self.cube_frame[index] = colour;
	}

	pub fn set_at_coord(&mut self, coord: Voxel, colour: Apa106Led) {
		let idx = self.coord_to_index(coord);

		self.cube_frame[idx] = colour;
	}

	pub fn get_at_coord(&self, coord: Voxel) -> Apa106Led {
		let idx = self.coord_to_index(coord);

		self.cube_frame[idx]
	}

	pub fn fill(&mut self, fill_colour: Apa106Led) {
		self.cube_frame = [fill_colour; 64];
	}

	pub fn fill_layer(&mut self, layer: u8, fill_colour: Apa106Led) {
		for x in 0..4 {
			for y in 0..4 {
				self.set_at_coord(Voxel { x: x, y: y, z: layer }, fill_colour);
			}
		}
	}

	pub fn fill_slice(&mut self, slice: u8, fill_colour: Apa106Led) {
		for y in 0..4 {
			for z in 0..4 {
				self.set_at_coord(Voxel { x: slice, y: y, z: z }, fill_colour);
			}
		}
	}

	pub fn fill_panel(&mut self, panel: u8, fill_colour: Apa106Led) {
		for x in 0..4 {
			for z in 0..4 {
				self.set_at_coord(Voxel { x: x, y: panel, z: z }, fill_colour);
			}
		}
	}

	pub fn fill_column(&mut self, column: Voxel, fill_colour: Apa106Led) {
		for z in 0..4 {
			self.set_at_coord(Voxel { x: column.x, y: column.y, z: z }, fill_colour);
		}
	}

	// #[inline(always)]
	pub fn flush(&mut self) {
		let mut bytes: Vec<u8> = self.cube_frame
			.into_iter()
			.map(|led| colour_to_raw(led).into_iter().map(|byte| *byte).collect::<Vec<u8>>())
			.flat_map(|thing| thing)
			.collect();

		let mut clear: Vec<u8> = (0..5000).map(|b| 0x00).collect();

		// self.spi.write(&clear.as_slice());
		// thread::sleep(Duration::from_millis(100));

		// self.spi.write(&clear.as_slice());

		// thread::sleep(time::Duration::from_millis(200));		
			
		// self.spi.write(&bytes.as_slice());
		self.spi.write(&bytes[0..(24 * 8)]);

		thread::sleep(time::Duration::from_millis(200));
	}
}

fn bit_is_set(byte: u8, bit_index: u8) -> bool {
	(byte & (1 << bit_index)) != 0
}

fn colour_to_raw(input: &Apa106Led) -> [u8; 24] {
	let mut bytes: [u8; 24] = [0; 24];

	// Gamma correct colours
	// let gamma_corrected_input = Apa106Led {
	// 	red: GAMMA_MAP[input.red as usize],
	// 	green: GAMMA_MAP[input.green as usize],
	// 	blue: GAMMA_MAP[input.blue as usize],
	// };

	// SPI transmits MSB first
	// Gamma correction
	// for pos in 0..8 {
	// 	bytes[7 - pos as usize] = if bit_is_set(gamma_corrected_input.red, pos as u8) { ON_BYTE } else { OFF_BYTE };

	// 	bytes[8 + (7 - pos as usize)] = if bit_is_set(gamma_corrected_input.green, pos as u8) { ON_BYTE } else { OFF_BYTE };

	// 	bytes[16 + (7 - pos as usize)] = if bit_is_set(gamma_corrected_input.blue, pos as u8) { ON_BYTE } else { OFF_BYTE };
	// }

	// No gamma correction
	for pos in 0..8 {
		bytes[7 - pos as usize] = if bit_is_set(input.red, pos as u8) { ON_BYTE } else { OFF_BYTE };

		bytes[8 + (7 - pos as usize)] = if bit_is_set(input.green, pos as u8) { ON_BYTE } else { OFF_BYTE };

		bytes[16 + (7 - pos as usize)] = if bit_is_set(input.blue, pos as u8) { ON_BYTE } else { OFF_BYTE };
	}

	bytes
}
