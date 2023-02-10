use crate::notes::*;

#[derive(Copy, Clone)]
pub struct Piano{
    previous_sharp_note: Note,
    previous_regular_note: Note,
    volume: u8,
    is_sharp_playing: bool,
    is_regular_playing: bool,
    sharp_distance: u16,
    regular_distance: u16,
    note_to_play: (u16, u32),
}

impl Piano{
    pub fn new() -> Piano {
        Piano{
            previous_sharp_note: Note::None,
            previous_regular_note: Note::None,
            volume: 0,
            is_sharp_playing: false,
            is_regular_playing: false,
            sharp_distance: 800,
            regular_distance: 800,
            note_to_play: (0,0),
        }
    }
    fn set_note_to_play(&mut self, note:Note) {
        if note == Note::None{
            self.note_to_play = (50, 0);
        }
        else{
            self.note_to_play = (get_frequency(note) as u16, 4294967295);
        }
    }
    pub fn test_sharp_note(&mut self, note:Note) {
        if self.previous_sharp_note != note {
            self.previous_sharp_note = note;
            self.set_note_to_play(note);
        }
    }
    pub fn test_regular_note(&mut self, note:Note) {
        if self.previous_regular_note != note {
            self.previous_regular_note = note;
            self.set_note_to_play(note);
        }
    }
    pub fn get_sharp_distance(&self) -> u16{
        return self.sharp_distance;
    }
    pub fn get_regular_distance(&self) -> u16{
        return self.regular_distance;
    }
    pub fn get_volume(&self) -> u8{
        return self.volume;
    }
    pub fn get_note_to_play(&self) -> (u16, u32) {
        return self.note_to_play;
    }
    
    pub fn cb_linear_pot_position(&mut self, position:u8) {
        self.volume = position / 10;
    }
    pub fn cb_distance_sharps(&mut self, distance:u16) {
        self.sharp_distance = distance;
        if distance >= 640 || (distance >= 480 && distance < 520) || (distance >= 360 && distance < 400) || (distance >= 200 && distance < 240) || distance < 120 {
            if !self.is_regular_playing {
                self.test_sharp_note(Note::None);
            }
            self.previous_sharp_note = Note::None;
            self.is_sharp_playing = false;
            return;
        }
        else if distance >= 120 && distance < 160 {
            self.test_sharp_note(Note::C4s);
        }
        else if distance >= 160 && distance < 200 {
            self.test_sharp_note(Note::D4s);
        }
        else if distance >= 240 && distance < 280 {
            self.test_sharp_note(Note::F4s);
        }
        else if distance >= 280 && distance < 320 {
            self.test_sharp_note(Note::G4s);
        }
        else if distance >= 320 && distance < 360 {
            self.test_sharp_note(Note::A4s);
        }
        else if distance >= 400 && distance < 440 {
            self.test_sharp_note(Note::C5s);
        }
        else if distance >= 440 && distance < 480 {
            self.test_sharp_note(Note::D5s);
        }
        else if distance >= 520 && distance < 560 {
            self.test_sharp_note(Note::F5s);
        }
        else if distance >= 560 && distance < 600 {
            self.test_sharp_note(Note::G5s);
        }
        else if distance >= 600 && distance < 640 {
            self.test_sharp_note(Note::A5s);
        }
        self.is_sharp_playing = true;
        self.is_regular_playing = false;
    }
    pub fn cb_distance(&mut self, distance:u16) {
        self.regular_distance = distance;
    
        if self.is_sharp_playing {
            self.previous_regular_note = Note::None;
            return;
        }
        if distance >= 660 {
            self.is_regular_playing = false;
            self.test_regular_note(Note::None);
            self.previous_regular_note = Note::None;
            return;
        }
        else if distance >= 100 && distance < 140 {
            self.test_regular_note(Note::C4);
        }
        else if distance >= 140 && distance < 180 {
            self.test_regular_note(Note::D4);
        }
        else if distance >= 180 && distance < 220 {
            self.test_regular_note(Note::E4);
        }
        else if distance >= 220 && distance < 260 {
            self.test_regular_note(Note::F4);
        }
        else if distance >= 260 && distance < 300 {
            self.test_regular_note(Note::G4);
        }
        else if distance >= 300 && distance < 340 {
            self.test_regular_note(Note::A4);
        }
        else if distance >= 340 && distance < 380 {
            self.test_regular_note(Note::B4);
        }
        else if distance >= 380 && distance < 420 {
            self.test_regular_note(Note::C5);
        }
        else if distance >= 420 && distance < 460 {
            self.test_regular_note(Note::D5);
        }
        else if distance >= 460 && distance < 500 {
            self.test_regular_note(Note::E5);
        }
        else if distance >= 500 && distance < 540 {
            self.test_regular_note(Note::F5);
        }
        else if distance >= 540 && distance < 580 {
            self.test_regular_note(Note::G5);
        }
        else if distance >= 580 && distance < 620 {
            self.test_regular_note(Note::A5);
        }
        else if distance >= 620 && distance < 660 {
            self.test_regular_note(Note::B5);
        }
        self.is_regular_playing = true;
    }

}