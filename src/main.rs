use clap::{App, Arg};

fn main() {
    let matches = App::new("Pomodoro Application")
        .version("1.0")
        .author("Anass Belcaid anass.belcaid@gmail.com")
        .about("Pomodoro application where I will learn how to handle arguments and some SQL")
        .arg(
            Arg::with_name("duration")
                .short('d')
                .long("duration")
                .value_name("duration")
                .takes_value(true)
                .default_value("25")
                .help("duration of a single pomodoro")
                .validator(|x| match x.parse::<i32>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(String::from("please provide a valid duration")),
                }),
        )
        .get_matches();

    // Now you could manipulate the matches objects to get information on the arguments
    println!(
        "your pomodoro duration is {}",
        matches.value_of("duration").unwrap()
    );
}
