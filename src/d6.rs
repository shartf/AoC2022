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
