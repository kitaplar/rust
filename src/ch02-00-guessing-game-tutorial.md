# Bir Tahmin Oyunu Programlama

Hadi uygulamalı bir proje ile Rust'a başlayalım! Bu bölüm, birkaç yaygın Rust
kavramını, bunları gerçek bir programda nasıl kullanacağınızı göstererek
tanıtır. `let`, `match`, metodlar, ilişkili fonksiyonlar, harici crate'ler ve
daha fazlası hakkında öğreneceksiniz! Sonraki bölümlerde, bu fikirleri daha
ayrıntılı bir şekilde ele alacağız. Bu bölümde sadece temelleri pratik
edeceksiniz.

Klasik bir başlangıç programlama problemini gerçekleyeceğiz: bir tahmin oyunu.
Şöyle çalışıyor: program 1 ve 100 arasında rasgele bir tamsayı üretecek. Daha
sonra oyuncunun bir tahmin girmesini isteyecek. Bir tahmin girildikten sonra
program, tahminin çok küçük ya da büyük olduğunu söyleyecek. Eğer tahmin
doğruysa, program bir tebrik mesajı yazdıracak ve çıkacak.

## Yeni bir Proje Ayarlamak

Yeni bir proje ayarlamak için 1. bölümde oluşturduğunuz *projeler* dizinine
gidin ve Cargo kullanarak yeni bir proje yapın, şunun gibi:

```console
$ cargo new tahmin_oyunu
$ cd tahmin_oyunu
```

İlk komut, yani `cargo new`, projenin adını (`tahmin_oyunu`) ilk argüman olarak
alır. İkinci komut, yeni projenin dizinine gidilmesini sağlar.

Üretilen *Cargo.toml* dosyasına bakın:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Bölüm 1'de gördüğünüz gibi `cargo new` bir “Hello, world!” programı üretir.
*src/main.rs* dosyasına bakın:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Şimdi hadi bu “Hello, world!” programını `cargo run` komutunu kullanarak tek
adımda derleyip koşalım:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

`run` komutu, bir proje üzerinde hızlıca yinelemek istediğinizde kullanışlıdır.
Bu oyunda da bunu yapacağız çünkü bir sonrakine geçmeden önce her adımı hızlıca
test etmemiz gerekir.

*src/main.rs* dosyasını yeniden açın. Tüm kodunuzu bu dosyaya yazacaksınız.

## Bir Tahmini İşleme

Tahmin oyununun ilk kısmı kullanıcı girdisini isteyecek, bunu işleyecek ve
bunun beklenen biçimde olup olmadığını kontrol edecek. Başlamak için oyuncunun
bir tahmin girmesine izin vereceğiz. *src/main.rs*'e Listing 2-1'deki kodu
girin.

<Listing number="2-1" file-name="src/main.rs" caption="Kullanıcıdan bir tahmin alan ve bunu yazdıran kod">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing> 

Bu kod birçok bilgi içermektedir, hadi satır satır inceleyelim. Kullanıcı
girdisi almak ve sonra sonucu çıktı olarak yazdırmak için `io` girdi/çıktı
kütüphanesini kapsam içine almamız lazım. `io` kütüphanesi `std` olarak bilinen
standart kütüphaneden gelir:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Varsayılan olarak Rust, her programın kapsamına getirdiği, standart kütüphanede
tanımlı bir küme öğeye sahiptir. Bu kümeye *başlangıç* denir. Bunun içindeki
her şeyi [standart kütüphane belgelendirmesinde][prelude] görebilirsiniz.

Eğer kullanmak istediğiniz bir tip başlangıçta değilse, bir `use` ifadesi ile
harici olarak bu tipi kapsama getirmeniz gerekir. `std::io` kütüphanesini
kullanmak size birçok kullanışlı özellik sağlar, kullanıcı girişini kabul etme
yeteneği dahil.

Bölüm 1'de gördüğünüz gibi, `main` fonksiyonu programa giriş noktasıdır:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` sözdizimi yeni bir fonksiyon tanımlar; parantezler, `()`, hiçbir parametre
olmadığına işarettir; ve süslü parantez, `{`, fonksiyonun gövdesini başlatır.

Ayrıca 1. bölümde öğrendiğiniz gibi, `println!`, ekrana bir string yazdıran
bir makrodur:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Bu kod, oyunun ne olduğunu belirten ve kullanıcıdan bir girdi isteyen bir istem
yazdırır.

### Değerleri Değişkenler ile Saklamak

Şimdi kullanıcı girdisini saklamak için bir *değişken* oluşturacağız, şu
şekilde:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Şimdi program ilginç olmaya başladı! Bu küçük satırda olan bir sürü şey var.
Değişkeni oluşturmak için `let` ifadesini kullanırız. Bir diğer örnek:

```rust,ignore
let elmalar = 5;
```

Bu satır `elmalar` isminde yeni bir değişken oluşturur ve bunu 5 değerine
bağlar. Rust'ta değişkenler varsayılan olarak değişemezdir, yani bir değişkene
bir defa değer verdik mi, değer bir daha değişmez. Bu kavramı ayrıntılı olarak
3. bölümün [“Değişkenler ve Değişebilirlik”][variables-and-mutability] kısmında
tartışacağız. Değişkeni değişebilir yapmak için değişken adından önce `mut`
ekleriz:

```rust,ignore
let elmalar = 5; // değişemez
let mut muzlar = 5; // değişebilir
```

