use itertools::Itertools;

pub fn day_6() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("files/day6.txt").unwrap();

    let res = part_two(input);
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
    idx.unwrap() + 4
}

fn part_two(parsed: String) -> usize {
    let bottom: usize = 0;
    let upper: usize = 16;
    while upper <= parsed.chars().count() {
        let slice = &parsed[bottom..=upper];
        let len = slice.len();
        if len > slice.unique().len() {
            break;
        } else {
            bottom += 1;
            upper += 1;
        }
    }
    upper
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

    #[test]
    fn test_part_two() {
        let input_1 = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let input_2 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let input_3 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let input_4 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

        assert_eq!(part_two(input_1), 19 as usize);
        assert_eq!(part_two(input_2), 23 as usize);
        assert_eq!(part_two(input_3), 23 as usize);
        assert_eq!(part_two(input_4), 29 as usize);
    }
}
