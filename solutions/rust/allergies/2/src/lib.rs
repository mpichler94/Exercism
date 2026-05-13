use Allergen::*;
pub struct Allergies {
    allergies: u32
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

const ALLERGIES: [Allergen; 8] = [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen,  Cats];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { allergies: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        self.allergies & allergen != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGIES.iter().filter(|a| self.is_allergic_to(a)).cloned().collect()
    }
}
