pub mod day_twenty_two {
    use std::collections::VecDeque;
    use std::fmt;
    use itertools::Itertools;

    type Card = i32;
    struct Deck {
        cards: VecDeque<Card>,
    }
    struct Game {
        one: Deck,
        two: Deck,
    }
    impl Game {
        fn play(&mut self) -> &Deck {
            let mut winner: &Deck;
            loop {
                if self.one.cards.is_empty() {
                    winner = &self.two;
                    break
                } else if self.two.cards.is_empty() {
                    winner = &self.one;
                    break
                }
                let (card_one, card_two) = (self.one.play().unwrap(), self.two.play().unwrap());
                if card_one < card_two {
                    self.two.take(vec![card_two, card_one])
                } else {
                    self.one.take(vec![card_one, card_two])
                }
            }
            winner
        }
        fn recurse(&self) -> &Deck {

        }
    }
    impl Deck {
        fn play(&mut self) -> Option<Card> {
            self.cards.pop_front()
        }
        fn take(&mut self, cards: Vec<Card>) {
            for card in cards {
                self.cards.push_back(card)
            }
        }
    }
    impl fmt::Display for Deck {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.cards.iter().map(|c| c.to_string()).join(" "))
        }
    }
    fn deckify(input: &str) -> Deck {
        Deck{cards: input.lines().skip(1).filter_map(|l| l.parse::<i32>().ok()).collect::<VecDeque<i32>>()}
    }
    pub fn run_one(input: String) {
        let cut = input.find("\n\n").expect("No blank line?!");
        let (fore, aft) = input.split_at(cut);
        let mut one = deckify(fore);
        let mut two = deckify(aft);
        let mut game = Game{one, two};
        let winner = game.play();
        let mut score = 0;
        for (i, card) in winner.cards.iter().rev().enumerate() {
            let this = (i as i32 + 1) * *card;
            score += this;
        }
        println!("Winning score: {}", score);

    }
}