extern crate rand;

use rand::Rng;
use std::io;
use std::mem::transmute;

trait Item {
	fn to_str(&self) -> &str;
	fn count() -> i32;
	fn rand() -> Self;
	fn rand_str() -> String {
		String::from(Self::rand().to_str())
	}
}

macro_rules! item_ext {
	( $x:ident ) => (
		fn count() -> i32 { $x::End as i32 }
		fn rand() -> $x { unsafe { transmute( rand::thread_rng().gen_range(0, $x::count()) ) } }
	);
}


#[repr(i32)]
pub enum Note {
	A, B, C, D, E, F, G, End
}

impl Item for Note {
	fn to_str(&self) -> &str {
		match *self {
			Note::A => "A",
			Note::B => "B",
			Note::C => "C",
			Note::D => "D",
			Note::E => "E",
			Note::F => "F",
			Note::G => "G",
			_ => "",
		}
	}
	
	item_ext!(Note);
}

#[repr(i32)]
pub enum Sign {
	No,
	Sharp,
	Flat,

	End
}

impl Item for Sign {
	fn to_str(&self) -> &str {
		match *self {
			Sign::Sharp => "#",
			Sign::Flat => "b",
			_ => "",
		}
	}

	item_ext!(Sign);
}

#[repr(i32)]
pub enum Tone {
	Major,
	Minor,

	End
}

impl Item for Tone {
	fn to_str(&self) -> &str {
		match *self {
			Tone::Minor => "m",
			_ => "",
		}
	}
	
	item_ext!(Tone);
}

#[repr(i32)]
pub enum ExtraTone {
	Major,
	Minor,
	
	Dom7,
	Maj7,
	Min7,
	
	Dom9,
	DomMin9,
	Maj9,
	Min9,
	Add9,
	
	Dom13,
	Add13,
	
	Aug,
	Dim,
	Sus2,
	Sus4,
	Power,

	End,
}

impl Item for ExtraTone {
	fn to_str(&self) -> &str {
		match *self {
			ExtraTone::Minor => "m",
	
			ExtraTone::Dom7 => "7",
			ExtraTone::Maj7 => "maj7",
			ExtraTone::Min7 => "m7",
			
			ExtraTone::Dom9 => "9",
			ExtraTone::DomMin9 => "7b9",
			ExtraTone::Maj9 => "maj9",
			ExtraTone::Min9 => "m9",
			ExtraTone::Add9 => "add9",
			
			ExtraTone::Dom13 => "13",
			ExtraTone::Add13 => "add13",
			
			ExtraTone::Aug => "aug",
			ExtraTone::Dim => "dim",
			ExtraTone::Sus2 => "sus2",
			ExtraTone::Sus4 => "sus4",
			ExtraTone::Power => "5",
			_ => "",
		}
	}

	item_ext!(ExtraTone);
}

fn main() {
	let mut extra: bool = false;

	loop {
		match extra {
			false => println!("{}{}{}", Note::rand_str(), Sign::rand_str(), Tone::rand_str()),
			true => println!("\r\n{}{}{}", Note::rand_str(), Sign::rand_str(), ExtraTone::rand_str()),
		}

		extra = false;

		let mut key: String = String::new();
		io::stdin().read_line(&mut key).expect("failed to read line");

		key = String::from(key.trim());

		if key == "x" {
			extra = true;
			continue;
		}

		if key != "" {
			break;
		}
	}
}
