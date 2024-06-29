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

        io::stdin()
            .read_line(&mut tahmin)
            .expect("Satırı okumak başarısız");

        let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı girin!");

        println!("Şunu tahmin ettiniz: {tahmin}");

        // ANCHOR: here
        // --snip--

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
// ANCHOR_END: here
