use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    id: usize,
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn send_message(&self, message: String) {
        self.thoughts.send(message).unwrap();
    }

    fn eat(&self) {
        // Solve the deadlock problem by acquiring the forks in a order 
        // that is odd philosophers pick up the left fork first and even 
        // philosophers pick up the right fork first.
        if self.id % 2 == 0 {
            let _right = self.right_fork.lock().unwrap();
            self.send_message(format!("{} has picked up the right fork", &self.name));
            let _left = self.left_fork.lock().unwrap();
            self.send_message(format!("{} has picked up the left fork", &self.name));
        } else {
            let _left = self.left_fork.lock().unwrap();
            self.send_message(format!("{} has picked up the left fork", &self.name));
            let _right = self.right_fork.lock().unwrap();
            self.send_message(format!("{} has picked up the right fork", &self.name));
        }
        self.thoughts
            .send(format!("{} is eating", &self.name))
            .unwrap();
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks
    let forks = vec![
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
        Arc::new(Mutex::new(Fork)),
    ];

    // Create philosophers
    let (tx, rx) = mpsc::channel();
    let mut philosophers = Vec::new();
    for (i, name) in PHILOSOPHERS.iter().enumerate() {
        let left_fork = forks[i].clone();
        let right_fork = forks[(i + 1) % forks.len()].clone();
        philosophers.push(Philosopher {
            id: i,
            name: format!("{}.{}", i, name),
            left_fork,
            right_fork,
            thoughts: tx.clone(),
        });
    }

    // Make them think and eat
    for philosopher in philosophers {
        let tx = philosopher.thoughts.clone();
        thread::spawn(move || loop {
            philosopher.think();
            philosopher.eat();
            tx.send(format!("{} is done eating!", &philosopher.name))
                .unwrap();
        });
    }

    // Output their thoughts
    for message in rx {
        println!("{}", message);
    }
}
