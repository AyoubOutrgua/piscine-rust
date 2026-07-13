use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

impl FromStr for BloodType {
	type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
            "A+" => Ok(BloodType{antigen: Antigen::A, rh_factor: RhFactor::Positive}),
            "A-" => Ok(BloodType{antigen: Antigen::A, rh_factor: RhFactor::Negative}),
            "B+" => Ok(BloodType{antigen: Antigen::B, rh_factor: RhFactor::Positive}),
            "B-" => Ok(BloodType{antigen: Antigen::B, rh_factor: RhFactor::Negative}),
            "O+" => Ok(BloodType{antigen: Antigen::O, rh_factor: RhFactor::Positive}),
            "O-" => Ok(BloodType{antigen: Antigen::O, rh_factor: RhFactor::Negative}),
            "AB+" => Ok(BloodType{antigen: Antigen::AB, rh_factor: RhFactor::Positive}),
            "AB-" => Ok(BloodType{antigen: Antigen::AB, rh_factor: RhFactor::Negative}),
            _ => Err(())
        }
    }
}

impl fmt::Debug for BloodType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh_factor = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}",antigen, rh_factor)?;
        Ok(())
    }
}

impl BloodType {
	pub fn can_receive_from(self, other: Self) -> bool {
        let rh = match (self.rh_factor, other.rh_factor) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            _ => false,
        };
        let antigen = match (self.antigen, other.antigen) {
            (Antigen::AB, _) => true,
            (Antigen::A, Antigen::A | Antigen::O) => true,
            (Antigen::B, Antigen::B | Antigen::O) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        };
        rh && antigen
    }

    pub fn donors(self) -> Vec<Self> {
        get_all_bloods().into_iter().filter(|boond| self.can_receive_from(*boond)).collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        get_all_bloods().into_iter().filter(|boond| boond.can_receive_from(self)).collect()
    }
}

fn get_all_bloods() -> Vec<BloodType>{
    vec![
        BloodType{
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        },
        BloodType{
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        },
        BloodType{
            antigen: Antigen::B,
            rh_factor: RhFactor::Positive,
        },
        BloodType{
            antigen: Antigen::B,
            rh_factor: RhFactor::Negative,
        },
        BloodType{
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        },
        BloodType{
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        },
        BloodType{
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        },
        BloodType{
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        },
    ]
}