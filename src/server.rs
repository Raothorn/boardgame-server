use serde_json::{json, Value};
use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};
use websocket::{
    sync::{Server, Writer},
    Message, OwnedMessage,
};

use crate::game_state::{
    action::{get_action, Action},
    GameState,
};

struct GameManager {
    state: GameState,
}

impl GameManager {
    fn execute_action(&mut self, action: &dyn Action) -> Option<String> {
        action.execute(&mut self.state)
    }

    fn restart(&mut self) {
        self.state = GameState::init_state();
    }
}

type Sender = Writer<TcpStream>;

struct ServerState {
    manager: Mutex<GameManager>,
    clients: Mutex<HashMap<String, Sender>>,
}

impl ServerState {
    fn add_client(&self, addr: &str, client: Sender) {
        let mut clients = self.clients.lock().unwrap();

        clients.insert(addr.to_owned(), client);
    }

    fn broadcast(&self, msg: &str) {
        let mut clients = self.clients.lock().unwrap();

        for client in clients.values_mut() {
            let _ = client.send_message(&Message::text(msg));
        }
    }

    fn broadcast_gamestate(&self) {
        let manager = self.manager.lock().unwrap();
        let message = json!({
            "msgType": "update",
            "msgData": manager.state,
        });

        self.broadcast(&message.to_string())
    }

    fn notify(&self, addr: &str, msg: &str) {
        let mut clients = self.clients.lock().unwrap();
        let client = clients.get_mut(addr);

        if let Some(client) = client {
            let message = json!({
                "msgType": "notify",
                "msgData": msg
            });
            client.send_message(&Message::text(message.to_string())).unwrap();
        }
    }

    fn prompt(&self, addr: &str, msg:&str) {
        println!("prompting: {}", msg);
        let mut clients = self.clients.lock().unwrap();
        let client = clients.get_mut(addr);

        if let Some(client) = client {
            let message = json!({
                "msgType": "prompt",
                "msgData": msg
            });
            client.send_message(&Message::text(message.to_string())).unwrap();
        }
    }

    fn handle_action_message(&self, addr: &str, msg: &str) {
        let mut manager = self.manager.lock().unwrap();
        let action = get_action(msg);

        println!("Action recieved: {}", action.name());
        if action.name() == "no action" {
            println!("Empty action message: {}", msg);
        }

        let result = manager.execute_action(action.as_ref());
        match result {
            Some(err) => {
                println!("Error executing action: {}", err);
                self.notify(addr, &err);
            }
            None => {
                println!("Action executed successfully.");

                drop(manager);
                self.broadcast_gamestate();
                let mut manager = self.manager.lock().unwrap();

                if let Some(prompt) = &manager.state.prompt {
                    self.prompt(addr, &prompt);
                    manager.state.prompt = None;
                    println!("prompting");
                }
            }
        }
    }

    fn handle_restart_message(&self) {
        println!("Restarting...");
        let mut manager = self.manager.lock().unwrap();
        manager.restart();
        drop(manager);
        self.broadcast_gamestate();
    }

    fn handle_message(&self, addr: &str, msg: &str) {
        let msg: Value = serde_json::from_str(msg).unwrap();

        if let Value::Object(obj) = msg {
            let msg_type = obj.get("msgType");
            let msg_data = obj.get("msgData");

            if let (Some(Value::String(msg_type)), Some(msg_data)) =
                (msg_type, msg_data)
            {
                match msg_type.as_str() {
                    "action" => {
                        self.handle_action_message(addr, &msg_data.to_string())
                    }
                    "restart" => self.handle_restart_message(),
                    _ => (),
                }
            }
        }
    }
}

pub fn run_server() {
    let state = Arc::new(ServerState {
        manager: Mutex::new(GameManager {
            state: GameState::init_state(),
        }),
        clients: Mutex::new(HashMap::new()),
    });
    let wsserver = Server::bind("localhost:2000").unwrap();

    for connection in wsserver.filter_map(Result::ok) {
        let state = Arc::clone(&state);
        // let clients = Arc::clone(&clients);
        let _ = thread::spawn(move || {
            let client = connection.accept().unwrap();
            let client_addr = client.peer_addr().unwrap().to_string();

            println!("{}", client_addr);

            // On initial connection

            let (mut reciever, sender) = client.split().unwrap();
            state.add_client(&client_addr, sender);

            state.broadcast_gamestate();
            // let mut client_list = clients.lock().unwrap();
            // client_list.push(sender);
            // drop(client_list);

            for message in reciever.incoming_messages() {
                match message {
                    Ok(OwnedMessage::Text(txt)) => {
                        state.handle_message(&client_addr, &txt);
                    }
                    Ok(OwnedMessage::Close(_)) => {
                        println!("closing");
                        break;
                    }
                    _ => {}
                }
            }
        });
    }
}
