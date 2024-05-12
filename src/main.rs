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

    // <region> sample=samples/C4v1.flac lokey=59 hikey=61 lovel=1 hivel=26 tune=-6
    // <region> sample=samples/C4v2.flac lokey=59 hikey=61 lovel=27 hivel=34 tune=-6
    // <region> sample=samples/C4v3.flac lokey=59 hikey=61 lovel=35 hivel=36 tune=-6
    // <region> sample=samples/C4v4.flac lokey=59 hikey=61 lovel=37 hivel=43 tune=-6
    // <region> sample=samples/C4v5.flac lokey=59 hikey=61 lovel=44 hivel=46 tune=-6
    // <region> sample=samples/C4v6.flac lokey=59 hikey=61 lovel=47 hivel=50 tune=-6
    // <region> sample=samples/C4v7.flac lokey=59 hikey=61 lovel=51 hivel=56 tune=-6
    // <region> sample=samples/C4v8.flac lokey=59 hikey=61 lovel=57 hivel=64 tune=-6
    // <region> sample=samples/C4v9.flac lokey=59 hikey=61 lovel=65 hivel=72 tune=-6
    // <region> sample=samples/C4v10.flac lokey=59 hikey=61 lovel=73 hivel=80 tune=-6
    // <region> sample=samples/C4v11.flac lokey=59 hikey=61 lovel=81 hivel=88 tune=-6
    // <region> sample=samples/C4v12.flac lokey=59 hikey=61 lovel=89 hivel=96 tune=-6
    // <region> sample=samples/C4v13.flac lokey=59 hikey=61 lovel=97 hivel=104 tune=-6
    // <region> sample=samples/C4v14.flac lokey=59 hikey=61 lovel=105 hivel=112 tune=-6
    // <region> sample=samples/C4v15.flac lokey=59 hikey=61 lovel=113 hivel=120 tune=-6
    // <region> sample=samples/C4v16.flac lokey=59 hikey=61 lovel=121 tune=-6

    // Store a sorted map of MIDI Note #s to file paths
    struct Note {
        key: u8,
        vel: u8,
    }

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
