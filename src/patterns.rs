use num::clamp;
use std::{thread, time};

use apa106led::{Apa106Led, OFF};
use colour_functions::{christmas_wheel, temp_to_rgb};
use cube::{Cube4, Voxel};
use rand::{self, Rng};

pub fn fire(cube: &mut Cube4) {
    let mut temps = [[[0i32; 4]; 4]; 4];
    let start: u32 = 1600;
    let cooling: i32 = 1600 as i32 / 4;

    for x in 0..4 {
        for y in 0..4 {
            // Start temp in kelvin
            temps[x][y][0] = rand::thread_rng().gen_range(start as i32 - 100, start as i32 + 100);

            cube.set_at_coord(
                Voxel {
                    x: x as u8,
                    y: y as u8,
                    z: 0,
                },
                temp_to_rgb(temps[x][y][0] as u32),
            );
        }
    }

    // cube.flush();

    for _ in 0..32767 {
        for x in 0..4 {
            for y in 0..4 {
                for z in (1..4).rev() {
                    temps[x][y][z] = clamp(temps[x][y][z - 1] - cooling, 0, start as i32);

                    cube.set_at_coord(
                        Voxel {
                            x: x as u8,
                            y: y as u8,
                            z: z as u8,
                        },
                        temp_to_rgb(temps[x][y][z] as u32),
                    );
                }

                // if iter % 20 == 0 {
                temps[x][y][0] =
                    rand::thread_rng().gen_range(start as i32 - 500, start as i32 + 500);
                // }

                cube.set_at_coord(
                    Voxel {
                        x: x as u8,
                        y: y as u8,
                        z: 0,
                    },
                    temp_to_rgb(temps[x][y][0] as u32),
                );
            }
        }

        cube.flush();

        thread::sleep(time::Duration::from_millis(100));
    }
}

pub fn rain(cube: &mut Cube4, raindrop_colour: Apa106Led) {
    let wait = time::Duration::from_millis(120);

    // Spawn some new raindrops
    for index in 0..16 {
        cube.set_at_index(
            (index + 16 * 3) as usize,
            if rand::thread_rng().gen_range(0, 100) < 16 {
                raindrop_colour
            } else {
                OFF
            },
        );
    }

    cube.flush();

    thread::sleep(wait);

    for _ in 0..4 {
        // Move existing raindrops down
        for z in 1..4 {
            for x in 0..4 {
                for y in 0..4 {
                    let current_position = Voxel { x: x, y: y, z: z };
                    let next_position = Voxel {
                        x: x,
                        y: y,
                        z: z - 1,
                    };
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
            let pos: u16 = (index * 4) + counter;

            let wheel_col = christmas_wheel(pos as u8);

            cube.set_at_index(index as usize, wheel_col);
        }

        cube.flush();

        thread::sleep(time::Duration::from_millis(16));
    }
}

pub fn animated_slices(cube: &mut Cube4) {
    let frame_time = time::Duration::from_millis(2);

    // Fade red panels up
    for panel in 0..4 {
        for i in 0..255 {
            cube.fill_panel(
                panel,
                Apa106Led {
                    red: i,
                    green: 0,
                    blue: 0,
                },
            );

            cube.flush();

            thread::sleep(frame_time);
        }
    }

    // Fade all that shit out
    for i in (0..255).rev() {
        for panel in 0..4 {
            cube.fill_panel(
                panel,
                Apa106Led {
                    red: i,
                    green: 0,
                    blue: 0,
                },
            );
        }

        cube.flush();

        thread::sleep(frame_time);
    }

    // Fade green slices up
    for slice in 0..4 {
        for i in 0..255 {
            cube.fill_slice(
                slice,
                Apa106Led {
                    red: 0,
                    green: i,
                    blue: 0,
                },
            );

            cube.flush();

            thread::sleep(frame_time);
        }
    }

    // Fade all that shit out
    for i in (0..255).rev() {
        for slice in 0..4 {
            cube.fill_slice(
                slice,
                Apa106Led {
                    red: 0,
                    green: i,
                    blue: 0,
                },
            );
        }

        cube.flush();

        thread::sleep(frame_time);
    }

    // Fade white layers  up
    for layer in (0..4).rev() {
        for i in 0..255 {
            cube.fill_layer(
                layer,
                Apa106Led {
                    red: i,
                    green: i,
                    blue: i,
                },
            );

            cube.flush();

            thread::sleep(frame_time);
        }
    }

    // Fade all that shit out
    for i in (0..255).rev() {
        for layer in 0..4 {
            cube.fill_layer(
                layer,
                Apa106Led {
                    red: i,
                    green: i,
                    blue: i,
                },
            );
        }

        cube.flush();

        thread::sleep(frame_time);
    }
}

pub fn blender(cube: &mut Cube4, fill_colour: Apa106Led) {
    for offs in 0..6 {
        for i in 0..64 {
            cube.set_at_index(
                i,
                Apa106Led {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
            );

            // 1 microsecond
            thread::sleep(time::Duration::new(0, 1000));
        }

        // Inside ring
        match offs {
            0 | 1 | 5 => {
                cube.fill_column(Voxel { x: 1, y: 2, z: 0 }, fill_colour);
                cube.fill_column(Voxel { x: 2, y: 1, z: 0 }, fill_colour);
            }
            _ => {
                cube.fill_column(Voxel { x: 1, y: 1, z: 0 }, fill_colour);
                cube.fill_column(Voxel { x: 2, y: 2, z: 0 }, fill_colour);
            }
        }

        // Outside ring
        if offs < 4 {
            cube.fill_column(
                Voxel {
                    x: 3 - offs,
                    y: 0,
                    z: 0,
                },
                fill_colour,
            );
            cube.fill_column(
                Voxel {
                    x: offs,
                    y: 3,
                    z: 0,
                },
                fill_colour,
            );
        } else {
            cube.fill_column(
                Voxel {
                    x: 0,
                    y: offs - 3,
                    z: 0,
                },
                fill_colour,
            );
            cube.fill_column(
                Voxel {
                    x: 3,
                    y: 3 - (offs - 3),
                    z: 0,
                },
                fill_colour,
            );
        }

        cube.flush();

        thread::sleep(time::Duration::from_millis(100));
    }
}
