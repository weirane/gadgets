mod note;

use std::f32::consts::PI;
use std::fs::File;
use std::io::{self, prelude::*};

const SAMPLE_RATE: f32 = 48000.;
static mut BPM: f32 = 80.;

fn wave(note: f32, beats: f32) -> Vec<u8> {
    let omega = note * 2. * PI / SAMPLE_RATE;
    // not using multi-threading so the unsafe here is safe
    let one_beat = 60. / unsafe { BPM } * SAMPLE_RATE;
    let duration = beats * one_beat;
    // duration for attack and release
    let att_rel = duration.min(one_beat) / 5.;

    let mut notes = Vec::new();
    for n in 0..duration as i32 {
        let n = n as f32;
        // sine wave with attack and release
        let y = (n * omega).sin() * 1f32.min(n / att_rel) * 1f32.min((duration - n) / att_rel);
        notes.extend(y.to_le_bytes().iter().cloned());
    }
    notes
}

macro_rules! write_note {
    ($file:ident, $($n:expr, $b:expr;)*) => {
        $($file.write_all(&wave($n, $b))?;)*
    }
}

fn main() -> io::Result<()> {
    use crate::note::*;

    let mut f = File::create("output.bin")?;
    write_note! {f,
        A3, 0.3; F4s, 0.5; REST, 0.5; F4s, 2.; E4, 0.25; D4, 0.25; F4s, 0.25; A4, 0.25;

        E4, 1.5; F4s, 0.5; E4, 1.; F4s, 1.;

        B3, 0.1; G4, 0.5; REST, 0.5; G4, 1.75; A4, 0.25; B4, 0.25; C5s, 0.25; D5, 0.25; E5, 0.25;

        A4, 1.5; B4, 1./6.; D5, 1./6.; B4, 1./6.; A4, 0.5; B4, 0.5; C5s, 0.5; D5, 0.5;

        E5, 0.5; G5, 0.5; A4, 1.5; B4, 0.5; C5s, 0.5; D5, 0.5;

        E5, 0.5; G5, 0.5; A4, 4./3.; G4s, 1./3.; A4, 1./3.; A4s, 1./3.; A4, 1./3.; A4s, 1./3.;

        B4, 1./3.; D5, 1./3.; F5s, 1./3.; G4s, 1.5; D4s, 0.25; E4, 0.25; F4s, 0.25;
        G4s, 0.25; A4, 0.25; B4, 0.25; E5, 0.25;

        E5, 1.25; F5, 0.25; F5s, 0.25; G5s, 0.25; A5, 0.5; B5, 0.375; B5, 0.125; C5s, 1.5;

        E5, 0.25; REST, 0.125; E5, 0.125; G4, 1.5; B4, 0.25; REST, 0.125; B4, 0.125;
        C4s, 0.5; G4, 0.25; REST, 0.125; G4, 0.125;

        B3, 0.5; E4, 0.25; REST, 0.125; E4, 0.125; B3f, 0.5; C4s, 0.25; REST, 0.125; C4s, 0.125;
        A3, 0.5; B3f, 0.25; REST, 0.125; B3f, 0.125; G3s, 0.5; B3f, 0.25; REST, 0.125; B3f, 0.125;

        A3, 0.5; B3f, 0.25; REST, 0.125; B3f, 0.125; G3s, 0.5; B3f, 0.25; REST, 0.125; B3f, 0.125;
        A3, 0.5; B3f, 0.25; REST, 0.125; B3f, 0.125; G3s, 0.5; A3, 0.25; REST, 0.125; B3f, 0.125;

        G3s, 0.5; A3, 0.25; REST, 0.125; B3f, 0.125; G3s, 0.5; A3, 0.25; REST, 0.125; B3f, 0.125;
        G3s, 1./6.; A3, 1./6.; B3f, 1./6.; G3s, 1./6.; A3, 1./6.; B3f, 1./6.;
        G3s, 1./6.; A3, 1./6.; B3f, 1./6.; G3s, 1./6.; A3, 1./6.; B3f, 1./6.;

        G3s, 1./6.; A3, 1./6.; B3f, 1./6.; B3, 1./6.; C4, 1./6.; C4s, 1./6.;
        D4, 1./6.; D4s, 1./6.; E4, 1./6.; F4, 1./6.; F4s, 1./6.; G4, 1./6.;
        G4s, 1./6.; A4, 1./6.; B4f, 1./6.; B4, 1./6.; C5, 1./6.; C5s, 1./6.;
        D5, 0.25; D5s, 0.25; E5, 0.25; F5, 0.25;

        F5s, 0.25; REST, 0.25; A4, 0.15; F5s, 2.;
    }
    Ok(())
}
