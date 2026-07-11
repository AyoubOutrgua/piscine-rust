use std::fmt;

#[derive(Debug)]
pub struct Player<'a> {
	pub name: &'a str,
	pub strength: f64,
	pub score: u32,
	pub money: u32,
	pub weapons: Vec<&'a str>,
}

pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player<'_> {
	pub fn eat(&mut self, food: impl Food) {
		self.strength += food.gives();
	}
}

impl fmt::Display for Player<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f,"{}",self.name)?;
        let line_2 = format!("Strength: {}, Score: {}, Money: {}", self.strength,self.score,self.money);
        writeln!(f,"{}",line_2)?;
        write!(f,"Weapons: {:?}",self.weapons)?;
        Ok(())
    }
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
	fn gives(&self) -> f64 {
        self.weight_in_kg * 4.0
	}
}

impl Food for Meat {
	fn gives(&self) -> f64 {
		let total_fat = self.fat_content * self.weight_in_kg;
        let fat_strength = total_fat * 9.0;
        let total_proten = self.weight_in_kg - total_fat;
        let proten_strength = total_proten * 4.0;
        fat_strength + proten_strength
	}
}