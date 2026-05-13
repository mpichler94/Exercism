pub struct Allergies {
    allergies: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies = Vec::new();
        if score & 1 != 0 {
            allergies.push(Allergen::Eggs);
        }
        if score & 2 != 0 {
            allergies.push(Allergen::Peanuts);
        }
        if score & 4 != 0 {
            allergies.push(Allergen::Shellfish);
        }
        if score & 8 != 0 {
            allergies.push(Allergen::Strawberries);
        }
        if score & 16 != 0 {
            allergies.push(Allergen::Tomatoes);
        }
        if score & 32 != 0 {
            allergies.push(Allergen::Chocolate);
        }
        if score & 64 != 0 {
            allergies.push(Allergen::Pollen);
        }
        if score & 128 != 0 {
            allergies.push(Allergen::Cats);
        }
        Allergies { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
