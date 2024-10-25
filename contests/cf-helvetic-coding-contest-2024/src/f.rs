use std::collections::HashMap;

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::point_2d::Point2D;

type Point = Point2D<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Team {
    Red,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    team: Team,
    id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity {
    Player(Player),
    Gate(Team),
    Quaffle,
    Bludger,
    Snitch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Move(Point),
    Catch,
    Throw,
}

impl Team {
    fn other(&self) -> Team {
        match self {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }

    fn name(&self) -> &str {
        match self {
            Team::Red => "RED",
            Team::Blue => "BLUE",
        }
    }
}

impl Player {
    fn name(&self) -> String {
        let team = match self.team {
            Team::Red => "R",
            Team::Blue => "B",
        };
        format!("{}{}", team, self.id)
    }
}

fn parse_entity(s: &str) -> Option<Entity> {
    match s {
        ".." => None,
        ".Q" => Some(Entity::Quaffle),
        ".B" => Some(Entity::Bludger),
        ".S" => Some(Entity::Snitch),
        "RG" => Some(Entity::Gate(Team::Red)),
        "BG" => Some(Entity::Gate(Team::Blue)),
        _ => {
            let team = match &s[..1] {
                "R" => Team::Red,
                "B" => Team::Blue,
                _ => unreachable!(),
            };
            let id = s[1..].parse().unwrap();
            Some(Entity::Player(Player { team, id }))
        }
    }
}

fn parse_direction(s: &str) -> Option<Point> {
    match s {
        "U" => Some(Point::new(0, -1)),
        "D" => Some(Point::new(0, 1)),
        "L" => Some(Point::new(-1, 0)),
        "R" => Some(Point::new(1, 0)),
        _ => None,
    }
}

fn parse_action(s: &str) -> Option<Action> {
    match s {
        "C" => Some(Action::Catch),
        "T" => Some(Action::Throw),
        _ => parse_direction(s).map(|d| Action::Move(d)),
    }
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.i32s();
    let mut player_pos = HashMap::new();
    let mut quaffle_pos = Point::new(-1, -1);
    let mut quaffle_carried_by = None;
    let mut bludger_pos = Point::new(-1, -1);
    let mut snitch_pos = Point::new(-1, -1);
    let mut gates = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            if let Some(e) = parse_entity(&read.word()) {
                match e {
                    Entity::Player(p) => {
                        player_pos.insert(p, Point::new(j, i));
                    }
                    Entity::Gate(team) => {
                        gates.insert(Point::new(j, i), team);
                    }
                    Entity::Quaffle => {
                        quaffle_pos = Point::new(j, i);
                    }
                    Entity::Bludger => {
                        bludger_pos = Point::new(j, i);
                    }
                    Entity::Snitch => {
                        snitch_pos = Point::new(j, i);
                    }
                }
            }
        }
    }

    let mut score: HashMap<Team, i32> = HashMap::from_iter([(Team::Red, 0), (Team::Blue, 0)]);
    let total_time = read.usize();
    for t in 0..total_time {
        let entity = parse_entity(&read.word()).unwrap();
        let action = parse_action(&read.word()).unwrap();
        match (entity, action) {
            (Entity::Gate(_), _) => panic!(),
            (Entity::Player(p), Action::Move(d)) => {
                *player_pos.get_mut(&p).unwrap() += d;
                if quaffle_carried_by == Some(p) {
                    quaffle_pos = player_pos[&p];
                }
            }
            (Entity::Quaffle, Action::Move(d)) => {
                quaffle_pos += d;
            },
            (Entity::Bludger, Action::Move(d)) => {
                bludger_pos += d;
            },
            (Entity::Snitch, Action::Move(d)) => {
                snitch_pos += d;
            },
            (Entity::Player(p), Action::Catch) => {
                let target = parse_entity(&read.word()).unwrap();
                match target {
                    Entity::Quaffle => {
                        assert_eq!(quaffle_pos, player_pos[&p]);
                        quaffle_carried_by = Some(p);
                    }
                    Entity::Snitch => {
                        assert_eq!(snitch_pos, player_pos[&p]);
                        *score.get_mut(&p.team).unwrap() += 10;
                        emitln!(write, t, p.team.name(), "CATCH GOLDEN SNITCH");
                    }
                    _ => panic!(),
                }
            }
            (_, Action::Catch) => panic!(),
            (Entity::Player(p), Action::Throw) => {
                quaffle_carried_by = None;
                if let Some(gate_owner) = gates.get(&quaffle_pos) {
                    let goal_team = gate_owner.other();
                    *score.get_mut(&goal_team).unwrap() += 1;
                    emitln!(write, t, goal_team.name(), "GOAL");
                    quaffle_pos = Point::new(m / 2, n / 2);
                }
            }
            (_, Action::Throw) => panic!(),
        }

        let mut eliminated_players = Vec::new();
        player_pos.retain(|p, pos| {
            let elimiated = *pos == bludger_pos;
            if elimiated {
                if quaffle_carried_by == Some(*p) {
                    quaffle_carried_by = None;
                }
                eliminated_players.push(p.name());
            }
            !elimiated
        });
        eliminated_players.sort();
        for name in eliminated_players {
            emitln!(write, t, name, "ELIMINATED");
        }
    }

    emitln!(write, "FINAL SCORE:", score[&Team::Red], score[&Team::Blue]);
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        5 5
        .. R1 .. B1 ..
        RG .. .. .. BG
        RG R0 .Q B0 BG
        RG .. .. .. BG
        .. R2 .. B2 ..
        13
        B2 U
        B2 U
        B2 L
        B2 C .Q
        B2 L
        B2 L
        B2 T
        R0 R
        R0 C .Q
        R0 D
        R0 R
        R0 R
        R0 T"), "\
        6 BLUE GOAL
        12 RED GOAL
        FINAL SCORE: 1 1");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 5
        .. .. R0 .. ..
        RG .. .Q .. BG
        .. .. B0 .. ..
        12
        R0 D
        R0 C .Q
        R0 R
        R0 T
        R0 D
        B0 R
        B0 U
        B0 C .Q
        B0 L
        B0 L
        B0 L
        B0 T
        "), "\
        11 BLUE GOAL
        FINAL SCORE: 0 1");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 5
        .. .. R0 .. ..
        RG .. .Q .. BG
        .. .. B0 .. ..
        5
        R0 D
        R0 C .Q
        R0 L
        R0 L
        R0 T"), "\
        4 BLUE GOAL
        FINAL SCORE: 0 1");

        assert_trimmed_eq!(&run_solver(solve, "\
        3 5
        .. .. R0 .. ..
        RG .. .Q .. BG
        .. .. B0 .. ..
        12
        R0 D
        R0 C .Q
        R0 R
        R0 T
        R0 D
        B0 R
        B0 U
        B0 C .Q
        B0 L
        B0 L
        B0 L
        B0 T"), "\
        11 BLUE GOAL
        FINAL SCORE: 0 1");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 5
        .. .. R0 .. ..
        RG .. .Q .. BG
        .. .. B0 .. ..
        5
        R0 D
        R0 C .Q
        R0 L
        R0 L
        R0 T"), "\
        4 BLUE GOAL
        FINAL SCORE: 0 1");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 5
        .. .. .. .. ..
        .. .. .. .. ..
        RG R0 .Q B0 BG
        .. .. .. .. ..
        .. .. .B .. ..
        5
        .B L
        .B U
        .B U
        B0 L
        B0 L"), "\
        2 R0 ELIMINATED
        4 B0 ELIMINATED
        FINAL SCORE: 0 0");

        assert_trimmed_eq!(&run_solver(solve, "\
        3 5
        .. .. R0 .. ..
        RG .. .Q .. BG
        .. .. B0 .. ..
        12
        R0 D
        R0 C .Q
        R0 R
        R0 T
        R0 D
        B0 R
        B0 U
        B0 C .Q
        B0 L
        B0 L
        B0 L
        B0 T"), "\
        11 BLUE GOAL
        FINAL SCORE: 0 1");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 5
        .. .. R0 .. ..
        RG .. .Q .. BG
        .. .. B0 .. ..
        5
        R0 D
        R0 C .Q
        R0 L
        R0 L
        R0 T"), "\
        4 BLUE GOAL
        FINAL SCORE: 0 1");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 5
        .. .. .. .. ..
        .. .. .. .. ..
        RG R0 .Q B0 BG
        .. .. .. .. ..
        .. .. .B .. ..
        5
        .B L
        .B U
        .B U
        B0 L
        B0 L"), "\
        2 R0 ELIMINATED
        4 B0 ELIMINATED
        FINAL SCORE: 0 0");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 5
        .. R0 .S B0 ..
        .. .. .. .. ..
        RG .. .Q .. BG
        .. .. .. .. ..
        .. R1 .B B1 ..
        4
        .S D
        R0 D
        R0 R
        R0 C .S"), "\
        3 RED CATCH GOLDEN SNITCH
        FINAL SCORE: 10 0");

        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