> Not: `//` sözdizimi, satırın sonuna kadar devam edecek olan bir yorum
> başlatır. Rust, yorumlardaki her şeyi boşverir. Yorumları ayrıntılı bir
> şekilde [3. bölümde][comments] tartışacağız.

Tahmin oyunu programına geri dönersek, şimdi `let mut tahmin`'in `tahmin`
isminde, değişebilir bir değişken tanıttığını biliyorsunuz. Eşittir işareti
(`=`) Rust'a bu değişkene bir şey bağlamak istediğimizi söyler. Eşittir
işaretinin sağında `tahmin`'in bağlandığı değer vardır, bu değer ise
`String::new` fonksiyonunu çağırmanın sonucudur. Bu fonksiyon bir `String`
örneği döndürür. [`String`][string], standart kütüphane tarafından sağlanan,
büyüyebilir, UTF-8 kodlanmış bir string tipidir.

`::new`'deki `::` sözdizimi, `new`'in `String` tipinin bir ilişkili fonksiyonu
olduğuna işaret eder. *İlişkili fonksiyon*, bir tipte, burada `String`,
gerçeklenen bir fonksiyondur. Bu `new` fonksiyonu yeni, boş bir string
oluşturur. Birçok tipte `new` fonksiyonuna denk geleceksiniz çünkü bu, bir
türde yeni bir değer yapmak için yaygın kullanılan bir fonksiyon adıdır.

Tam olarak `let mut tahmin = String::new();` satırı değişebilir bir değişken
oluşturdu ve bu değişken şu anda `String`'in yeni, boş bir örneğine bağlı. Vay
be!

### Kullanıcı Girdisini Getirmek

Programın ilk satırında `use std::io` ile standart kütüphaneden girdi/çıktı
işlevselliği dahil ettiğimizi hatırlayın. Şimdi `io` modülünden `stdin`
fonksiyonunu çağıracağız, bu fonksiyon ise kullanıcı girişini ele almamızı
sağlayacak:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Programın başında `io` kütüphanesini `use std::io;` ile içe aktarmamış olsak
bile, bu fonksiyon çağrısını `std::io::stdin` olarak yazarak yine de bu
fonksiyonu kullanabilirdik. `stdin` fonksiyonu, bir [`std::io::Stdin`][iostdin]
örneği döndürür. Bu tip, terminalinizdeki standart girdi için bir ele alışı
ifade eder.

Sonra `.read_line(&mut tahmin)`, kullanıcıdan girdi almak için standart girdi
ele alışı üzerinde [`read_line`][read_line] metodunu çağırır. Ayrıca
`&mut tahmin`'i `read_line`'a argüman olarak geçiriyoruz, böylece kullanıcı
girdisini hangi string içinde saklayacağımızı belirtmiş oluyoruz.
`read_line`'ın tam olarak işi, kullanıcının standart girdiye yazdığı her şeyi
almak ve bunu bir string'e eklemektir (string'in içeriğinin üzerine yazmadan),
bu sebeple bu string'i bir argüman olarak geçiriyoruz. String argümanı
değişebilir olmalıdır, böylece metod, string'in içeriğini değiştirebilir.

`&`, bu argümanın bir *referans* olduğunu belirtir. Referanslar, kodunuzun
birden çok kısmının bir veriye erişebilmesini sağlar, bu veriyi hafızaya birçok
kez kopyalamak zorunda kalmadan. Referanslar karmaşık bir özelliktir ve Rust'ın
ana avantajlarından biri, referansları kullanmanın ne kadar güvenli ve kolay
olduğudur. Bu programı bitirmek için bu ayrıntıları bilmenize gerek yok.
Şimdilik sadece referansların da, değişkenler gibi, varsayılan olarak değişemez
olduklarını bilmeniz yeterli. Bu sebeple bunu değişebilir yapmak için `&tahmin`
yerine `&mut tahmin` yazmanız gerekiyor. (4. bölüm, referansları ayrıntılı bir
şekilde işleyecek.)

<!-- Old heading. Do not remove or links may break. -->
<a id="handling-potential-failure-with-the-result-type"></a>

### `Result` ile Potansiyel Başarısızlığı Ele Alma

Hala bu kod satırı üzerinde çalışıyoruz. Şimdi bunun üçüncü satırını
tartışıyoruz ama bunun tek bir mantıksal kod satırının bir parçası olduğuna
dikkat edin. Bir sonraki parça şu metod:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Bu kodu şu şekilde de yazabilirdik:

```rust,ignore
io::stdin().read_line(&mut tahmin).expect("Satırı okumak başarısız");
```

Fakat uzun bir satırı okuması zordur, bu sebeple en iyisi onu bölmek.
Genellikle, bir metodu `.metod_adı()` sözdizimi ile çağırdığınızda, uzun
satırları bölmeye yardımcı olması için bir yenisatır ve diğer boşlukları
eklemek akıllıcadır. Şimdi bu satırın ne yaptığını tartışalım.

Daha önce bahsettiğimiz gibi, `read_line`, kullanıcı her ne girdiyse bunu, ona
geçirdiğimiz string'e koyar ama ayrıca bir `Result` değeri döner.
[`Result`][result], genellikle *enum* olarak isimlendirilen bir
[*numaralandırma*][enums]dır. enum, birden çok muhtemel durumdan birinde
bulunan bir tiptir. Bu muhtemel durumların her birine *varyant* diyeceğiz.

[6. bölüm][enums], enum'ları ayrıntılı bir şekilde kapsayacaktır. Bu `Result`
tiplerinin amacı, hata ele alma bilgisini kodlamaktır.

