#![allow(dead_code)]
mod game_state;
mod server;

use server::run_server;

fn main() {
    run_server();
}
