// ANCHOR: here
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    // ANCHOR_END: here
    println!("Sayıyı tahmin et!");

    let gizli_sayı = rand::thread_rng().gen_range(1..=100);

    println!("Gizli sayı: {gizli_sayı}");

    println!("Lütfen tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin()
        .read_line(&mut tahmin)
        .expect("Satırı okumak başarısız");
    // ANCHOR: here

    println!("Şunu tahmin ettiniz: {tahmin}");

    match tahmin.cmp(&gizli_sayı) {
        Ordering::Less => println!("Çok küçük!"),
        Ordering::Greater => println!("Çok büyük!"),
        Ordering::Equal => println!("Kazandın!"),
    }
}
// ANCHOR_END: here
