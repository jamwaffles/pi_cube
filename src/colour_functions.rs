use apa106led::Apa106Led;

#[allow(dead_code)]
pub fn rgb_wheel(wheelpos: u8) -> Apa106Led {
	let mut thingy = wheelpos;

	if thingy < 85 {
		Apa106Led { red: thingy * 3, green: 255 - thingy * 3, blue: 0 }
	} else if thingy < 170 {
		thingy -= 85;

		Apa106Led { red: 255 - thingy * 3, green: 0, blue: thingy * 3 }
	} else {
		thingy -= 170;

		Apa106Led { red: 0, green: thingy * 3, blue: 255 - thingy * 3 }
	}
}

// Red - green - white colour wheel
pub fn christmas_wheel(wheelpos: u8) -> Apa106Led {
	let mut thingy = wheelpos;

	// Ramp red down to 0, green up to 255
	if thingy < 85 {
		Apa106Led { red: 255 - thingy * 3, green: thingy * 3, blue: 0 }
	} else if thingy < 170 {	// Ramp red and blue up, leave green at 255
		thingy -= 85;

		Apa106Led { red: thingy * 3, green: 255, blue: thingy * 3 }
	} else {		// Ramp green and blue down, leave red at 255
		thingy -= 170;

		Apa106Led { red: 255, green: 255 - thingy * 3, blue: 255 - thingy * 3 }
	}
}

#[allow(dead_code)]
pub fn temp_to_rgb(kelvin: u32) -> Apa106Led {
	let temp = kelvin as f32 / 100.0;

	let red = if temp <= 66.0 {
		255
	} else {
		let mut r = temp - 60.0;

		r = 329.698727446 * (r.powf(-0.1332047592));

		r as u8
	};

	let green = if temp <= 66.0 {
		(99.4708025861 * temp.ln() - 161.1195681661) as u8
	} else {
		let mut g = temp - 60.0;

		g = 329.698727446 * (g.powf(-0.1332047592));

		g as u8
	};

	let blue = if temp >= 66.0 {
		255
	} else if temp <= 19.0 {
		0
	} else {
		let mut b = temp - 10.0;

		b = 138.5177312231 * b.ln() - 305.0447927307;

		b as u8
	};


	Apa106Led { red, green, blue }
}

#[allow(dead_code)]
pub fn fade(colour: Apa106Led, multiplier: f32) -> Apa106Led {
	let divisor: u8 = (1.0 / multiplier) as u8;

	Apa106Led {
		red: (colour.red / divisor) as u8,
		green: (colour.green / divisor) as u8,
		blue: (colour.blue / divisor) as u8,
	}
}