`Result`'ın varyantları `Ok` ve `Err`'dir. `Ok` varyantı, işlemin başarılı
olduğunu belirtir, içinde de başarılı bir şekilde üretilen değeri tutar. `Err`
varyantı, işlemin başarısız olduğu anlamına gelir ve içinde işlemin nasıl ya da
neden başarısız olduğuyla ilgili bilgi içerir.

`Result` tipinin değerleri, herhangi bir tipin değerleri gibi, üzerlerinde
tanımlı metodlara sahiptir. Bir `Result` örneği, çağırabileceğiniz bir
[`expect` metoduna][expect] sahiptir. Eğer bu `Result` örneği, bir `Err` değeri
ise, `expect`, programın patlamasına neden olacak ve `expect`'e argüman olarak
geçirdiğiniz mesajı görüntüleyecektir. Eğer `read_line` metodu bir `Err`
döndürürse, bu muhtemelen alttaki işletim sisteminden gelen bir hatanın sonucu
olacaktır. Eğer bu `Result` örneği bir `Ok` değeri ise, `expect`, `Ok`'in
tuttuğu döndürme değerini alacak ve sadece bu değeri döndürecektir, böylece
bunu kullanabilirsiniz. Bu durumda, bu değer, kullanıcının girdisindeki
baytların sayısıdır.

Eğer `expect`'i çağırmazsanız, program derlenir ama bir uyarı alırsınız:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust, `read_line`'dan döndürülen `Result` değerini kullanmadığınız konusunda
sizi uyarıyor. Bu da programın muhtemel bir hatayı ele almadığı anlamına gelir.

Uyarıyı bastırmanın doğru yolu, hata ele alma kodunu yazmaktır ancak bizim
durumumuzda bir problem olduğunda programın patlamasını istiyoruz yani
`expect`'i kullanabiliriz. [9. bölümde][recover] hataları iyileştirmek ile
ilgili bilgi edineceksiniz.

### `println!` Yer Tutucuları ile Değerleri Yazdırmak

Kapayan süslü parantez dışında kodda tartışacak bir satır kaldı:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Bu satır, şimdi kullanıcı girdisini içeren string'i yazdırır. Süslü
parantezler `{}` bir yer tutucudur: `{}` ikilisini, bir değeri yerinde tutan
küçük yengeç kıskaçları olarak düşünün. Bir değişkenin değerini
yazdırdığımızda, değişken ismi süslü parantezler içine girebilir. Bir deyimi
değerlendirmenin sonucunu yazdıracağınız zaman, biçimlendirme string'i içine
boş süslü parantezler koyun, sonra aynı sırada her boş süslü parantez içinde
yazdırmak için virgüllerle ayırdığınız deyim listesini biçimlendirme
string'inden sonra sıralayın. Bir değişkeni ve bir deyimin sonucunu
`println!`'e bir çağrıda yazdırmak şu şekilde görünürdü:

```rust
let x = 5;
let y = 10;

println!("x = {x} ve y + 2 = {}", y + 2);
```

Bu kod `x = 5 ve y + 2 = 12` yazdıracaktır.

### İlk Kısmı Test Etmek

