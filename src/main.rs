#![allow(dead_code)]
mod game_state;

use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use websocket::{Message, OwnedMessage};
use websocket::sync::{Server, Writer};

use crate::game_state::action::get_action;

fn broadcast(clients: &mut Vec<Sender>, message: &str) {
    for client in clients {
        client.send_message(&Message::text(message)).unwrap();
    }
}

type Sender = Writer<TcpStream>;
fn run_server() {
    let server = Server::bind("localhost:2000").unwrap();
    // let clients = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for connection in server.filter_map(Result::ok) {
        // let clients = Arc::clone(&clients);
        let handle = thread::spawn(move || {
            let mut client = connection.accept().unwrap();

            let state = game_state::GameState::init_state();
            let state_data = serde_json::to_string(&state).unwrap();
            let message = Message::text(state_data);
            let _ = client.send_message(&message);

            let (mut reciever, _) = client.split().unwrap();

            // let mut client_list = clients.lock().unwrap();
            // client_list.push(sender);
            // drop(client_list);

            for message in reciever.incoming_messages() {

                match message {
                    Ok(OwnedMessage::Text(txt)) => {
                        println!("{}", txt);
                        let action = get_action(&txt);
                        println!("{}", action.name());
                    },
                    Ok(OwnedMessage::Close(_)) => {
                        println!("closing");
                        break;
                    },
                    _ => {}
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

}

fn main() {
    run_server();
}
