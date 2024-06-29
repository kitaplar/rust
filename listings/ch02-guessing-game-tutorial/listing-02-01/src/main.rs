// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Sayıyı tahmin et!");

    println!("Lütfen tahmininizi girin.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut tahmin = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut tahmin)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Satırı okumak başarısız");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Şunu tahmin ettiniz: {}", tahmin);
    // ANCHOR_END: print_guess
}
// ANCHOR: all
