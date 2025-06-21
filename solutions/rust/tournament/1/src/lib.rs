use std::collections::HashMap;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

struct WinLossTally {
    wins: u32,
    losses: u32,
    draws: u32,
}

enum MatchType {
    Win,
    Loss,
    Draw,
}

impl WinLossTally {
    pub fn new() -> Self {
        Self {
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    pub fn matches(&self) -> u32 {
        self.wins + self.losses + self.draws
    }

    pub fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }

    pub fn tally_match(&self, match_type: MatchType) -> WinLossTally {
        use self::MatchType::*;
        match match_type {
            Win => Self {
                wins: self.wins + 1,
                ..*self
            },
            Loss => Self {
                losses: self.losses + 1,
                ..*self
            },
            Draw => Self {
                draws: self.draws + 1,
                ..*self
            },
        }
    }
}

pub fn tally(match_results: &str) -> String {
    use self::MatchType::*;
    let mut retval: String = HEADER.to_string();
    let mut tallies: HashMap<&str, WinLossTally> = HashMap::new();
    for line in match_results.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut split_line = line.split(";");
        let err_msg = format!("Invalid line: {}", line);
        let first_team_name = split_line.next().expect(&err_msg);
        let second_team_name = split_line.next().expect(&err_msg);
        let result = split_line.next().expect(&err_msg);
        let first_result = match result {
            "win" => Win,
            "loss" => Loss,
            "draw" => Draw,
            _ => panic!("Invalid match result type"),
        };
        let second_result = match first_result {
            Win => Loss,
            Loss => Win,
            Draw => Draw,
        };
        let first_team_record = tallies
            .entry(first_team_name)
            .or_insert(WinLossTally::new());
        *first_team_record = first_team_record.tally_match(first_result);

        let second_team_record = tallies
            .entry(second_team_name)
            .or_insert(WinLossTally::new());
        *second_team_record = second_team_record.tally_match(second_result);
    }
    let mut tallies: Vec<_> = tallies.iter().collect();
    tallies.sort_by(|(a, awl), (b, bwl)| bwl.points().cmp(&awl.points()).then_with(|| a.cmp(&b)));

    for (_idx, (team_name, wld)) in tallies.iter().enumerate() {
        retval += "\n";
        retval += &format!(
            "{:31}| {:^3}| {:^3}| {:^3}| {:^3}|  {}",
            team_name,
            wld.matches(),
            wld.wins,
            wld.draws,
            wld.losses,
            wld.points()
        );
    }
    retval
}
