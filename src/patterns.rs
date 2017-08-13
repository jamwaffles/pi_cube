// use zinc::hal::timer::Timer;
// use zinc::hal::tiva_c::timer::{ Timer as TivaCTimer };

use std::{thread, time};

use colour_functions::{ christmas_wheel, fade };
use apa106led::{ Apa106Led, WARM_WHITE, OFF };
use cube::{ Cube4, Voxel };
use rand::{ rand_range };

pub const MAX_BRIGHTNESS: u8 = 25;

pub fn rain(cube: &mut Cube4, raindrop_colour: Apa106Led) {
	let wait = time::Duration::from_millis(120);

	// Spawn some new raindrops
	for index in 0..16 {
		cube.set_at_index((index + 16 * 3) as usize, if rand_range(0, 64) < 16 { raindrop_colour } else { OFF });
	}

	cube.flush();

	thread::sleep(wait);

	for _ in 0..4 {
		// Move existing raindrops down
		for z in (1..4) {
			for x in 0..4 {
				for y in 0..4 {
					let current_position = Voxel { x: x, y: y, z: z };
					let next_position = Voxel { x: x, y: y, z: z - 1 };
					let current_col = cube.get_at_coord(current_position);

					// cube.set_at_coord(current_position, fade(current_col, 0.35));
					cube.set_at_coord(current_position, OFF);
					cube.set_at_coord(next_position, current_col);
				}
			}
		}


		cube.flush();

		thread::sleep(wait);
	}
}

pub fn christmas_rainbow(cube: &mut Cube4) {
	for counter in 0..255 {
		for index in 0..64 {
			let wheel_col = christmas_wheel(((index * 4) + counter as u8) & 255);

			cube.set_at_index(index as usize, fade(wheel_col, (MAX_BRIGHTNESS as f32 / 255.0)));
		}

		cube.flush();

		thread::sleep(time::Duration::from_millis(16));
	}
}

pub fn animated_slices(cube: &mut Cube4) {
	// const FRAME_TIME: u32 = 40;
	let FRAME_TIME = time::Duration::from_millis(40);

	// Fade red panels up
	for panel in 0..4 {
		for i in 0..MAX_BRIGHTNESS {
			cube.fill_panel(panel, Apa106Led { red: i, green: 0, blue: 0 });

			cube.flush();

			thread::sleep(FRAME_TIME);
		}
	}

	// Fade all that shit out
	for i in (0..MAX_BRIGHTNESS).rev() {
		for panel in 0..4 {
			cube.fill_panel(panel, Apa106Led { red: i, green: 0, blue: 0 });
		}

		cube.flush();

		thread::sleep(FRAME_TIME);
	}

	// Fade green slices up
	for slice in 0..4 {
		for i in 0..MAX_BRIGHTNESS {
			cube.fill_slice(slice, Apa106Led { red: 0, green: i, blue: 0 });

			cube.flush();

			thread::sleep(FRAME_TIME);
		}
	}

	// Fade all that shit out
	for i in (0..MAX_BRIGHTNESS).rev() {
		for slice in 0..4 {
			cube.fill_slice(slice, Apa106Led { red: 0, green: i, blue: 0 });
		}

		cube.flush();

		thread::sleep(FRAME_TIME);
	}

	// Fade white layers  up
	for layer in (0..4).rev() {
		for i in 0..MAX_BRIGHTNESS {
			cube.fill_layer(layer, Apa106Led { red: i, green: i, blue: i });

			cube.flush();

			thread::sleep(FRAME_TIME);
		}
	}

	// Fade all that shit out
	for i in (0..MAX_BRIGHTNESS).rev() {
		for layer in 0..4 {
			cube.fill_layer(layer, Apa106Led { red: i, green: i, blue: i });
		}

		cube.flush();

		thread::sleep(FRAME_TIME);
	}
}

pub fn blender(cube: &mut Cube4, fill_colour: Apa106Led) {
	for offs in 0..6 {
		for i in 0..64 {
			cube.set_at_index(i, Apa106Led { red: 0, green: 0, blue: 0 });

			// timer.wait_us(1);
			// 1 microsecond
			thread::sleep(time::Duration::new(0, 1000));
		}

		// Inside ring
		match offs {
			0|1|5 => {
				cube.fill_column(Voxel { x: 1, y: 2, z: 0 }, fill_colour);
				cube.fill_column(Voxel { x: 2, y: 1, z: 0 }, fill_colour);
			},
			_ => {
				cube.fill_column(Voxel { x: 1, y: 1, z: 0 }, fill_colour);
				cube.fill_column(Voxel { x: 2, y: 2, z: 0 }, fill_colour);
			}
		}

		// Outside ring
		if offs < 4 {
			cube.fill_column(Voxel { x: 3 - offs, y: 0, z: 0 }, fill_colour);
			cube.fill_column(Voxel { x: offs, y: 3, z: 0 }, fill_colour);
		} else {
			cube.fill_column(Voxel { x: 0, y: offs - 3, z: 0 }, fill_colour);
			cube.fill_column(Voxel { x: 3, y: 3 - (offs - 3), z: 0 }, fill_colour);
		}

		cube.flush();

		thread::sleep(time::Duration::from_millis(100));
	}
}