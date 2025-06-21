use self::Allergen::*;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

const ALLERGEN_SCORES: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score: score % 256 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let score = *allergen as u32;
        self.score >= score
    }

    pub fn allergies(&mut self) -> Vec<Allergen> {
        let mut retval: Vec<Allergen> = Vec::new();
        let original_score = self.score.clone();

        for allergen in ALLERGEN_SCORES.iter().rev() {
            if self.is_allergic_to(allergen) {
                retval.push(allergen.clone());
                let score = *allergen as u32;
                self.score -= score;
            }
        }
        self.score = original_score;
        retval
    }
}
