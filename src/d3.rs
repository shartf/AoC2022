use itertools::Itertools;

pub fn day_3() -> anyhow::Result<()> {
    let input: String = std::fs::read_to_string("files/day3.txt").unwrap();
    let parsed: Vec<&str> = input.split('\n').collect_vec();
    let mut acc: u32 = 0;
    let mut idx = 0;
    while idx < parsed.len() - 3 {
        // println!("v.len is {}, start{} === {} stop", len, fst, scd);
        let fst = parsed[idx];
        let scd = parsed[idx + 1];
        let trd = parsed[idx + 2];
        'charloop: for c in fst.chars() {
            if scd.contains(c) && trd.contains(c) {
                acc += mapping(c);
                // need to break on first hit, otherwise we get duplicates
                break 'charloop;
            }
        }
        idx += 3;
    }
    println!("acc is {acc}");

    Ok(())
}

fn mapping(ch: char) -> u32 {
    let mut ascii = ch as u32;
    if ascii < 97 {
        // uppercase ascii, starts with 65 (A) and ends with 90 (Z)
        ascii -= 38
    } else {
        // lowercase ascii, starts wit a (97) and ends with z (122)
        ascii -= 96
    }
    ascii
}
