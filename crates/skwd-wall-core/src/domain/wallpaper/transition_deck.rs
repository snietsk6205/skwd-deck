//! Custom build (M3): which transition of a playlist plays next.
//!
//! Pure state, owned by `ApplyRuntime`: "in order" walks the list, "shuffle"
//! plays every entry once per round in a new random order, "no repeat"
//! picks at random but never one of the most recent picks (about half the
//! list), so the same transition never plays twice in a row.

use crate::domain::random::xorshift64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeckOrder {
    InOrder,
    Shuffle,
    NoRepeat,
}

impl DeckOrder {
    pub fn parse(order: &str) -> Self {
        match order {
            "shuffle" => Self::Shuffle,
            "no-repeat" => Self::NoRepeat,
            _ => Self::InOrder,
        }
    }
}

#[derive(Debug, Default)]
pub struct TransitionDeck {
    /// The list the state below belongs to; a different list starts over.
    entries: Vec<String>,
    order: Option<DeckOrder>,
    /// In order: the next index. Shuffle: position in `round`.
    cursor: usize,
    /// Shuffle: this round's order (indexes into `entries`).
    round: Vec<usize>,
    /// No repeat: the most recent picks, newest last.
    recent: Vec<usize>,
    seed: u64,
}

impl TransitionDeck {
    /// The next entry of `entries` in `order`; `None` for an empty list.
    /// `seed` feeds the random orders the first time (any value).
    pub fn next(&mut self, entries: &[String], order: DeckOrder, seed: u64) -> Option<String> {
        if entries.is_empty() {
            return None;
        }
        if self.entries != entries || self.order != Some(order) {
            *self = Self {
                entries: entries.to_vec(),
                order: Some(order),
                seed: if self.seed == 0 { seed | 1 } else { self.seed },
                ..Self::default()
            };
        }
        let count = entries.len();
        let index = match order {
            DeckOrder::InOrder => {
                let index = self.cursor % count;
                self.cursor = (index + 1) % count;
                index
            }
            DeckOrder::Shuffle => {
                if self.cursor >= self.round.len() {
                    let last = self.round.last().copied();
                    self.round = (0..count).collect();
                    for i in (1..count).rev() {
                        let j = (xorshift64(&mut self.seed) % (i as u64 + 1)) as usize;
                        self.round.swap(i, j);
                    }
                    // A new round never starts with the one that just played.
                    if count > 1 && last == self.round.first().copied() {
                        self.round.swap(0, count - 1);
                    }
                    self.cursor = 0;
                }
                let index = self.round[self.cursor];
                self.cursor += 1;
                index
            }
            DeckOrder::NoRepeat => {
                let keep = (count / 2).max(usize::from(count > 1));
                while self.recent.len() > keep {
                    self.recent.remove(0);
                }
                let free: Vec<usize> = (0..count).filter(|i| !self.recent.contains(i)).collect();
                let index = free[(xorshift64(&mut self.seed) % free.len() as u64) as usize];
                self.recent.push(index);
                if self.recent.len() > keep {
                    self.recent.remove(0);
                }
                index
            }
        };
        Some(entries[index].clone())
    }
}

/// Splits a comma list ("burn, ripple,,iris") into its entries.
pub fn transition_list(text: &str) -> Vec<String> {
    text.split(',').map(str::trim).filter(|entry| !entry.is_empty()).map(String::from).collect()
}

#[cfg(test)]
#[path = "transition_deck_tests.rs"]
mod tests;
