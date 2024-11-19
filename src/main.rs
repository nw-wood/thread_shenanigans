use std::{sync::{Arc, Mutex}, thread, time::Duration};
use rand::Rng;

struct Application {
    new_messages: Vec<String>,
}

impl Application {
    fn add_input(&mut self, input: String) {
        self.new_messages.push(input);
    }

    fn draw(&mut self) {
        for item in &self.new_messages {
            println!(" draw*: {item}");
        }
        self.new_messages = vec!();
    }

    fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            Self {
                new_messages: Vec::new(),
            }
        ))
    }
}

fn main() {
    let app = Application::new();
    let io_ref = app.clone();
    let server_ref = app.clone();
    let mut handles = vec!();
    
    handles.push(thread::spawn(move || user_input_thread(io_ref)));
    handles.push(thread::spawn(move || server_thread(server_ref)));
    handles.push(thread::spawn(move || drawing_thread(app)));

    for handle in handles {
        handle.join().unwrap();
    }

    loop { thread::sleep(Duration::from_secs(1)); }

}

fn user_input_thread(app: Arc<Mutex<Application>>) {
    loop {
        let mut buf = String::new();
        let result = std::io::stdin().read_line(&mut buf);
        if let Ok(_) = result {
            let mut app = app.lock().unwrap();
            app.add_input(buf);
        } else if let Err(e) = result {
            println!("user_input_thread: {e}");
        }
    }
}

fn server_thread(app: Arc<Mutex<Application>>) {
    loop {

        let number = rand::thread_rng().gen_range(10000..99999);
        let random_string = format!("number... {number}");
        thread::sleep(Duration::from_millis(250));
        {
            let mut app = app.lock().unwrap();
            app.add_input(random_string);
        }
    }
}

fn drawing_thread(app: Arc<Mutex<Application>>) {
    loop {
        print!(".");
        thread::sleep(Duration::from_millis(10));
        let mut app = app.lock().unwrap();
        app.draw();
    }
}