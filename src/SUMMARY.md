# Rust Programlama Dili

[Rust Programlama Dili](title-page.md)
[Önsöz](foreword.md)
[Giriş](ch00-00-introduction.md)

## Başlangıç

- [Başlangıç](ch01-00-getting-started.md)
    - [Kurulum](ch01-01-installation.md)
    - [Merhaba, Dünya!](ch01-02-hello-world.md)
    - [Merhaba, Cargo!](ch01-03-hello-cargo.md)

- [Bir Tahmin Oyunu Programlama](ch02-00-guessing-game-tutorial.md)

- [Yaygın Programlama Kavramları](ch03-00-common-programming-concepts.md)
    - [Değişkenler ve Değişebilirlik](ch03-01-variables-and-mutability.md)
    - [Veri Tipleri](ch03-02-data-types.md)
    - [Fonksiyonlar](ch03-03-how-functions-work.md)
    - [Yorumlar](ch03-04-comments.md)
    - [Kontrol Akışı](ch03-05-control-flow.md)

- [Sahipliği Anlama](ch04-00-understanding-ownership.md)
    - [Sahiplik Nedir?](ch04-01-what-is-ownership.md)
    - [Referanslar ve Ödünç Alma](ch04-02-references-and-borrowing.md)
    - [Dilim Tipi](ch04-03-slices.md)

