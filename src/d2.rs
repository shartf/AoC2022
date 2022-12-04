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

    Ok(())
}

fn calculate_score(scores: Vec<String>) -> u32 {
    let acc: i32 = 0;
    for x in scores {
        //quick and dirty!
        let v: Vec<&str> = x.split(' ').collect();
        // let elf: i32 = x.as_bytes()[0].parse::<i32>().unwrap();
        // let you: i32 = x.as_bytes()[2].parse::<i32>().unwrap;
        // println! ("parsed to {} {}", elf, you)
        dbg!(v);
    }
    42
}
