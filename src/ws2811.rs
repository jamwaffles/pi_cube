#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn get_strings() -> ws2811_t {
	unsafe {
		let hw = rpi_hw_detect();

		let mut rust_ledstring = ws2811_t {
			// pub render_wait_time: u64,
			// pub device: *mut ws2811_device,
			// pub rpi_hw: *const rpi_hw_t,
			// pub freq: u32,
			// pub dmanum: ::std::os::raw::c_int,
			// pub channel: [ws2811_channel_t; 2usize],

			freq: 800_000,
			dmanum: 5,
			device: &mut ws2811_device { _unused: [] },
			rpi_hw: hw,
			render_wait_time: 100,

			channel: [
				ws2811_channel_t {
					gpionum: 21,
					count: 4,
					invert: 0,
					brightness: 32,
					strip_type: (WS2811_STRIP_BGR as i32),
					leds: &mut 0,
					wshift: 0,
					rshift: 0,
					gshift: 0,
					bshift: 0,
					gamma: &mut 0,
				},
				ws2811_channel_t {
					gpionum: 0,
					count: 0,
					invert: 0,
					brightness: 0,
					strip_type: (WS2811_STRIP_RGB as i32),
					leds: &mut 0,
					wshift: 0,
					rshift: 0,
					gshift: 0,
					bshift: 0,
					gamma: &mut 0,
				}
			]
		};

		rust_ledstring
	}
}
