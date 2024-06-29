use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_sayı = rand::thread_rng().gen_range(1..=100);

    println!("Gizli sayı: {gizli_sayı}");

    loop {
        println!("Lütfen tahmininizi girin.");

        let mut tahmin = String::new();

        // ANCHOR: here
        // --snip--

        io::stdin()
            .read_line(&mut tahmin)
            .expect("Satırı okumak başarısız");

        // ANCHOR: ch19
        let tahmin: u32 = match tahmin.trim().parse() {
            Ok(sayı) => sayı,
            Err(_) => continue,
        };
        // ANCHOR_END: ch19

        println!("Şunu tahmin ettiniz: {tahmin}");

        // --snip--
        // ANCHOR_END: here

        match tahmin.cmp(&gizli_sayı) {
            Ordering::Less => println!("Çok küçük!"),
            Ordering::Greater => println!("Çok büyük!"),
            Ordering::Equal => {
                println!("Kazandın!");
                break;
            }
        }
    }
}
