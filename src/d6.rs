use itertools::Itertools;

pub fn day_6() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("files/day6.txt").unwrap();

    let res = part_one(input);
    println!("{res}");
    Ok(())
}

fn part_one(parsed: String) -> usize {
    let idx = parsed
        .chars()
        .tuple_windows()
        .position(|(fst, scd, trd, fth)| {
            fst != scd && fst != trd && fst != fth && scd != trd && scd != fth && trd != fth
        });
    idx.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_provided_input() {
        let input_1 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let input_2 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let input_3 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let input_4 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(part_one(input_1), 5 as usize);
        assert_eq!(part_one(input_2), 6 as usize);
        assert_eq!(part_one(input_3), 10 as usize);
        assert_eq!(part_one(input_4), 11 as usize);
    }
}