Haydi tahmin oyununun ilk kısmını test edelim. `cargo run` ile çalıştırın:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Lütfen tahmininizi girin.
6
Şunu tahmin ettiniz: 6
```

Bu noktada, oyunun ilk kısmı tamam: klavyeden girdi alıyoruz ve sonra bunu
yazdırıyoruz.

## Gizli bir Sayı Üretmek

Şimdi kullanıcının tahmin edeceği gizli bir sayı üretmemiz gerekiyor. Gizli
sayı her seferinde farklı olmalı, böylece oyun birden fazla kez oynamak için
de eğlenceli olsun. 1 ve 100 arasında rasgele bir sayı kullanacağız, böylece
oyun çok zor olmayacak. Rust henüz standart kütüphanesinde rasgele sayı
işlevselliği içermiyor. Fakat Rust ekibi bahsedilen işlevselliğe sahip
[`rand` crate'ini][randcrate] sağlamaktadır.

### Daha Fazla Fonksiyonellik için bir Crate Kullanmak

Bir crate'in Rust kaynak kod dosyalarının bir koleksiyonu olduğunu hatırlayın.
İnşa ettiğimiz proje bir *ikili crate*, yani bir çalıştırılabilir. `rand`
crate'i bir *kütüphane crate'i*, yani diğer programlarda kullanılmak üzere
kodlar içerir ve kendi başına çalıştırılamaz.

Cargo'nun harici crate'leri koordine etmesi, Cargo'nun en iyi olduğu şeydir.
`rand` kullanan kodu yazmadan önce, bu crate'i bağımlılık olarak eklemek için
*Cargo.toml* dosyasını değiştirmeliyiz. Bu dosyayı açın ve aşağıdaki satırı
Cargo'nun sizin için oluşturduğu, dosyanın altındaki `[dependencies]` bölüm
başlığının altına ekleyin. `rand`'ı tam olarak buradaki gibi, bu sürüm numarası
ile belirttiğinize emin olun, yoksa bu kılavuzdaki kod örnekleri
çalışmayabilir:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

*Cargo.toml* dosyasında, bir başlığı takip eden her şey, bu kısmın bir
parçasıdır ve bir sonraki kısım başlayana kadar sürer. `[dependencies]`'de
Cargo'ya projenizin hangi harici crate'lere bağımlı olduğunu ve bu crate'lerin
hangi sürümlerinin gerekli olduğunu söylüyorsunuz. Bu durumda `rand` crate'ini
`0.8.5` anlamsal sürüm belirteci ile belirtiyoruz. Cargo [Anlamsal
Sürümlemeyi][semver] (bazen *SemVer* de denir) anlar. Bu, sürüm numaralarını
yazmak için bir standarttır. `0.8.5` belirteci aslında `^0.8.5` için bir
kısayoldur. Bu, en az 0.8.5 ama 0.9.0'dan küçük herhangi bir sürüm anlamına
gelir.

Cargo, bu sürümlerin 0.8.5 sürümü ile uyumlu genel API'larının olduğunu
düşünür, ve bu belirtim, bu bölümdeki kodlar ile derlenmesinde bir sorun
olmayan en son yama yayınını alacağınıza emin olur. 0.9.0 ya da daha büyük
herhangi bir sürümün, aşağıdaki örneklerin kullandığı ile aynı API'ı kullandığı
garanti değildir.

Şimdi herhangi bir kodu değiştirmeden, projeyi Listing 2-2'de gösterildiği gibi
inşa edelim.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="rand crate'ini bir bağımlılık olarak ekledikten sonra `cargo build` komutunun çıktısı">

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
  Downloaded libc v0.2.127
  Downloaded getrandom v0.2.7
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
   Compiling libc v0.2.127
   Compiling getrandom v0.2.7
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

</Listing>

Farklı sürüm numaraları görüyor olabilirsiniz (fakat SemVer sayesinde hepsi kod
ile uyumlu olacaktır) ve işletim sistemine bağlı olarak sizde farklı satırlar
da olabilir. Ayrıca satırlar farklı sıralamada da olabilir.

Harici bir bağımlılık dahil ettiğimizde, Cargo, bu bağımlılığın ihtiyaç duyduğu
her şeyin en son sürümünü *kayıt*tan çeker. Bu, [Crates.io][cratesio]'daki
verinin bir kopyasıdır. Crates.io, Rust ekosistemindeki insanların, diğer
insanların kullanması için açık kaynak Rust projelerini gönderdiği yerdir.

Kaydı güncelledikten sonra Cargo, `[dependencies]` bölümünü kontrol eder ve
listelenen henüz indirilmemiş her crate'i indirir. Bu durumda sadece `rand`'ı
listelemiş olsak da, Cargo ayrıca `rand`'ın çalışması için bağımlı olduğu diğer
crate'leri de kaptı. Crate'leri indirdikten sonra, Rust onları derler ve
sonrasında projeyi mevcut bağımlılıklar ile derler.

Eğer hemen herhangi bir değişiklik yapmadan `cargo build`'ı tekrar koşarsanız,
`Finished` satırı dışında herhangi bir çıktı almazsınız. Cargo, bağımlılıkları
henüz indirip derlediğini ve bunlar hakkında *Cargo.toml* dosyanızda herhangi
bir değişiklik yapmadığınızı bilir. Cargo ayrıca kodunuz ile ilgili herhangi
bir değişiklik yapmadığınızı da bilir, yani kodu da yeniden derlemez. Herhangi
bir şey olmadığı için sadece çıkış yapar.

Eğer *src/main.rs* dosyasını açıp küçük bir değişiklik yaparsanız, ve sonra
kaydedip tekrar inşa ederseniz, sadece iki satır çıktı göreceksiniz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Bu satırlar, *src/main.rs* dosyasına yapılan küçük değişiklik ile Cargo'nun
sadece inşayı güncelleyeceğini gösterir. Bağımlılıklarınız değişmedi yani
Cargo, indirip derlediği bağımlılıkları tekrar kullanabileceğini biliyor.

#### *Cargo.lock* Dosyası ile Yeniden Üretilebilir İnşalardan Emin Olmak

Cargo, siz ya da bir başkası kodunuzu her inşa ettiğinde aynı eserin yeniden
inşa edilebildiğinden emin olmanızı sağlayan bir mekanizmaya sahiptir: Cargo,
siz diğer türlüsünü söyleyene kadar sadece belirttiğiniz bağımlılık sürümlerini
kullanacaktır. Örneğin, bir sonraki hafta `rand` crate'inin 0.8.6 sürümü geldi
diyelim ve bu sürüm önemli bir hata düzeltmesi içeriyor olsun, ama ayrıca
kodunuzu kıracak bir gerileme de içeriyor. Bunu ele almak için Rust, ilk kez
`cargo build` komutunu koştuğunuzda *Cargo.lock* dosyasını oluşturur, yani
şimdi *tahmin_oyunu* dizininde bu dosya var.

Bir projeyi ilk kez inşa ettiğinizde, Cargo, kriterlere uyan tüm bağımlılık
sürümlerini çözer ve bunları *Cargo.lock* dosyasına yazar. Gelecekte projenizi
inşa ettiğinizde, Cargo, *Cargo.lock* dosyasının var olduğunu görecek ve
tüm o sürümleri çözme işini tekrar yapmadan önce orada belirtilen sürümleri
kullanacak. Bu, otomatik olarak yeniden üretilebilir inşalara sahip olmanızı
sağlar. Diğer bir deyişle, *Cargo.lock* dosyası sayesinde projeniz, siz harici
olarak yükseltmedikçe, 0.8.5'te kalacak. *Cargo.lock* dosyası, yeniden
üretilebilir inşalar için önemli olduğundan, genellikle kaynak kontrolüne,
projenizdeki diğer kodlar ile birlikte kaydedilir.

#### Bir Crate'i Yeni bir Sürüme Güncellemek

Bir crate'i güncellemek *istediğinizde* Cargo'nun `update` komutunu
kullanabilirsiniz. Bu komut *Cargo.lock* dosyasını boşverecek ve
*Cargo.toml*'daki belirtimlerinize uyan en son sürümleri çözecektir. Cargo daha
sonra bu sürümleri *Cargo.lock* dosyasına yazacaktır. Bu durumda Cargo sadece
0.8.5'ten büyük ve 0.9.0'dan küçük sürümlere bakacaktır. Eğer `rand` crate'i,
0.8.6 ve 0.9.0 olarak iki yeni sürüm yayınlamışsa, `cargo update`'i
koştuğunuzda aşağıdakini görürsünüz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```

