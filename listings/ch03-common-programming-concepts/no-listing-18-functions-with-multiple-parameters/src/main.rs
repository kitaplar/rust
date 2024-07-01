fn main() {
    etiketli_ölçüyü_yazdır(5, 's');
}

fn etiketli_ölçüyü_yazdır(değer: i32, birim_etiketi: char) {
    println!("Ölçü: {değer}{birim_etiketi}");
}
