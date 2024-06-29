// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Sayıyı tahmin et!");

    // ANCHOR: ch07-04
    let gizli_sayı = rand::thread_rng().gen_range(1..=100);
    // ANCHOR_END: ch07-04

    println!("Gizli sayı: {gizli_sayı}");

    println!("Lütfen tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin()
        .read_line(&mut tahmin)
        .expect("Satırı okumak başarısız");

    println!("Şunu tahmin ettiniz: {tahmin}");
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
