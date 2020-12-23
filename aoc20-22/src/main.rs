use std::collections::VecDeque;
use std::collections::HashSet;

fn play_round(hands: & mut [VecDeque<usize>; 2], recursive: bool) {
    let c = [hands[0].pop_front().unwrap(), hands[1].pop_front().unwrap()];
    
    let winner = if recursive && (c[0] <= hands[0].len()) && (c[1] <= hands[1].len()) {
        let mut sub_hands = [VecDeque::with_capacity(c[0]), VecDeque::with_capacity(c[1])];
        for p in 0..=1 {
            for i in 0..c[p] {
                sub_hands[p].push_back(hands[p][i]);
            }
        }
        play_game(& mut sub_hands ,true, &mut HashSet::new())}
    else if c[0] > c[1] {0} else {1};

    hands[winner].push_back(c[winner]);
    hands[winner].push_back(c[1-winner]);
}

fn play_game(hands: & mut [VecDeque<usize>; 2], recursive: bool, done_pos: &mut HashSet<[VecDeque<usize>; 2]>) -> usize{
    let mut hands = hands;
    let mut winner = 99;
    while winner == 99 {
        if recursive {
            if done_pos.contains(hands) { return 0;}
            done_pos.insert(hands.clone());
        }
        play_round(& mut hands, recursive);
        // println!("{:?}", hands[0]);
        // println!("{:?}", hands[1]);
        // println!("----------------");
        if hands[0].is_empty() {
            winner = 1;
        }
        else if hands[1].is_empty() {
            winner = 0;
        }
    }
    winner
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut hands : [VecDeque<usize>; 2]= Default::default();
    let mut idx = 0;
    for l in contents.lines() {
        if l == "" || l == "Player 1:"{
            continue;
        }
        if l == "Player 2:" {
            idx = 1;
            continue;
        }
        hands[idx].push_back(l.parse::<usize>().unwrap());
    }
    let mut game1 = hands.clone();
    let winner = play_game(& mut game1, false, &mut HashSet::new());
    let winner2 = play_game(& mut hands, true, &mut HashSet::new());
    let nb_cards = game1[winner].len() as usize;
    let mut result1 = 0;
    let mut result2 = 0;
    for i in 0..nb_cards{
        result1 += game1[winner][i]*(nb_cards-i);
        result2 += hands[winner2][i]*(nb_cards-i);
    } 
    println!("Winner game 1 score  is {}", result1);
    println!("Winner game 2 score  is {}", result2);
}
