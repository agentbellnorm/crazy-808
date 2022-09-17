use std::{thread, time, sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}}};
use crate::sound::Sound;

const NUMBER_OF_CHANNELS: usize = 3;

pub struct MachineState {
    pub variation_a: [[u8; 16]; NUMBER_OF_CHANNELS],
    pub variation_b: [[u8; 16]; NUMBER_OF_CHANNELS],
    pub playing: bool
}

impl Default for MachineState {
    fn default() -> Self {
        MachineState {
            variation_a: [
                [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                [1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            ],
            variation_b: [
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
                [1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            ],
            playing: false,
        }
    }
}

pub struct Engine {
    state: Arc<Mutex<MachineState>>,
    beat_callback: Option<Arc<Mutex<dyn FnMut() + 'static + Sync + Send>>>,
    sender: Sender<usize>,
    receiver: Arc<Mutex<Receiver<usize>>>,
}

impl Engine {
    pub fn new(state: Arc<Mutex<MachineState>>) -> Self {
        let (sender, receiver) = mpsc::channel();
        Engine {
            state,
            beat_callback: None,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn onBeat(mut self, callback: impl FnMut() + 'static + Sync + Send) {
        self.beat_callback = Some(Arc::new(Mutex::new(callback)));
    }

    pub fn run(&self) {
        let state_arc = self.state.clone();

        let sender_2 = self.sender.clone();

        thread::spawn(move || {
            let sound = Sound::new();
            let mut bar = 0;
            loop {
                let state = state_arc.lock().unwrap();

                for channel in 0..state.variation_a.len() {
                    if state.variation_a[channel][bar] == 1 {
                        sound.play(channel);
                    }
                }
                println!("sending!!");
                match sender_2.send(bar) {
                    Err(m) => println!("Åh nej!: {}", m),
                    _ => (),
                }

                bar += 1;
                if bar == 16 {
                    bar = 0;
                }


                thread::sleep(time::Duration::from_millis(200));
            }
        });

        println!("starting receiver thread!");
        if let Some(c) = &self.beat_callback {
            let cb = c.clone();
            let receiver2 = self.receiver.clone();
            thread::spawn(move || {
                let rcv = receiver2.lock().unwrap();
                for _ in rcv.try_recv() {
                    println!("receiving on receiver!");
                    cb.lock().unwrap()();  
                }
            });
        }
    }
}

