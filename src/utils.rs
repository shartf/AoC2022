use std::{
    env,
    fs::{self, File},
    io,
    path::Path,
};

pub fn read_api_key() -> io::Result<String> {
    let api_path = "secret/api.txt";
    let mut curr_path = env::current_dir()?;
    curr_path.push(api_path);
    // println!("{}", curr_path.display());
    let contents =
        fs::read_to_string(curr_path).expect("... not able to read api key. Is it there?");
    Ok(contents)
}

pub fn download_file(day_id: &str) -> Result<String, ureq::Error> {
    let prefix: &str = "https://adventofcode.com/2016/day/";
    let postfix: &str = "/input";
    let binding = format!("{prefix}{day_id}{postfix}",).to_string();
    let body: String = ureq::get(&binding)
        .set("Cookie", &read_api_key().unwrap())
        .call()?
        .into_string()?;
    // println!("Headers: \n{:#?}", body);
    Ok(body)
}

pub fn check_for_file(day_id: &str) {
    let binding = format!("{}{}{}", "files/day", day_id, ".txt").to_string();
    let path = Path::new(&binding);
    //exists is prone to TOCTOU and should not be used in serious code
    match path.exists() {
        true => println!("file is here"),
        false => {
            if let Ok(body) = download_file(day_id) {
                println!("body is: {body}")
                // TODO write a file and write that download is finished
            } else {
                panic!()
            }
        }
    }
}
