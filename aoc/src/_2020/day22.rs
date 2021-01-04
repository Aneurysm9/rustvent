use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        self.parse_input().play_simple().to_string()
    }

    fn run_b(&self) -> String {
        self.parse_input().play_recursive().1.to_string()
    }
}

impl Runner {
    fn parse_input(&self) -> Game {
        Game {
            states: HashSet::new(),
            hands: self
                .input
                .trim()
                .split("\n\n")
                .map(|p| {
                    p.lines()
                        .skip(1)
                        .map(|l| l.parse::<usize>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}

struct Game {
    states: HashSet<String>,
    hands: Vec<VecDeque<usize>>,
}

impl Game {
    fn play_simple(&mut self) -> usize {
        while self.hands[0].len() > 0 && self.hands[1].len() > 0 {
            let cards = self.draw();
            self.simple_round(cards);
        }
        self.score(if self.hands[0].len() > 0 { 0 } else { 1 })
    }

    fn simple_round(&mut self, (a, b): (usize, usize)) {
        if a > b {
            self.hands[0].push_back(a);
            self.hands[0].push_back(b);
        } else {
            self.hands[1].push_back(b);
            self.hands[1].push_back(a);
        }
    }

    fn play_recursive(&mut self) -> (usize, usize) {
        while self.hands[0].len() > 0 && self.hands[1].len() > 0 {
            if self.seen() {
                return (0, self.score(0));
            }

            let cards = self.draw();
            if cards.0 <= self.hands[0].len() && cards.1 <= self.hands[1].len() {
                let (winner, _) = Game {
                    states: HashSet::new(),
                    hands: self
                        .hands
                        .iter()
                        .enumerate()
                        .map(|(i, h)| {
                            h.iter()
                                .cloned()
                                .take(if i == 0 { cards.0 } else { cards.1 })
                                .collect()
                        })
                        .collect(),
                }
                .play_recursive();
                let (a, b) = if winner == 0 {
                    (cards.0, cards.1)
                } else {
                    (cards.1, cards.0)
                };
                self.hands[winner].push_back(a);
                self.hands[winner].push_back(b);
            } else {
                self.simple_round(cards);
            }
        }

        let winner = if self.hands[0].len() > 0 { 0 } else { 1 };
        (winner, self.score(winner))
    }

    fn draw(&mut self) -> (usize, usize) {
        (
            self.hands[0].pop_front().unwrap(),
            self.hands[1].pop_front().unwrap(),
        )
    }

    fn score(&self, winner: usize) -> usize {
        self.hands[winner]
            .iter()
            .enumerate()
            .fold(0, |acc, (i, v)| acc + *v * (self.hands[winner].len() - i))
    }

    fn seen(&mut self) -> bool {
        !self.states.insert(
            self.hands
                .iter()
                .fold(String::new(), |a, b| a + &b.iter().join("|") + "\n"),
        )
    }
}