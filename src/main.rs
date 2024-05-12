use std::{collections::BTreeMap, path::Path};

use sofiza::{Instrument, Opcode};

fn main() {
    let filepath = Path::new("SalamanderGrandPiano-SFZ+FLAC-V3+20200602/mod.sfz");

    let instrument = Instrument::from_file(filepath).expect("Couldn't parse the instrument.");

    // println!("{:#?}", instrument);
    // for group in instrument.groups {
    //     println!("{:?}", group.label);
    //     println!("{:?}", group.opcodes.);
    // }

    // Store a sorted map of MIDI Note #s to file paths
    let mut note_map = BTreeMap::new();

    for region in &instrument.regions {
        // println!("{:?}", region);

        let sample = match region.opcodes.get("sample") {
            Some(Opcode::sample(s)) => s,
            other => panic!("somehow found a goof {:?}", other),
        };

        let lokey = match region.opcodes.get("lokey") {
            Some(Opcode::lokey(l)) => l,
            other => panic!("somehow found a goof {:?}", other),
        }
        .to_owned();

        let hikey = match region.opcodes.get("hikey") {
            Some(Opcode::hikey(h)) => h,
            other => panic!("somehow found a goof {:?}", other),
        }
        .to_owned();

        let difference = hikey - lokey;

        let expected_key_num = match difference {
            1 => hikey,
            2 => lokey + 1,
            _ => panic!("Hmm, this one's odd lol"),
        };

        // println!("\tdifference: {:?}", difference);
        // println!("\texpected_key_num: {:?}", expected_key_num);

        note_map.entry(expected_key_num).or_insert(sample);
    }

    println!("NOTE MAP:");
    for (key, filename) in note_map.iter() {
        println!("{:?}: {:?}", key, filename)
    }
}
