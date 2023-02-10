#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Note{
    None,

    C4s,
    D4s,
    F4s,
    G4s,
    A4s,
    C5s,
    D5s,
    F5s,
    G5s,
    A5s,

    C4,
    D4,
    E4,
    F4,
    G4,
    A4,
    B4,
    C5,
    D5,
    E5,
    F5,
    G5,
    A5,
    B5,
}
pub fn get_frequency(note: Note) -> f64{
    match note{
        Note::None => 0.0,

        Note::C4s => 277.18,
        Note::D4s => 311.13,
        Note::F4s => 369.99,
        Note::G4s => 415.30,
        Note::A4s => 466.16,
        Note::C5s => 554.37,
        Note::D5s => 622.25,
        Note::F5s => 739.99,
        Note::G5s => 830.61,
        Note::A5s => 932.33,

        Note::C4 => 261.63,
        Note::D4 => 393.66,
        Note::E4 => 329.63,
        Note::F4 => 349.23,
        Note::G4 => 392.00,
        Note::A4 => 440.00,
        Note::B4 => 493.88,
        Note::C5 => 523.25,
        Note::D5 => 587.33,
        Note::E5 => 659.25,
        Note::F5 => 698.46,
        Note::G5 => 783.99,
        Note::A5 => 880.00,
        Note::B5 => 987.77,
    }
}
