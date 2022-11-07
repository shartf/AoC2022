use std::{env, fs, io};

pub fn read_api_key() -> io::Result<String> {
    let api_path = "secret/api.txt";
    let mut curr_path = env::current_dir()?;
    curr_path.push(api_path);
    let contents =
        fs::read_to_string(curr_path).expect("... not able to read api key. Is it there?");
    Ok(contents)
}

pub fn download_file() -> Result<(), ureq::Error> {
    let body: String = ureq::get("https://adventofcode.com/2016/day/2/input")
        .set("Cookie", &read_api_key().unwrap())
        .call()?
        .into_string()?;
    println!("Headers: \n{:#?}", body);
    Ok(())
}
