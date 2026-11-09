use super::*;

fn list(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

fn draw(
    deck: &mut TransitionDeck,
    entries: &[String],
    order: DeckOrder,
    count: usize,
) -> Vec<String> {
    (0..count).filter_map(|_| deck.next(entries, order, 42)).collect()
}

#[test]
fn in_order_walks_the_list_and_wraps() {
    let entries = list(&["burn", "ripple", "iris"]);
    let mut deck = TransitionDeck::default();
    assert_eq!(
        draw(&mut deck, &entries, DeckOrder::InOrder, 5),
        list(&["burn", "ripple", "iris", "burn", "ripple"])
    );
}

#[test]
fn shuffle_plays_everything_once_per_round() {
    let entries = list(&["a", "b", "c", "d", "e"]);
    let mut deck = TransitionDeck::default();
    let picks = draw(&mut deck, &entries, DeckOrder::Shuffle, 15);
    for round in picks.chunks(5) {
        let mut sorted = round.to_vec();
        sorted.sort();
        assert_eq!(sorted, entries);
    }
    for pair in picks.windows(2) {
        assert_ne!(pair[0], pair[1], "{picks:?}");
    }
}

#[test]
fn no_repeat_never_plays_one_twice_in_a_row() {
    let entries = list(&["a", "b", "c", "d"]);
    let mut deck = TransitionDeck::default();
    let picks = draw(&mut deck, &entries, DeckOrder::NoRepeat, 60);
    for pair in picks.windows(2) {
        assert_ne!(pair[0], pair[1], "{picks:?}");
    }
    for window in picks.windows(3) {
        assert!(window[0] != window[1] && window[1] != window[2] && window[0] != window[2]);
    }
    for entry in &entries {
        assert!(picks.contains(entry), "{entry} never played");
    }
    let two = list(&["a", "b"]);
    let mut deck = TransitionDeck::default();
    assert_eq!(
        draw(&mut deck, &two, DeckOrder::NoRepeat, 4).windows(2).filter(|p| p[0] == p[1]).count(),
        0
    );
}

#[test]
fn a_changed_list_starts_over() {
    let mut deck = TransitionDeck::default();
    let first = list(&["a", "b", "c"]);
    assert_eq!(deck.next(&first, DeckOrder::InOrder, 1).as_deref(), Some("a"));
    assert_eq!(deck.next(&first, DeckOrder::InOrder, 1).as_deref(), Some("b"));
    let second = list(&["x", "y"]);
    assert_eq!(deck.next(&second, DeckOrder::InOrder, 1).as_deref(), Some("x"));
    assert_eq!(deck.next(&[], DeckOrder::InOrder, 1), None);
    let one = list(&["solo"]);
    assert_eq!(draw(&mut deck, &one, DeckOrder::NoRepeat, 3), list(&["solo", "solo", "solo"]));
    assert_eq!(draw(&mut deck, &one, DeckOrder::Shuffle, 2), list(&["solo", "solo"]));
}

#[test]
fn lists_and_orders_parse() {
    assert_eq!(transition_list(" burn, ripple,,iris ,"), list(&["burn", "ripple", "iris"]));
    assert!(transition_list("").is_empty());
    assert_eq!(DeckOrder::parse("shuffle"), DeckOrder::Shuffle);
    assert_eq!(DeckOrder::parse("no-repeat"), DeckOrder::NoRepeat);
    assert_eq!(DeckOrder::parse("whatever"), DeckOrder::InOrder);
}
