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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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

static SEPT7_SET: [ExtraTone; 3] = [
	ExtraTone::Dom7,
    ExtraTone::Maj7,
    ExtraTone::Min7,
];

static SEPT9_SET: [ExtraTone; 5] = [
	ExtraTone::Dom9,
	ExtraTone::DomMin9,
    ExtraTone::Maj9,
    ExtraTone::Min9,
	ExtraTone::Add9,
];

static SEPT13_SET: [ExtraTone; 2] = [
	ExtraTone::Dom13,
	ExtraTone::Add13,
];

static OTHER_SET: [ExtraTone; 5] = [
	ExtraTone::Aug,
	ExtraTone::Dim,
	ExtraTone::Sus2,
	ExtraTone::Sus4,
	ExtraTone::Power,
];

fn rand_from_set(set: &[ExtraTone]) -> ExtraTone {
	let i = rand::thread_rng().gen_range(0, set.len());
	set[i]
}

pub enum ChordType {
	No,
	Simple,
	Any,
	
	Sept7,
	Sept9,
	Sept13,
	
	Other,
}

impl ChordType {
	fn from_str(string: &str) -> ChordType {
		match string {
			"" => ChordType::Simple,
			"x" => ChordType::Any,
			"7" => ChordType::Sept7,
			"9" => ChordType::Sept9,
			"13" => ChordType::Sept13,
			"s" => ChordType::Other,
			_ => ChordType::No,
		}
	}
}

fn main() {
	loop {
		let mut key: String = String::new();
		io::stdin().read_line(&mut key).expect("failed to read line");
		key = String::from(key.trim());
		
		let chord = ChordType::from_str(&key);
		
		let last_str = match chord {
			ChordType::Simple => Tone::rand_str(),
			ChordType::Any    => ExtraTone::rand_str(),
			ChordType::Sept7  => String::from(rand_from_set(&SEPT7_SET).to_str()),
			ChordType::Sept9  => String::from(rand_from_set(&SEPT9_SET).to_str()),
			ChordType::Sept13 => String::from(rand_from_set(&SEPT13_SET).to_str()),
			ChordType::Other  => String::from(rand_from_set(&OTHER_SET).to_str()),
			_ => break,
		};
		
		println!("\n{}{}{}", Note::rand_str(), Sign::rand_str(), last_str);
	}
}
