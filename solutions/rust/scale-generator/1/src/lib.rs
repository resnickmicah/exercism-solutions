use self::Chromatic::*;
// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug, PartialEq)]
pub enum Chromatic {
    A,
    AB, // A# or Bb
    B,
    C,
    CD, // C# or Db
    D,
    DE, // D# or Eb
    E,
    F,
    FG, // F# or Gb
    G,
    GA, // G# or Ab
}

#[derive(Debug)]
pub enum Error {
    TonicError,
    IntervalError,
}

#[derive(Debug)]
pub enum SharpsOrFlats {
    Sharps,
    Flats,
}

const CHROMATICS: [Chromatic; 12] = [A, AB, B, C, CD, D, DE, E, F, FG, G, GA];
const OCTAVE_INTERVALS: usize = 12;

pub struct Scale {
    tonic: Chromatic,
    intervals: Vec<usize>,
    sharp_flat: SharpsOrFlats,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        use self::SharpsOrFlats::*;
        let sof: Result<SharpsOrFlats, Error> = match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#" => {
                Ok(Sharps)
            }
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => {
                Ok(Flats)
            }
            _ => Err(Error::TonicError),
        };
        if let Err(err) = sof {
            return Err(err);
        }

        let starting_note: Chromatic = match tonic {
            "A" | "a" => A,
            "Bb" | "bb" => AB,
            "B" | "b" => B,
            "C" | "c" => C,
            "c#" | "Db" => CD, 
            "D" | "d" => D,
            "d#" | "Eb" | "eb" => DE,
            "E" | "e" => E,
            "F" | "f" => F,
            "F#" | "f#" | "Gb" => FG,
            "G" | "g" => G,
            "g#" | "Ab" => GA,
            _ => panic!("Invalid tonic in struct")
        };

        let mut numeric_intervals: Vec<usize> = vec![];

        for iv in intervals.chars() {
            let iv: Result<usize, Error> = match iv {
                'm' => Ok(1), // half step
                'M' => Ok(2), // whole step
                'A' => Ok(3), // augmented whole = whole + half
                _ => Err(Error::IntervalError),
            };
            if let Err(err) = iv {
                return Err(err);
            } else if let Ok(niv) = iv {
                numeric_intervals.push(niv);
            }
        }
        if numeric_intervals.iter().sum::<usize>() != OCTAVE_INTERVALS {
            return Err(Error::IntervalError);
        } else {
            // handle off by one when enumerating
            numeric_intervals.push(0);
        }
        Ok(Scale {
            tonic: starting_note,
            intervals: numeric_intervals,
            sharp_flat: sof.unwrap(),
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        let starting_index = CHROMATICS.iter().position(|n| *n == self.tonic).unwrap();
        let mut chromatic_index = starting_index as usize;
        let mut retval: Vec<String> = vec!();
        for interval in &self.intervals {
            use self::SharpsOrFlats::*;
            let next_note = match (&CHROMATICS[chromatic_index], &self.sharp_flat) {
                (A, _) => "A",
                (AB, Sharps) => "A#",
                (AB, Flats) => "Bb",
                (B, _) => "B",
                (C, _) => "C",
                (CD, Sharps) => "C#",
                (CD, Flats) => "Db",
                (D, _) => "D",
                (DE, Sharps) => "D#",
                (DE, Flats) => "Eb",
                (E, _) => "E",
                (F, _) => "F",
                (FG, Sharps) => "F#",
                (FG, Flats) => "Gb",
                (G, _) => "G",
                (GA, Sharps) => "G#",
                (GA, Flats) => "Ab",
            };
            retval.push(next_note.to_string());
            chromatic_index = (chromatic_index + *interval as usize) % OCTAVE_INTERVALS;
        }

        retval
    }
}
