use crate::midi::constance::{byte, MIDI_MAGIC_HEAD_MTHD};
use crate::midi::track::Track;
use crate::midi::util::array::{u32_as_2l_byte_array, u32_as_4l_byte_array, vec_put_array};

#[derive(Debug)]
pub enum MidiFormat {
    MidiMultiple,
    MidiSingle,
    MidiPattern,
}

impl MidiFormat {
    pub fn value(&self) -> byte {
        match self {
            Self::MidiSingle => 0,
            Self::MidiMultiple => 1,
            Self::MidiPattern => 2
        }
    }
}

#[derive(Debug)]
pub struct MidiFile {
    format: MidiFormat,
    track_div: u32,
    pub track_chain: Vec<Track>,
    pub buffer: Vec<byte>,
}

impl MidiFile {
    pub fn new(format: MidiFormat, track_div: u32) -> Self {
        Self {
            format, track_div,
            track_chain: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn append_track(&mut self, track: Track) {
        self.track_chain.push(track);
    }

    pub fn generate(&mut self) {
        self.buffer.clear();
        let seclen = 6;
        let capacity = 4 + 4 + seclen;

        self.put_array(&MIDI_MAGIC_HEAD_MTHD);
        self.put_array(&u32_as_4l_byte_array(seclen));
        self.put_array(&u32_as_2l_byte_array(self.format.value() as u32));
        self.put_array(&u32_as_2l_byte_array(self.track_chain.len() as u32));
        self.put_array(&u32_as_2l_byte_array(self.track_div));

        for track in &*self.track_chain {
            track.write_head(&mut self.buffer);
            track.write_message(&mut self.buffer);
        }
    }

    pub fn file_size(&self) -> u32 {
        let sum: u32 = self.track_chain
            .iter()
            .map(| item | item.head_occupied as u32 + item.message_occupied())
            .sum();

        sum + 1 + 6 + 4 + 4
    }

    fn put_array(&mut self, array: &[byte]) {
        vec_put_array(self.buffer.as_mut(), array)
    }

    fn load_midi_bytes(bytes: &Vec<byte>) {
        let mut pointer: u32 = 0;


        loop {
            bytes[pointer];

            if pointer > &bytes.len() as u32 {
                break
            } else {
                pointer += 1;
            }
        }
    }
}



