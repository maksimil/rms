use std::env;

mod join;
mod server;

const SERVER_COMMAND: &str = "server";
const JOIN_COMMAND: &str = "join";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough args");
    }

    match args[1].as_str() {
        SERVER_COMMAND => {
            if args.len() < 3 {
                panic!("Not enough args");
            }

            server::start_server(&args[2]);
        }
        JOIN_COMMAND => {
            if args.len() < 3 {
                panic!("Not enough args");
            }

            join::join_server(&args[2]);
        }
        s => {
            panic!("{} is an unknown commad", s);
        }
    }
}