Cargo, 0.9.0 sürümünü boşverir. Bu noktada ayrıca *Cargo.lock*'ınızda, şu anda
kullandığınız `rand` crate'inin sürümünün 0.8.6 olduğunu ifade eden bir
değişiklik olduğunu fark edeceksiniz. `rand`'ın 0.9.0 sürümünü ya da
0.9.*x* serisindeki herhangi bir sürümü kullanmak için, *Cargo.toml* dosyasını
şu şekilde güncellemeniz gerekir:

```toml
[dependencies]
rand = "0.9.0"
```

Bir sonraki `cargo build` çalıştırışınızda Cargo, mevcut crate'lerin kaydını
güncelleyecek ve sizin `rand` gerekliliklerinizi, belirttiğiniz yeni sürüme
göre yeniden değerlendirecek.

[Cargo][doccargo] ve [ekosistemi][doccratesio] hakkında söylenecek daha birçok
şey var, ki bunları 14. bölümde tartışacağız, fakat şimdilik bu kadarını
bilmeniz yeterli. Cargo, kütüphaneleri yeniden kullanmayı oldukça
kolaylaştırır böylece Rustacean'lar, birkaç paketin bir araya gelmesi ile
oluşmuş daha küçük projeler yazabiliyorlar.

### Rasgele bir Sayı Üretmek

Haydi `rand` kullanarak tahmin etmek için bir sayı üretelim. Sonraki adım,
Listing 2-3'te gösterildiği gibi *src/main.rs*'i güncellemektir.

<Listing number="2-3" file-name="src/main.rs" caption="Rasgele bir sayı üretmek için kod eklemek">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

İlk olarak `use rand:Rng;` satırını ekledik. `Rng` trait'i, rasgele sayı
üreteçlerinin gerçeklediği metodları tanımlar, ve bu metodları kullanabilmemiz
için bu trait, kapsam içinde olmalıdır. 10. bölüm, trait'leri ayrıntılı bir
şekilde kapsayacaktır.

Şimdi ortaya iki satır ekliyoruz. İlk satırda, kullanacağımız belirli rasgele
sayı üretecini bize veren `rand::thread_rng` fonksiyonunu çağırıyoruz. Bu
üreteç, çalıştırmanın şu anki thread'inde yerel olarak yer alan ve işletim
sistemi tarafından beslenen bir üreteçtir. Sonrasında rasgele sayı üreteci
üzerinde `gen_range` metodunu çağırıyoruz. Bu metod, `use rand::Rng;` ifadesi
ile kapsam içine getirdiğimiz `Rng` trait'i tarafından tanımlanır. `gen_range`
metodu argüman olarak bir aralık deyimi alır ve bu aralıkta rasgele bir sayı
üretir. Burada kullandığımız aralık deyiminin türü `başlangıç..=son`
biçimindedir ve, aşağıdaki ve yukarıdaki sınırları içine alır, yani 1 ile 100
arasında bir sayı istemek için `1..=100` ifadesini kullanmalıyız.

> Not: Hangi trait'i kullanacağınızı ve bir crate'teki hangi metod veya
> fonksiyonu çağırmanız gerektiğini bir anda bilemezsiniz, bu sebeple her
> crate, bu crate'i nasıl kullanacağınızı açıklayan bir belgelendirmeye
> sahiptir. Cargo'nun bir diğer güzel özelliği şudur: `cargo doc --open`
> komutu, yerelinizdeki bağımlılıklar tarafından sağlanan tüm belgelendirmeyi
> inşa eder ve bunu tarayıcınızda açar. Örneğin, eğer `rand` crate'indeki bir
> diğer fonksiyonellik ile ilgileniyorsanız, `cargo doc --open` komutunu verin
> ve soldaki kenar çubuğunda `rand`'a tıklayın.

İkinci yeni satır gizli numarayı yazdırır. Bu, programı geliştirirken, test
edebilmek için kullanışlıdır, fakat bunu son sürümden sileceğiz. Eğer program,
başlar başlamaz cevabı yazdırırsa, bu pek oyun olmaz, değil mi?!

