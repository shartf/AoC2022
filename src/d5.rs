use itertools::Itertools;

pub fn day_5() -> anyhow::Result<()> {
    // Nikolaus brought me this input!!
    // No, seriously - is it advent of crappy parsing this year?
    let mut one: Vec<String> = vec![
        'N'.to_string(),
        'R'.to_string(),
        'G'.to_string(),
        'P'.to_string(),
    ];
    let mut two: Vec<String> = vec![
        'J'.to_string(),
        'T'.to_string(),
        'B'.to_string(),
        'L'.to_string(),
        'F'.to_string(),
        'G'.to_string(),
        'D'.to_string(),
        'C'.to_string(),
    ];
    let mut three: Vec<String> = vec!['M'.to_string(), 'S'.to_string(), 'V'.to_string()];
    let mut four: Vec<String> = vec![
        'L'.to_string(),
        'S'.to_string(),
        'R'.to_string(),
        'C'.to_string(),
        'Z'.to_string(),
        'P'.to_string(),
    ];
    let mut five: Vec<String> = vec![
        'P'.to_string(),
        'S'.to_string(),
        'L'.to_string(),
        'V'.to_string(),
        'C'.to_string(),
        'W'.to_string(),
        'D'.to_string(),
        'Q'.to_string(),
    ];
    let mut six: Vec<String> = vec![
        'C'.to_string(),
        'T'.to_string(),
        'N'.to_string(),
        'W'.to_string(),
        'D'.to_string(),
        'M'.to_string(),
        'S'.to_string(),
    ];
    let mut seven: Vec<String> = vec![
        'H'.to_string(),
        'D'.to_string(),
        'G'.to_string(),
        'W'.to_string(),
        'P'.to_string(),
    ];
    let mut eight: Vec<String> = vec![
        'Z'.to_string(),
        'L'.to_string(),
        'P'.to_string(),
        'H'.to_string(),
        'S'.to_string(),
        'C'.to_string(),
        'M'.to_string(),
        'V'.to_string(),
    ];
    let mut nine: Vec<String> = vec![
        'R'.to_string(),
        'P'.to_string(),
        'F'.to_string(),
        'L'.to_string(),
        'W'.to_string(),
        'G'.to_string(),
        'Z'.to_string(),
    ];
    let mut crates = vec![one, two, three, four, five, six, seven, eight, nine];
    let input = std::fs::read_to_string("files/day5.txt").unwrap();
    let parsed = input.lines().skip(10).collect_vec();
    let res_one = second_part(crates.to_vec(), parsed);

    Ok(())
}

fn first_part(mut crates: Vec<Vec<String>>, parsed: Vec<&str>) -> String {
    for line in parsed {
        let line_split = line.split(' ').collect_vec();
        let mut times: i32 = line_split[1].parse().unwrap();
        let from: usize = line_split[3].parse::<usize>().unwrap() - 1; // adjust for tuple indexing
        let to: usize = line_split[5].parse::<usize>().unwrap() - 1;

        while times > 0 {
            let crane;
            crane = crates[from].pop().unwrap();
            crates[to].push(crane.to_string());
            times -= 1;
        }
    }
    dbg!(crates);
    String::from("42")
}

fn second_part(mut crates: Vec<Vec<String>>, parsed: Vec<&str>) -> String {
    for line in parsed {
        let line_split = line.split(' ').collect_vec();
        let times: usize = line_split[1].parse().unwrap();
        let from: usize = line_split[3].parse::<usize>().unwrap() - 1; // adjust for tuple indexing
        let to: usize = line_split[5].parse::<usize>().unwrap() - 1;

        let mut crane: Vec<_>;
        let idx = crates[from].len() - (times);
        crane = crates[from].drain(idx..).collect();
        crates[to].append(&mut crane);
    }

    dbg!(crates);
    String::from("42")
}