- [İlişkili Verileri Yapılandırmak için Struct'ları Kullanmak](ch05-00-structs.md)
    - [Struct'ları Tanımlamak ve İlklendirmek](ch05-01-defining-structs.md)
    - [Struct'ları Kullanan Örnek bir Program](ch05-02-example-structs.md)
    - [Metod Sözdizimi](ch05-03-method-syntax.md)

- [Enum'lar ve Desen Eşleştirme](ch06-00-enums.md)
    - [Bir Enum Tanımlama](ch06-01-defining-an-enum.md)
    - [`match` Kontrol Akışı Yapısı](ch06-02-match.md)
    - [`if let` ile Özlü Kontrol Akışı](ch06-03-if-let.md)

## Temel Rust Okuryazarlığı

- [Büyüyen Projeleri Paketler, Crate'ler ve Modüller ile Yönetmek](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [Paketler ve Crate'ler](ch07-01-packages-and-crates.md)
    - [Kapsam ve Gizliliği Kontrol Etmek için Modülleri Tanımlama](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Modül Ağacındaki bir Öğeye Atıfta Bulunmak için Yollar](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [`use` Anahtar Kelimesi ile Yolları Kapsam İçine Getirmek](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Modülleri Farklı Dosyalara Ayırmak](ch07-05-separating-modules-into-different-files.md)

- [Yaygın Koleksiyonlar](ch08-00-common-collections.md)
    - [Vektörler ile Değer Listelerini Saklamak](ch08-01-vectors.md)
    - [String'ler ile UTF-8 ile Kodlanmış Metinleri Saklamak](ch08-02-strings.md)
    - [Hash Haritalarında İlişkili Değerler ile Anahtarları Saklamak](ch08-03-hash-maps.md)

- [Hata Ele Alma](ch09-00-error-handling.md)
    - [`panic!` ile Düzeltilemez Hatalar](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result` ile Düzeltilebilir Hatalar](ch09-02-recoverable-errors-with-result.md)
    - [`panic!`'leme ya da `panic!`'lememe](ch09-03-to-panic-or-not-to-panic.md)

- [Genel Tipler, Trait'ler ve Yaşam Süreleri](ch10-00-generics.md)
    - [Genel Veri Tipleri](ch10-01-syntax.md)
    - [Trait'ler: Paylaşılan Davranışı Tanımlama](ch10-02-traits.md)
    - [Referansları Yaşam Süreleri ile Doğrulama](ch10-03-lifetime-syntax.md)

- [Otomatik Testler Yazmak](ch11-00-testing.md)
    - [Testler Nasıl Yazılır?](ch11-01-writing-tests.md)
    - [Testlerin Nasıl Koşacağını Kontrol Etmek](ch11-02-running-tests.md)
    - [Test Organizasyonu](ch11-03-test-organization.md)

- [Bir G/Ç Projesi: Bir Komut Satırı Programı İnşa Etmek](ch12-00-an-io-project.md)
    - [Komut Satırı Argümanlarını Kabul Etmek](ch12-01-accepting-command-line-arguments.md)
    - [Bir Dosyayı Okumak](ch12-02-reading-a-file.md)
    - [Modülerlik ve Hata Ele Almayı İyileştirmek için Yeniden Düzenleme](ch12-03-improving-error-handling-and-modularity.md)
    - [Kütüphanenin İşlevselliğini Test Odaklı Geliştirme ile Geliştirmek](ch12-04-testing-the-librarys-functionality.md)
    - [Ortam Değişkenleri ile Çalışmak](ch12-05-working-with-environment-variables.md)
    - [Hata Mesajlarını Standart Çıktı Yerine Standart Hataya Yazmak](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Rust'ta Düşünmek

- [Fonksiyonel Dil Özellikleri: Yineleyiciler ve Kapamalar](ch13-00-functional-features.md)
    - [Kapamalar: Ortamını Yakalayan Anonim Fonksiyonlar](ch13-01-closures.md)
    - [Yineleyiciler ile bir Seri Öğeyi İşlemek](ch13-02-iterators.md)
    - [G/Ç Projemizi İyileştirmek](ch13-03-improving-our-io-project.md)
    - [Performansı Karşılaştırmak: Döngüler mi Yineleyiciler mi?](ch13-04-performance.md)

- [Cargo ve Crates.io hakkında Daha Fazlası](ch14-00-more-about-cargo.md)
    - [Yayın Profilleri ile İnşaları Özelleştirmek](ch14-01-release-profiles.md)
    - [Bir Crate'i Crates.io'ya yayınlamak](ch14-02-publishing-to-crates-io.md)
    - [Cargo Çalışma Alanları](ch14-03-cargo-workspaces.md)
    - [`cargo install` ile Crates.io'dan İkilileri Kurmak](ch14-04-installing-binaries.md)
    - [Cargo'yu Özel Komutlar ile Genişletmek](ch14-05-extending-cargo.md)

- [Akıllı Pointer'lar](ch15-00-smart-pointers.md)
    - [Yığındaki Veriye İşaret Etmek için `Box<T>`'yi Kullanmak](ch15-01-box.md)
    - [`Deref` Trait'i ile Akıllı Pointer'lara Sıradan Referanslar Gibi Davranmak](ch15-02-deref.md)
    - [`Drop` Trait'i ile Temizlik Zamanında Kod Çalıştırmak](ch15-03-drop.md)
    - [`Rc<T>`, Referans Sayılan Akıllı Pointer](ch15-04-rc.md)
    - [`RefCell<T>` ve Dahili Değişkenlik Deseni](ch15-05-interior-mutability.md)
    - [Referans Döngüleri Hafızayı Sızdırabilir](ch15-06-reference-cycles.md)

- [Korkusuz Eşzamanlılık](ch16-00-concurrency.md)
    - [Kodları Aynı Anda Çalıştırmak için Thread'leri Kullanmak](ch16-01-threads.md)
    - [Thread'ler Arasında Veri Transferi Yapmak için Mesaj Geçirmeyi Kullanmak](ch16-02-message-passing.md)
    - [Paylaşılan Durum Eşzamanlılığı](ch16-03-shared-state.md)
    - [`Sync` ve `Send` Trait'leri ile Genişletilebilir Eşzamanlılık](ch16-04-extensible-concurrency-sync-and-send.md)

- [Rust'ın Nesne Yönelimli Programlama Özellikleri](ch17-00-oop.md)
    - [Nesne Yönelimli Dillerin Karakteristikleri](ch17-01-what-is-oo.md)
    - [Farklı Tiplerin Değerlerine İzin Veren Trait Nesnelerini Kullanmak](ch17-02-trait-objects.md)
    - [Bir Nesne Yönelimli Tasarım Deseni Gerçeklemek](ch17-03-oo-design-patterns.md)

## İleri Konular

- [Desenler ve Eşleme](ch18-00-patterns.md)
    - [Desenlerin Kullanılabileceği Tüm Yerler](ch18-01-all-the-places-for-patterns.md)
    - [Çürütülebilirlik: Bir Desenin Eşleşmesinin Başarısız Olması ya da Olmaması](ch18-02-refutability.md)
    - [Desen Sözdizimi](ch18-03-pattern-syntax.md)

- [İleri Özellikler](ch19-00-advanced-features.md)
    - [Güvensiz Rust](ch19-01-unsafe-rust.md)
    - [İleri Trait'ler](ch19-03-advanced-traits.md)
    - [İleri Tipler](ch19-04-advanced-types.md)
    - [İleri Fonksiyon ve Kapamalar](ch19-05-advanced-functions-and-closures.md)
    - [Makrolar](ch19-06-macros.md)

- [Son Proje: Çok Thread'li bir Web Sunucusu İnşa Etmek](ch20-00-final-project-a-web-server.md)
    - [Tek Thread'li bir Web Sunucusu İnşa Etmek](ch20-01-single-threaded.md)
    - [Tek Thread'li Sunucumuzu Çok Thread'li bir Sunucuya Çevirmek](ch20-02-multithreaded.md)
    - [Nazik Kapatma ve Temizlik](ch20-03-graceful-shutdown-and-cleanup.md)

- [Ekler](appendix-00.md)
    - [A - Anahtar Kelimeler](appendix-01-keywords.md)
    - [B - İşleçler ve Semboller](appendix-02-operators.md)
    - [C - Türetilebilir Trait'ler](appendix-03-derivable-traits.md)
    - [D - Kullanışlı Geliştirme Araçları](appendix-04-useful-development-tools.md)
    - [E - Baskılar](appendix-05-editions.md)
    - [F - Kitabın Çevrilmesi](appendix-06-translation.md)
    - [G - Rust Nasıl Yapılıyor ve “Gecelik Rust”](appendix-07-nightly-rust.md)