Programı birkaç kez koşmayı deneyin:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 7
Lütfen tahmininizi girin.
4
Şunu tahmin ettiniz: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 83
Lütfen tahmininizi girin.
5
Şunu tahmin ettiniz: 5
```

Farklı rasgele sayılar görmelisiniz ve bunlar 1 ile 100 arasında olmalıdır. İyi
iş!

## Tahmin ile Gizli Sayıyı Karşılaştırmak

Şimdi kullanıcı girdisi ve rasgele sayıya sahip olduğumuza göre, bunları
karşılaştırabiliriz. Bu adım Listing 2-4'te gösterilmiştir. Bu kod henüz
derlenmeyecek, bunun nedenini birazdan açıklayacağız.

<Listing number="2-4" file-name="src/main.rs" caption="İki sayıyı karşılaştırmanın muhtemel döndürme değerlerini ele alma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

İlk olarak bir diğer `use` ifadesi ekliyoruz ve standart kütüphaneden
`std:cmp::Ordering` tipini kapsam içine getiriyoruz. `Ordering` tipi, `Less`,
`Greater` ve `Equal` varyantlarına sahip bir diğer enum'dır. Bunlar, iki değeri
karşılaştırdığınızda muhtemel üç sonuçtur.

Sonrasında aşağıya, `Ordering` tipini kullanan 5 yeni satır ekliyoruz. `cmp`
metodu iki değeri karşılaştırır ve karşılaştırılabilir herhangi bir şey
üzerinden çağrılabilir. Argüman olarak, ne ile karşılaştırmak istiyorsanız, onu
alır: burada `tahmin` ile `gizli_sayı`yı karşılaştırıyor. Sonrasında `use`
ifadesi ile kapsama getirdiğimiz `Ordering` enum'ının bir varyantını
döndürüyor. `cmp`'ye `guess` ve `gizli_sayı` değerlerini vererek hangi
`Ordering` varyantının döndüğüne bağlı olarak sonraki adımda ne yapacağımıza
karar vermek için [`match`][match] deyimini kullanacağız.

Bir `match` deyimi, *kollar*dan meydana gelir. Bir kol, eşleştirme yapmak için
bir *desen* ve `match`'e verilen değer bu kolun desenine uyarsa çalıştırılması
gereken koddan oluşur. Rust, `match`'e verilen değeri alır ve her kolun
desenine sırayla bakar. Desenler ve `match` yapısı, güçlü Rust özellikleridir:
Bunlar kodunuzun karşılaşabileceği birçok durumu ifade edebilmenizi ve bunların
tümünü ele aldığınıza emin olmanızı sağlar. Bu özellikler ayrıntılı olarak
sırasıyla 6. ve 18. bölümde kapsanacak.

Hadi burada kullandığımız `match` deyimi ile bir örnek yapalım. Mesela
kullanıcının 50 tahmin ettiği sayının 50 olduğunu ve rasgele oluşturulan gizli
sayının da 38 olduğunu varsayalım.

Kod, 50 ile 38'i karşılaştırdığında, `cmp` metodu `Ordering::Greater`'ı
döndürecektir çünkü 50, 38'den büyüktür. `match` deyimi `Ordering::Greater`
değerini alacak ve her kolun desenini kontrol etmeye başlayacak. İlk kolun
deseni olan `Ordering::Less`'e bakacak ancak `Ordering::Greater`'ın
`Ordering::Less` ile eşleşmediğini görecek, bu sebeple bu koldaki kodu
boşverecek ve sonraki kola ilerleyecek. Bir sonraki kolun deseni
`Ordering::Greater`, ki bu desen `Ordering::Greater` ile *eşleşir*! Bu koldaki
ilişkili kod çalışacak ve ekrana `Çok büyük!` yazdıracaktır. `match` deyimi,
ilk başarılı eşleşmeden sonra biter, yani bu senaryoda son kola bakmayacaktır.

Fakat, Listing 2-4'teki kod henüz derlenmiyor. Hadi deneyelim:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Hata, *eşleşmeyen tipler* olduğunu belirtmektedir. Rust, güçlü, statik bir tip
sistemine sahiptir. Fakat ayrıca tip çıkarımına sahiptir.
`let mut tahmin = String::new()` yazdığımızda Rust, bu `tahmin`'in bir `String`
olması gerektiği çıkarımını yapabilir ve bizi, bu tipi yazmak zorunda bırakmaz.
Diğer yandan `gizli_sayı`, bir sayı tipidir. Birkaç Rust sayı tipi, 1 ile 100
arasındaki bir değere sahip olabilir: `i32`, 32-bit bir sayı; `u32`, işaretsiz
32-bit bir sayı; `i64`, 64-bit bir sayı; ve diğerleri. Başka türlüsü
belirtilmediği sürece Rust, `i32`'yi kullanır. Bu durumda `gizli_sayı`nın tipi
`i32`'dir, tabi başka bir yere tip bilgisi eklememişseniz, ki bu durumda Rust,
farklı bir sayısal tipe çıkarım yapacaktır. Hatanın sebebi, Rust'ın bir string
ile bir sayı tipini karşılaştıramıyor olmasıdır.

Sonuç olarak, programın girdi olarak okuduğu `String`'i bir sayı tipine
çevirmek istiyoruz, böylece bunu sayısal olarak gizli sayıyla
karşılaştırabiliriz. Bunu, şu satırı `main` fonksiyon gövdesine ekleyerek
yapacağız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Satır şu:

```rust,ignore
let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı girin!");
```

`tahmin` isminde bir değişken oluşturduk. Ama durun, programda zaten `tahmin`
isminde bir değişken yok muydu? Vardı fakat Rust bize yardımcı olmak için
`tahmin`in önceki değerini yeni bir değer ile gölgelememize izin verir.
*Gölgeleme*, `tahmin_str` ve `tahmin` gibi iki farklı değişken tanımlamamıza
gerek kalmadan `tahmin` değişken adını tekrar kullanmamızı sağlar. Bunu
ayrıntılı olarak [3. bölümde][shadowing] ele alacağız ancak şimdilik, bu
özelliğin, bir tipi diğer bir tipe dönüştürmek istediğimizde sıklıkla
kullanıldığını bilin.

Bu değişkeni `tahmin.trim().parse()` deyimine atadık. Deyimdeki `tahmin`,
girdiyi bir string olarak içeren orijinal `tahmin` değişkenini kastediyor. Bir
`String` örneğindeki `trim` metodu, baştaki ve sondaki tüm boşlukları
eleyecektir. Bunu, string'i `u32` ile karşılaştırmak için yapmalıyız çünkü
`u32` sadece sayısal veri içerebilir. Kullanıcı `read_line`'ı memnun etmek ve
tahminini girmek için <kbd>enter</kbd> tuşuna basmalıdır, ki bu da string'e bir
yenisatır karakteri ekleyecektir. Örneğin, eğer kullanıcı <kbd>5</kbd>
girmişse ve <kbd>enter</kbd> tuşuna basmışsa, `tahmin` şunun gibi görünecektir:
`5\n`. `\n`, “yenisatır”ı ifade eder. (Windows'ta <kbd>enter</kbd> tuşuna
basmak, bir taşıma geri dönüşü ve bir yeni satır ile sonuçlanacaktır: `\r\n`.)
`trim` metodu, `\n` ya da `\r\n`'yi eler ve sadece `5` kalır.

[String'lerdeki `parse` metodu][parse], bir string'i başka bir tipe dönüştürür.
Burada bunu, bir string'den bir sayıya dönüştürmek için kullanıyoruz. Rust'a
istediğimiz tam sayı tipini söylemeliyiz: `let tahmin: u32`. `tahmin`den
sonraki (`:`), Rust'a bizim değişkenin tipini bildireceğimizi söyler. Rust
birkaç gömülü sayı tipine sahiptir; burada görünen `u32`, işaretsiz, 32-bit bir
tamsayıdır. Bu, pozitif küçük bir sayı için iyi bir varsayılan seçimdir. Diğer
sayı tipleri hakkında [3. bölümde][integers] bilgi edineceksiniz.

Ek olarak bu örnek programdaki `u32` bildirimi ve `gizli_sayı` ile
karşılaştırma, Rust'ın bu `gizli_sayı`nın da bir `u32` olması gerektiğini
çıkaracağı anlamına gelir. Yani şimdi karşılaştırma aynı tipin iki değeri
arasında olacak!

`parse` metodu, mantıken sayılara dönüştürülebilen karakterler üzerinde işe
yarayacaktır yani kolaylıkla hatalara sebep olabilir. Örneğin, eğer bir string
`A👍%` karakterlerini içeriyorsa, bunu bir sayıya dönüştürmenin herhangi bir
yolu olmayacaktır. Başarısız olabileceği için, `parse` metodu bir `Result` tipi
döner, aynı `read_line` metodunun yaptığı gibi (daha önce [“`Result` ile
Potansiyel Başarısızlığı Ele
Alma”](#result-ile-potansiyel-başarısızlığı-ele-alma) bölümünde tartışılmıştı).
Bu `Result`'a da `expect` metodundaki kullanım ile aynı şekilde davranacağız.
Eğer `parse`, string'den bir sayı oluşturamadığı için bir `Err` `Result`
varyantı döndürürse, `expect` çağrısı oyunu patlatacak ve verdiğimiz mesajı
yazdıracak. Eğer `parse`, başarılı bir şekilde string'i bir sayıya
dönüştürebilirse, `Result`'ın `Ok` varyantını döndürecek, ve expect, `Ok`
değerinden bizim istediğimiz sayıyı döndürecek.

Hadi şimdi programı koşalım:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 58
Lütfen tahmininizi girin.
  76
Şunu tahmin ettiniz: 76
Çok büyük!
```

