use itertools::Itertools;

pub fn day_4() -> anyhow::Result<()> {
    let input: String = std::fs::read_to_string("files/day4.txt").unwrap();
    let parsed = input.lines().collect_vec();
    // let res = first_part(parsed);
    let res = second_part(parsed);
    println!("Result is {res}");
    Ok(())
}

fn first_part(input: Vec<&str>) -> u32 {
    let mut acc: u32 = 0;
    for str in input {
        let sep = str.split(',').collect_vec(); // split on comma
        let first_range = sep[0]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect_vec();

        let second_range = sep[1]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect_vec();

        if first_range[0] <= second_range[0] && first_range[1] >= second_range[1] {
            acc += 1;
        } else if second_range[0] <= first_range[0] && second_range[1] >= first_range[1] {
            acc += 1;
        }
    }
    acc
}

fn second_part(input: Vec<&str>) -> u32 {
    let mut acc: u32 = 0;
    for str in input {
        let sep = str.split(',').collect_vec(); // split on comma
        let first_range = sep[0]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect_vec();

        let second_range = sep[1]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect_vec();

        if first_range[0] <= second_range[0] || first_range[1] >= second_range[1] {
            acc += 1;
        } else if second_range[0] <= first_range[0] || second_range[1] >= first_range[1] {
            acc += 1;
        }
    }
    acc
}
