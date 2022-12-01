// use itertools::Itertools;

pub fn day_1() -> anyhow::Result<()> {
    let input: String = std::fs::read_to_string("files/day1.txt").unwrap();
    let inter = input.lines().collect::<Vec<_>>();

    let mut acc: u32 = 0;
    let mut result: Vec<u32> = vec![];
    // println!("output is {:?}", inter);
    for (idx, val) in inter.iter().enumerate() {
        if val.is_empty() && idx == 0 {
            println!("{}", idx);
            continue; // case where the empty line is the first value
        } else if val.is_empty() {
            result.push(acc);
            acc = 0;
        } else {
            let num_val = val.parse::<u32>();
            acc += num_val.unwrap();
        }
    }
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // dbg!(result);
    // get last 3 elements for results
    let last_3: u32 = result.as_slice()[result.len() - 3..]
        .to_vec()
        .into_iter()
        .sum();
    // dbg!(last_3);
    println!("sum is {last_3}");

    Ok(())
}