Güzel! Tahminden önce boşluklar eklenmiş olsa da, program yine de kullanıcının
76'yı tahmin ettiğini anladı. Farklı türde girdiler ile farklı davranışları
doğrulamak için programı birkaç kez çalıştırın: sayıyı doğru tahmin edin, çok
büyük bir sayı tahmin edin ve çok küçük bir sayı tahmin edin.

Oyunun büyük kısmı artık çalışıyor fakat kullanıcı sadece bir tahmin
yapabiliyor. Hadi bunu, bir döngü ekleyerek değiştirelim!

## Döngüleme ile Birden Fazla Tahmine İzin Verme

`loop` anahtar kelimesi sonsuz bir döngüye sebep olur. Kullanıcıya, sayıyı
tahmin etmede daha fazla şans vermek için bir döngü ekleyeceğiz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Gördüğünüz gibi, tahmin girdisi isteminden sonraki her şeyi bir döngüye
taşıdık. Döngünün içindeki her satırı dört boşluk daha girintilediğinizden emin
olun ve programı tekrar çalıştırın. Şimdi program bir sonraki tahmin için
sonsuza dek soracak, ki bu bir başka probleme neden oluyor. Kullanıcı çıkış
yapamıyor gibi görünüyor!

Kullanıcı her zaman programı <kbd>ctrl</kbd>-<kbd>c</kbd> klavye kısayolu
kullanarak kesebilir. Fakat bu doyumsuz canavardan kaçınmak için bir başka yol
daha var, ki bundan [“Tahmin ile Gizli Sayıyı
Karşılaştırmak”](#tahmin-ile-gizli-sayıyı-karşılaştırmak) bölümündeki `parse`
tartışmasında bahsetmiştik: eğer kullanıcı sayı olmayan bir cevap girerse,
program patlayacaktır. Bunu, kullanıcının çıkış yapmasına izin vermek için şu
şekilde kullanabiliriz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 59
Lütfen tahmininizi girin.
45
Şunu tahmin ettiniz: 45
Çok küçük!
Lütfen tahmininizi girin.
60
Şunu tahmin ettiniz: 60
Çok büyük!
Lütfen tahmininizi girin.
59
Şunu tahmin ettiniz: 59
Kazandın!
Lütfen tahmininizi girin.
çık
thread 'main' panicked at 'Lütfen bir sayı girin!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`çık` yazmak oyundan çıkılmasına neden olacak, fakat fark edebileceğiniz gibi
diğer sayı olmayan girdiler de buna neden olacak. Bu, en basit tabirle
standardın altındadır; ayrıca doğru sayı tahmin edildiğinde oyunun durmasını
istiyoruz.

### Doğru bir Tahminden Sonra Çıkma

Hadi bir `break` ifadesi ekleyerek oyunu, kullanıcı kazandığı zaman çıkacak
şekilde programlayalım.

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

`Kazandın!` ifadesinden sonra bir `break` eklemek, kullanıcı gizli sayıyı doğru
bir şekilde tahmin ettiğinde programın döngüden çıkmasına sebep olur. Döngüden
çıkmak ayrıca programdan da çıkmak anlamına gelir çünkü döngü `main`'in son
kısmıdır.

### Geçersiz Girdiyi Ele Almak

Oyunun davranışını biraz daha hassaslaştırmak için, kullanıcı sayı dışında bir
girdi girdiğinde programı patlatmak yerine, hadi programın sayı olmayan
girdileri boşvermesini sağlayalım, böylece kullanıcı tahmin etmeye devam
edebilir. Bunu, `tahmin`in `String`'den `u32`'ye dönüştürüldüğü satırı
düzenleyerek yapabiliriz, Listing 2-5'te gösterildiği gibi.

<Listing number="2-5" file-name="src/main.rs" caption="Sayı olmayan tahminleri boşvermek ve programın patlaması yerine bir diğer tahmin isteme">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

`expect` çağrısından `match` deyimine geçtik, böylece programı bir hatada
patlatmaktan hatayı ele almaya da geçmiş olduk. `parse`'ın bir `Result` tipi
döndürdüğünü ve `Result`'ın `Ok` ve `Err` varyantları olan bir enum olduğunu
hatırlayın. Burada bir `match` deyimi kullanıyoruz, tıpkı `cmp` metodunun
`Ordering` sonucunda yaptığımız gibi.

Eğer `parse`, bir string'i bir sayıya başarılı bir şekilde döndürebilirse, çıkan
sayıyı içeren `Ok` değerini dönecek. Bu `Ok` değeri ilk kolun deseniyle
eşleşecek ve `match` deyimi sadece, `parse`'ın üretip `Ok` içine koyduğu `sayı`
değerini döndürecek. Bu sayı, oluşturduğumuz `guess` değişkeni içinde yerini
alacak, tam da istediğimiz yerde!

Eğer `parse`, string'i bir sayıya *döndüremezse*, hata hakkında daha fazla
bilgi içeren `Err` değerini döndürecek. `Err` değeri ilk `match` kolundaki
`Ok(sayı)` deseni ile eşleşmeyecek ama ikinci koldaki `Err(_)` deseni ile
eşleşecek. Altçizgi (`_`) bir hepsini yakala değeri; bu örnekte, içinde hangi
bilgileri taşıyor olurlarsa olsunlar, tüm `Err` değerlerini eşleştirmek
istediğimizi söylüyoruz. Yani program ikinci kolun kodunu yani `continue` kodunu
çalıştıracak, bu kod programa, döngünün bir sonraki yinelemesine geçmesini ve
bir diğer tahmini istemesini söyler. Yani program, etkili bir şekilde `parse`'ın
karşılaşabileceği tüm hataları boşverir!

Şimdi programdaki her şey beklendiği gibi çalışmalıdır. Hadi deneyelim:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/tahmin_oyunu`
Sayıyı tahmin et!
Gizli sayı: 61
Lütfen tahmininizi girin.
10
Şunu tahmin ettiniz: 10
Çok küçük!
Lütfen tahmininizi girin.
99
Şunu tahmin ettiniz: 99
Çok büyük!
Lütfen tahmininizi girin.
foo
Lütfen tahmininizi girin.
61
Şunu tahmin ettiniz: 61
Kazandın!
```

Şahane! Son bir minik çimdik ile, tahmin oyununu bitireceğiz. Programın hala
gizli sayıyı yazdırdığını hatırlayın. Bu, test etme için iyidir ama oyunu
mahvetmekte. Hadi gizli sayıyı çıktı veren `println!`'i silelim. Listing 2-6 son
kodu göstermektedir.

<Listing number="2-6" file-name="src/main.rs" caption="Tahmin oyunu kodunu tamamla">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

Bu noktada, tahmin oyununu başarılı bir şekilde inşa ettiniz. Tebrikler!

## Özet

Bu proje, size birçok yeni Rust kavramını anlatmak için uygulamalı bir yoldu:
`let`, `match`, fonksiyonlar, harici crate'lerin kullanımı ve daha fazlası.
Sonraki birkaç bölümde, bu kavramlar ile ilgili daha ayrıntılı bir şekilde
öğreneceksiniz. 3. bölüm, değişkenler, veri tipleri ve fonksiyonlar gibi birçok
programlama dilinin sahip olduğu kavramları kapsar ve bunları Rust'ta nasıl
kullanacağınızı gösterir. 4. bölüm sahipliği keşfeder, bu özellik Rust'ı diğer
dillerden farklı yapar. 5. bölüm struct metod sözdizimini tartışır, ve 6. bölüm
enum'ların nasıl çalıştığını açıklar.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
