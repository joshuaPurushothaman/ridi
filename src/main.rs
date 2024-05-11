use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput};

    

use std::path::Path;

use sofiza::Instrument;

fn main() {
    let filepath = Path::new("SalamanderGrandPiano-SFZ+FLAC-V3+20200602/mod.sfz");

    let instrument = Instrument::from_file(filepath).expect(
        "Couldn't parse the instrument. \
        Please run the example from the examples/ directory",
    );

    // for region in &instrument.regions {
    //     println!("{:?}", region);
    // }

    // println!("{:#?}", instrument);

    // println!("groups: {}\nregions: {}", instrument.groups(), instrument.regions());
}
