use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str;

pub fn main(input: &str) -> String {
    let a = find_a(input);
    let b = find_b(input);
    format!("{} {}", a, b)
}

fn find_a(input: &str) -> u64 {
    let burrow = input.parse::<SmallBurrow>().unwrap();
    let all_home_a = concat!(
        "#############",
        "#...........#",
        "###A#B#C#D###",
        "###A#B#C#D###",
        "#############",
    );
    let final_state = all_home_a.parse::<SmallBurrow>().unwrap();
    burrow.go_home_little_ones(&final_state)
}

fn find_b(input: &str) -> u64 {
    let mut input: Vec<&str> = input.lines().collect();
    input.insert(3, "  #D#C#B#A#  ");
    input.insert(4, "  #D#B#A#C#  ");
    let burrow = input.join("\n").parse::<LargeBurrow>().unwrap();
    let all_home_b = concat!(
        "#############",
        "#...........#",
        "###A#B#C#D###",
        "###A#B#C#D###",
        "###A#B#C#D###",
        "###A#B#C#D###",
        "#############",
    );
    let final_state = all_home_b.parse::<LargeBurrow>().unwrap();
    burrow.go_home_little_ones(&final_state)
}

type SmallBurrow = Burrow<13, 5>;
type LargeBurrow = Burrow<13, 7>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Slot {
    Amber,
    Bronze,
    Copper,
    Desert,
    Empty,
    Wall,
}
use Slot::{Amber, Bronze, Copper, Desert, Empty, Wall};

impl Slot {
    fn move_cost(&self) -> u64 {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
            _ => panic!("slot '{:#?}' is not movable", self),
        }
    }

    fn y_home(&self) -> usize {
        match self {
            Amber => 3,
            Bronze => 5,
            Copper => 7,
            Desert => 9,
            _ => panic!("slot '{:#?}' does not have a home", self),
        }
    }

    fn is_shrimp(&self) -> bool {
        matches!(self, Amber | Bronze | Copper | Desert)
    }

    fn is_wall(&self) -> bool {
        matches!(self, Wall)
    }

    fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }
}

impl str::FromStr for Slot {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Amber),
            "B" => Ok(Bronze),
            "C" => Ok(Copper),
            "D" => Ok(Desert),
            "." => Ok(Empty),
            "#" | " " => Ok(Wall),
            s => Err(format!("unknown slot '{}'", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Burrow<const W: usize, const H: usize>
where
    [Slot; W * H]: Sized,
{
    slots: [Slot; W * H],
}

impl<const W: usize, const H: usize> str::FromStr for Burrow<W, H>
where
    [Slot; W * H]: Sized,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: String = s.chars().filter(|&ch| ch != '\n').collect();
        let mut slots = [Wall; W * H];
        for (i, slot) in slots.iter_mut().enumerate() {
            if i < input.len() {
                let input_slot = &input[i..i + 1];
                *slot = input_slot.parse::<Slot>()?
            }
        }
        Ok(Self { slots })
    }
}

impl<const W: usize, const H: usize> Burrow<W, H>
where
    [Slot; W * H]: Sized,
    Burrow<W, H>: Ord,
{
    /// Dijkstra's algorithm over burrow states where distance is move energy
    fn go_home_little_ones(&self, final_state: &Burrow<W, H>) -> u64 {
        let mut visited = HashSet::<Self>::new();
        let mut energy = HashMap::<Self, u64>::new();
        let mut priority = BinaryHeap::<(Reverse<u64>, Self)>::new();

        priority.push((Reverse(0), self.clone()));
        energy.insert(self.clone(), 0);

        while !priority.is_empty() {
            let (_, state) = priority.pop().unwrap();
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());

            for (src, dst, step_count) in state.possible_moves().into_iter() {
                let cost_start2current = *energy.get(&state).unwrap();
                let (next_state, cost_per_step) = state.transition(src, dst);
                let cost_current2next = step_count * cost_per_step;
                let cost_start2next = cost_start2current + cost_current2next;

                let prev_cost_start2next =
                    energy.get(&next_state).unwrap_or(&u64::max_value());
                if cost_start2next < *prev_cost_start2next {
                    energy.insert(next_state.clone(), cost_start2next);
                    priority
                        .push((Reverse(cost_start2next), next_state.clone()));
                }
            }
        }

        *energy.get(final_state).unwrap()
    }

    fn possible_moves(&self) -> Vec<(usize, usize, u64)> {
        let is_hallway = |i: usize| -> bool { W < i && i < 2 * W - 1 };
        let is_blocked_hallway =
            |i: usize| -> bool { [3, 5, 7, 9].iter().any(|j| j + W == i) };

        let mut moves = vec![];
        for src in self.free_shrimps().into_iter() {
            let mut candidate_moves = vec![];
            let mut visited: HashSet<usize> = [src].into();
            let mut queue = vec![(src, 0)];
            while !queue.is_empty() {
                let (pos, step_count) = queue.remove(0);
                let adjacent = [pos - 1, pos - W, pos + 1, pos + W];
                for dst in adjacent {
                    if visited.contains(&dst) {
                        continue;
                    }
                    visited.insert(dst);
                    if !self.slots[dst].is_empty() {
                        continue;
                    }
                    queue.push((dst, step_count + 1));
                    if is_hallway(src) && is_hallway(dst) {
                        continue;
                    }
                    if is_blocked_hallway(dst) {
                        continue;
                    }
                    if !is_hallway(dst) && !self.can_move_home(src, dst) {
                        continue;
                    }
                    candidate_moves.push((src, dst, step_count + 1));
                }
            }
            // Move home greedily, else use all candidate moves
            if let Some(&move_to_home) = candidate_moves
                .iter()
                .find(|(src, dst, _)| self.can_move_home(*src, *dst))
            {
                moves.push(move_to_home);
            } else {
                moves.extend(candidate_moves.into_iter());
            }
        }
        moves
    }

    fn transition(&self, src: usize, dst: usize) -> (Self, u64) {
        let mut next = self.clone();
        next.slots[src] = Empty;
        next.slots[dst] = self.slots[src];
        let cost = self.slots[src].move_cost();
        (next, cost)
    }

    fn free_shrimps(&self) -> Vec<usize> {
        (W + 1..W * (H - 1))
            .filter(|&i| self.slots[i].is_shrimp() && !self.shrimp_is_home(i))
            .collect()
    }

    fn shrimp_is_home(&self, i: usize) -> bool {
        let s = self.slots[i];
        s.is_shrimp()
            && self.is_home_for(i, i)
            && (self.slots[i + W].is_wall() || self.shrimp_is_home(i + W))
    }

    fn can_move_home(&self, src: usize, dst: usize) -> bool {
        self.is_home_for(src, dst)
            && self.all_empty_until(dst)
            && self.all_home_below(dst)
    }

    fn is_home_for(&self, src: usize, dst: usize) -> bool {
        dst > 2 * W && self.slots[src].y_home() == dst % W
    }

    fn all_empty_until(&self, dst: usize) -> bool {
        (2..=dst / W)
            .map(|y| dst % W + y * W)
            .all(|i| self.slots[i].is_empty())
    }

    fn all_home_below(&self, dst: usize) -> bool {
        (dst / W + 1..H - 1)
            .map(|y| dst % W + y * W)
            .all(|i| self.shrimp_is_home(i))
    }
}
