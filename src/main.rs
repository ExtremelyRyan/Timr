use std::env;
use chrono;

fn main(){
    let args : Vec<String> = env::args().collect();
    //println!("{:?}",args);

    let keyword = &args[1].to_lowercase();
    //let flag = &args[2];

    println!("Keyword passed in: {}", keyword);

    // get dt of now, since we need it anyway regardless
    // if we are starting or stopping.
    let dt = chrono::offset::Local::now();


    // "start"

    match keyword.as_str() {
        "start" | "init" | "go" => todo!(),
        "stop"  | "end" => todo!(),
        _ => println!("Unknown keyword"),
    };

    println!("{:?}", dt);

}