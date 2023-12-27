use std::collections::HashMap;
use std::io;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

#[allow(dead_code)]

fn main() {
    let morse_map = HashMap::from([
        // Letters
        ('A', ".-"),('B', "-..."),('C', "-.-."),('D', "-.."),('E', "."),('F', "..-."),('G', "--."),
        ('H', "...."),('I', ".."),('J', ".---"),('K', "-.-"),('L', ".-.."),('M', "--"),('N', "-."),
        ('O', "---"),('P', ".--."),('Q', "--.-"),('R', ".-."),('S', "..."),('T', "-"),('U', "..-"),
        ('V', "...-"),('W', ".--"),('X', "-..-"),('Y', "-.--"),('Z', "--.."),
        ('a', ".-"),('b', "-..."),('c', "-.-."),('d', "-.."),('e', "."),('f', "..-."),('g', "--."),
        ('h', "...."),('i', ".."),('j', ".---"),('k', "-.-"),('l', ".-.."),('m', "--"),('n', "-."),
        ('o', "---"),('p', ".--."),('q', "--.-"),('r', ".-."),('s', "..."),('t', "-"),('u', "..-"),
        ('v', "...-"),('w', ".--"),('x', "-..-"),('y', "-.--"),('z', "--.."),
        // Numbers
        ('0', "-----"),('1', ".----"),('2', "..---"),('3', "...--"),('4', "....-"),('5', "....."),
        ('6', "-...."),('7', "--..."),('8', "---.."),('9', "----."),
        // Characters
        ('.', ".-.-.-"),(',', "--..--"),('?', "..--.."),('\'', ".----."),('!', "-.-.--"),
        ('/', "-..-."),('(', "-.--."),(')', "-.--.-"),('&', ".-..."),(';', "-.-.-."),
        (':', "---..."),('=', "-...-"),('+', ".-.-."),('-', "-....-"),('_', "..--.-"),
        ('"', ".-..-."),('$', "...-..-"),('@', ".--.-."),(' ', "  ")
    ]);

    // Receive User Input
    println!("Enter text to translate: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read guess!");

    // Translate message, print, and play audio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    for character in input.trim().chars() {
        if let Some(value) = morse_map.get(&character) {
            print!("{} ", value);
            for sound in value.chars() {
                match sound {
                    '.' => {
                        let e_file = BufReader::new(File::open("audio/E_morse_code.ogg").unwrap());
                        let e_sound = Decoder::new(e_file).unwrap();
                        stream_handle.play_raw(e_sound.convert_samples()).expect("Play E failed!");
                        thread::sleep(Duration::from_millis(40))}, // Wait for audio file to finish playing
                    '-' => {
                        let t_file = BufReader::new(File::open("audio/T_morse_code.mp3").unwrap());
                        let t_sound = Decoder::new(t_file).unwrap();
                        stream_handle.play_raw(t_sound.convert_samples()).expect("Play T failed!");
                        thread::sleep(Duration::from_millis(90))}, // Wait for audio file to finish playing
                    ' ' => thread::sleep(Duration::from_millis(150)),
                    _ => {}
                }
                thread::sleep(Duration::from_millis(250))
            }
        } else {
            print!("Key '{}' not found in the HashMap", character);
        }
    }
    print!("\n");
}