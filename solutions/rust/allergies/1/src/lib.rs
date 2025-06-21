pub struct Allergies {
    score: u32,
    allergen_scores: [(Allergen, u32); 8],
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
        use crate::Allergen::*;
        Allergies {
            score: score % 256,
            allergen_scores: [
                (Eggs, 1),
                (Peanuts, 2),
                (Shellfish, 4),
                (Strawberries, 8),
                (Tomatoes, 16),
                (Chocolate, 32),
                (Pollen, 64),
                (Cats, 128),
            ],
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let score = self.allergen_scores.iter().find(|&alsc| alsc.0 == *allergen).unwrap().1;
        self.score >= score
    }

    pub fn allergies(&mut self) -> Vec<Allergen> {
        let mut retval: Vec<Allergen> = Vec::new();
        let original_score = self.score.clone();

        for (allergen, score) in self.allergen_scores.iter().rev() {
            if self.is_allergic_to(allergen) {
                retval.push(allergen.clone());
                self.score -= score;
            }
        }
        self.score = original_score;
        retval
    }
}
