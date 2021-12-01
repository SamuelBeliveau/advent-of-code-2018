pub fn solve_9_a() {
//    let players = 9usize;
    let players = 462usize;
//    let max_marble_score = 150u32;
    let max_marble_score = 7193800u32;


    let mut scores: Vec<u32> = vec![0; players];
    let mut curr_marble = 0u32;
    let mut curr_marble_index = 0usize;
    let mut last_marble_score: u32;

    let mut ring: Vec<u32> = Vec::with_capacity(max_marble_score as usize);
    ring.push(curr_marble);
    curr_marble += 1;

    while curr_marble < max_marble_score {
        let player_index = (curr_marble - 1) as usize % players;
        if curr_marble % 100000 == 0 {
            println!("Current marble = {}", curr_marble);
        }
//        println!("Starting loop with {} at index {}", curr_marble, curr_marble_index);
        if curr_marble > 0 && curr_marble % 23 == 0 {
            let index_to_remove = curr_marble_index as isize - 7;
            let marble_index_to_remove = if index_to_remove < 0 { (ring.len() as isize + index_to_remove) as usize } else { index_to_remove as usize };
//            println!("1 = {}", index_to_remove);
//            println!("2 = {}", ring.len());
//            println!("marble_index_to_remove = {}", marble_index_to_remove);
            let marble_to_remove = ring.remove(marble_index_to_remove);

            last_marble_score = curr_marble + marble_to_remove;
//            println!("Scoring {} and removing {} = {}", curr_marble, marble_to_remove, last_marble_score);

            scores[player_index] += last_marble_score;

            curr_marble += 1;
            curr_marble_index = marble_index_to_remove;
            continue;
        }


        curr_marble_index = (curr_marble_index + 2) % ring.len();
//        println!("Inserting {} at index {} in array of size {}", curr_marble, curr_marble_index, ring.len());
        ring.insert(curr_marble_index, curr_marble);
//        println!("[{}] {:?}", player_index + 1, ring);
        curr_marble += 1;
    }

    println!("{:?}", scores.iter().max());
}