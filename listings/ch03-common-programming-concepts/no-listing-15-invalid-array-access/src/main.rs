use std::io;

fn main() {
    let d = [1, 2, 3, 4, 5];

    println!("Lütfen bir dizi indeksi girin.");

    let mut indeks = String::new();

    io::stdin()
        .read_line(&mut indeks)
        .expect("Satırı okumak başarısız");

    let indeks: usize = indeks
        .trim()
        .parse()
        .expect("Girilen indeks bir sayı değil");

    let öğe = d[indeks];

    println!("{indeks} indeksindeki öğenin değeri: {öğe}");
}
