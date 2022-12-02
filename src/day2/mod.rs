use std::collections::HashMap;

pub fn day2() {
    println!("{:?}", part_one());
    println!("{:?}", part_two());
}


fn get_selection_score(s : &str) -> i32 {
    let selection_score: HashMap<&str, i32>  = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    match selection_score.get(s) {
        Some(i) => return *i,
        None => return 0,
    };
}


fn get_round_score(player_action: &str, opponent_action: &str) -> i32 {

    return match player_action { //Player picks:
        "X" => match opponent_action { //Rock
            "A" => 3,
            "B" => 0,
            "C" => 6,
            _ => 0,
        } 
        "Y" => match opponent_action { //Papper
            "A" => 6,
            "B" => 3,
            "C" => 0,
            _ => 0,
        }
        "Z" => match opponent_action { //Scicors
            "A" => 0,
            "B" => 6,
            "C" => 3,
            _ => 0,
        }
        _ => 0,
    };

}


fn outcome_mapper(s : &str) -> i32 {
    let outcome: HashMap<&str, i32>  = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);

    match outcome.get(s) {
        Some(i) => return *i,
        None => return 0,
    };
}


fn play_round(opponent_action: &str, player_action: &str) -> i32 {
    return get_selection_score(player_action) + get_round_score(player_action, opponent_action);
}


fn part_one() -> i32 {
    let f: &str = include_str!("input.txt");
    let inn: Vec<&str> = f.lines().collect();

    let mut score = 0;
    for ele in inn {

        let actions: Vec<&str> = ele.split_whitespace().collect();
        let opponet_pick: &str = actions[0];
        let player_pick: &str = actions[1];

        score += play_round(opponet_pick, player_pick);
    }    
    
    return score;
}


fn part_two() -> i32 {

    let f: &str = include_str!("input.txt");
    let inn: Vec<&str> = f.lines().collect();

    let mut score = 0;
    for ele in inn {
        let actions: Vec<&str> = ele.split_whitespace().collect();
        let opponet_pick: &str = actions[0];
        let round_outcome: &str = actions[1];
        let mut player_pick = "";

        for c in ["X", "Y", "Z"] {
            if get_round_score(c, opponet_pick) == outcome_mapper(round_outcome) {
                player_pick = c;
            }
        }

        score += play_round(opponet_pick, player_pick);
    }    
    
    return score;
}