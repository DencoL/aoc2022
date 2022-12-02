use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl FromStr for Play {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err(())
        }
    }
}

impl Play {
    pub fn beats(&self) -> Play {
        return match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper
        }
    }

    pub fn can_beat(&self, other_play: &Play) -> bool {
        return self.beats() == *other_play;
    }

    pub fn is_draw(first_play: &Play, second_play: &Play) -> bool {
        return first_play == second_play;
    } 

    pub fn loses_to(&self) -> Play {
        return match *self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock
        }
    }

    pub fn get_score(&self) -> u8 {
        return *self as u8;
    }
}

pub enum ExpectedGameResult {
    MyWin,
    MyLoss,
    Draw
}

impl FromStr for ExpectedGameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(ExpectedGameResult::MyLoss),
            "Y" => Ok(ExpectedGameResult::Draw),
            "Z" => Ok(ExpectedGameResult::MyWin),
            _ => Err(())
        }
    }
}
