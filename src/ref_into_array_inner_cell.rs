#![allow(dead_code, unused_variables)]

use std::cell::Cell;


type Note = u8;

pub trait Effect {
    fn start(&self);
    fn stop(&self);
    fn is_running(&self) -> bool;
}

struct DummyEffect {
    immutable: String,
    running: Cell<bool>
}

impl Effect for DummyEffect {
    fn start(&self) {
        self.running.set(true);
    }

    fn stop(&self) {
        self.running.set(false);
    }

    fn is_running(&self) -> bool {
        self.running.get()
    }
}

struct Patch {
    effects: Vec<(Note, Box<Effect>)>,
}

impl Patch {
    pub fn new(effects: Vec<(Note, Box<Effect>)>) -> Patch {
        Patch { effects: effects }
    }

    pub fn on_note(&mut self, pressed_key: Note) {
        let triggered_effects: Vec<_> = self.effects.iter()
            .filter(|&& (note, _)| note == pressed_key)
            .collect();

        if triggered_effects.len() > 0 {
            // stop running effects
            for & (_, ref eff) in self.effects.iter() {
                if eff.is_running() {
                    eff.stop();
                }
            }

            // start new effects
            for & (_, ref effect) in triggered_effects {
                effect.start();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
