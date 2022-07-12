

#[derive(Debug)]
enum Color {
    Club,
    Diamond,
    Heart, 
    Spade,
}


enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,    
}

pub struct Card {
    color : Color,
    value : Value,
}


pub fn run()
{
    println!("Let's play klaverjas!");
}


fn score (cards: Vec<Card>, trump: Color) -> i32
{
    let mut score = 0;
    let card_it = cards.iter();
    for card in card_it {
        let val = calc_card_value(card, &trump);
        score += val;
    }

    score
}

fn calc_card_value(card: &Card, trump: &Color) -> i32
{
    println!("{:?}", trump);

    match &card.color
    {
        Color::Diamond => match card.value {
                Value::Jack => 20,
                Value::Nine => 14,
                Value::Ace => 11,
                Value::Ten => 10,
                Value::King => 4,
                Value::Queen => 3,
                _ => 0,
            },        
        _ =>  match card.value {
                Value::Ace => 11,
                Value::Ten => 10,
                Value::King => 4,
                Value::Queen => 3,
                Value::Jack => 3,
                _ => 0,
            }
        
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_test(){
        let mut cards : Vec<Card> = Vec::new();
        let card1 : Card = Card{color: Color::Club, value : Value::Ace};
        let card2 : Card = Card{color: Color::Diamond, value : Value::Nine};
        
        cards.push(card1);
        cards.push(card2);

        let trump : Color = Color::Diamond;

        let score : i32 = score(cards, trump);
        assert_eq!(score, 25);
    }


    
}