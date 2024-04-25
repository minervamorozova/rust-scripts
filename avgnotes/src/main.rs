use std::io;

const SIZE: usize = 5;

fn avg_calc(notes: &[f32]) -> f32 {
    let sum: f32 = notes.iter().sum();
    sum / notes.len() as f32
}

fn main() {
    let mut names = Vec::new();
    let mut all_notes = Vec::new();

    for _ in 0..SIZE {
        println!("Type a Student's name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        names.push(name.trim().to_string());

        let mut notes = Vec::new();
        for x in 0..3 {
            println!("Type Note {}: ", x + 1);
            let mut note_str = String::new();
            io::stdin().read_line(&mut note_str).unwrap();
            let note: f32 = note_str.trim().parse().expect("Please type a number!");
            notes.push(note);
        }
        all_notes.push(notes.clone());
        println!("The average note is {}", avg_calc(&notes));
    }
}