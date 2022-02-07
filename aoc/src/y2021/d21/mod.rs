use std::collections::HashMap;

pub fn main(input: &str) -> String {
    let (p1, p2) = input.split_once('\n').unwrap();
    let p1 = parse_player_pos(p1, 1);
    let p2 = parse_player_pos(p2, 2);
    let a = find_a(p1, p2);
    let b = find_b(p1, p2);
    format!("{} {}", a, b)
}

fn parse_player_pos(input: &str, player: u64) -> u64 {
    let (_, p) = input
        .split_once(&format!("Player {} starting position: ", player))
        .unwrap();
    p.parse::<u64>().unwrap()
}

fn find_a(p1: u64, p2: u64) -> u64 {
    let mut state = GameState::new(p1, p2);
    let mut rolls = 0;
    loop {
        for _ in 0..3 {
            rolls += 1;
            state = state.roll(rolls);
        }
        state = state.next_turn();
        if state.score1 >= 1000 {
            return rolls * state.score2;
        }
        if state.score2 >= 1000 {
            return rolls * state.score1;
        }
    }
}

fn find_b(p1: u64, p2: u64) -> u64 {
    let mut universes = MemoizedUniverse::default();
    let (wins1, wins2) = universes.count_winners(GameState::new(p1, p2));
    wins1.max(wins2)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameState {
    score1: u64,
    score2: u64,
    pawn1:  u64,
    pawn2:  u64,
    turn:   u8,
}

impl GameState {
    fn new(pawn1: u64, pawn2: u64) -> Self {
        Self {
            score1: 0,
            score2: 0,
            pawn1,
            pawn2,
            turn: 1,
        }
    }

    fn roll(&self, die: u64) -> Self {
        let mut next = self.clone();
        if self.turn == 1 {
            next.pawn1 = (self.pawn1 + die - 1) % 10 + 1;
        } else {
            next.pawn2 = (self.pawn2 + die - 1) % 10 + 1;
        }
        next
    }

    fn next_turn(&self) -> Self {
        let mut next = self.clone();
        if self.turn == 1 {
            next.score1 += self.pawn1;
        } else {
            next.score2 += self.pawn2;
        }
        next.turn = next.turn % 2 + 1;
        next
    }
}

#[derive(Default)]
struct MemoizedUniverse {
    mem: HashMap<GameState, (u64, u64)>,
}

impl MemoizedUniverse {
    fn count_winners(&mut self, state1: GameState) -> (u64, u64) {
        if let Some(&cached) = self.mem.get(&state1) {
            return cached;
        }
        if state1.score1 >= 21 {
            return (1, 0);
        }
        if state1.score2 >= 21 {
            return (0, 1);
        }
        let mut states = vec![state1.clone()];
        for _throw in 1..=3 {
            for state2 in states.drain(..).collect::<Vec<GameState>>().iter() {
                for die in 1..=3 {
                    states.push(state2.roll(die));
                }
            }
        }
        let (wins1, wins2) = states
            .into_iter()
            .map(|state2| self.count_winners(state2.next_turn()))
            .reduce(|(wins1, wins2), (w1, w2)| (wins1 + w1, wins2 + w2))
            .unwrap();
        self.mem.insert(state1, (wins1, wins2));
        (wins1, wins2)
    }
}
