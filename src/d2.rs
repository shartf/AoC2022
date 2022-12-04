pub fn day_2() -> anyhow::Result<()> {
    let input: String = std::fs::read_to_string("files/day2.txt")
        .unwrap()
        .chars()
        .map(|x| match x {
            'X' => '1',
            'A' => '1',
            'B' => '2',
            'Y' => '2',
            'C' => '3',
            'Z' => '3',
            _ => x,
        })
        .collect::<String>();
    let parsed: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let total_score = calculate_score(parsed);
    println!("You total score is {total_score}");

    Ok(())
}

fn calculate_score(scores: Vec<String>) -> i32 {
    let mut acc: i32 = 0;
    for x in scores {
        //quick and dirty!
        let v: Vec<&str> = x.split(' ').collect();
        let elf: i32 = v[0].parse().unwrap();
        let you: i32 = v[1].parse().unwrap();
        // match spaghetti
        match (elf, you) {
            (1, 1) => acc += 3 + you, // draw
            (1, 2) => acc += 6 + you, // you won
            (1, 3) => acc += you,     // you lost R>S
            (2, 1) => acc += you,
            (2, 2) => acc += 3 + you,
            (2, 3) => acc += 6 + you,
            (3, 1) => acc += 6 + you, // you won R>S
            (3, 2) => acc += you,
            (3, 3) => acc += 3 + you, // draw
            _ => acc += 0,
        }
    }
    acc
}
