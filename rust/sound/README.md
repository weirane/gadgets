# sound

Make some sound with Rust. Play it with

    cargo run
    ffplay -nodisp -f f32le -ar 48000 output.bin

Convert to mp3:

    ffmpeg -f f32le -ar 48000 -i output.bin out.mp3

[vid]: https://www.youtube.com/watch?v=FYTZkE5BZ-0
