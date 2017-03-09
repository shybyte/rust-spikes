#![allow(dead_code, unused_variables)]

type Note = u8;

pub trait Effect {
    fn start(&mut self);
    fn stop(&mut self);
    fn is_running(&self) -> bool;
}

struct Patch {
    effects: Vec<(Note, Box<Effect>)>,
}

impl Patch {
    pub fn new(effects: Vec<(Note, Box<Effect>)>) -> Patch {
        Patch { effects: effects }
    }

    pub fn on_note(&mut self, pressed_key: Note) {
        let (mut triggered_effects, mut remaining_effects) = self.effects.iter_mut()
            .partition::<Vec<_>,_>(|&&mut (note, _)| note == pressed_key);

        if triggered_effects.len() > 0 {
            for &mut &mut (_, ref mut eff) in remaining_effects.iter_mut().chain(triggered_effects.iter_mut()) {
                if eff.is_running() {
                    eff.stop();
                }
            }

            // start new effects
            for &mut (_, ref mut effect) in triggered_effects {
                effect.start();
                //self.running_effects.push(effect);
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
