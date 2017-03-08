#![allow(dead_code, unused_variables)]

type Note = u8;

pub trait Effect {
    fn start(&mut self);
    fn stop(&mut self);
    fn is_running(&self) -> bool;
}

struct Patch<'a> {
    effects: Vec<(Note, Box<Effect>)>,
    running_effects: Vec<&'a mut Effect>,
}

impl<'a> Patch<'a> {
    pub fn new(effects: Vec<(Note, Box<Effect>)>) -> Patch {
        Patch { effects: effects, running_effects: vec![] }
    }

    // First attempt, but wrong approach?
    pub fn on_note(&mut self, pressed_key: Note) {
        let triggered_effects: Vec<_> = self.effects.iter_mut()
            .filter(|&&mut (note, _)| note == pressed_key)
            .collect();

        if triggered_effects.len() > 0 {
            // stop running effects
            for ref mut effect in &mut self.running_effects {
                effect.stop();
            }

            // start new effects
            for &mut(_, ref mut  effect) in triggered_effects {
                effect.start();
                //self.running_effects.push(effect);
            }
        }
    }

    // Ugly but working solution with indices
    pub fn on_note2(&mut self, pressed_key: Note) {
        let triggered_effect_indices: Vec<usize> = (0..self.effects.len()).filter(|&i| self.effects[i].0 == pressed_key).collect();
        if triggered_effect_indices.len() > 0 {
            for &mut (_, ref mut eff) in self.effects.iter_mut() {
                if eff.is_running() {
                    eff.stop();
                }
            }

            for triggered_index in triggered_effect_indices {
                self.effects.get_mut(triggered_index).unwrap().1.start();
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
