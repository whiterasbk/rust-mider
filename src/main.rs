use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use midi::file::{
    MidiFile, MidiFormat
};
use crate::midi::track::{TrackBuilder};

mod midi;

fn main() {

    let mut file = MidiFile::new(MidiFormat::MidiMultiple, 960);


    file.append_track(TrackBuilder::new()
        .tempo(120)
        .end()
        .build());

    file.append_track(
        TrackBuilder::new()
            .note_on(48, 920, 100, 0)
            .note_off(48, 0, 100, 0)
            .end()
            .build()
    );

    file.generate();

    let mut f = File::create("test.mid").expect("err!");

    f.write(file.buffer.deref()).expect("TODO: panic message");

    println!("{:?}", file)
}
