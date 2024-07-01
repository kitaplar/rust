## Veri Tipleri

Rust'taki her değer belirli bir `veri tipi`ndedir, bu Rust'a hangi tür verinin
belirtildiğini söyler, böylece Rust, bu veri ile nasıl çalışması gerektiğini
bilir. İki veri tipi alt kümesine bakacağız: skaler ve bileşik.

Rust'ın *statik tiplenen* bir dil olduğunu unutmayın, bu demek oluyor ki Rust,
tüm değişkenlerin değerini derleme zamanında bilmelidir. Derleyici genellikle
değer ve bu değeri nasıl kullandığımız üzerinden hangi tipi kullanmak
istediğimizi çıkarabilir. Birçok tipin mümkün olduğu durumlarda, mesela 2.
bölümdeki [“Tahmin ile Gizli Sayıyı
Karşılaştırmak”][comparing-the-guess-to-the-secret-number] bölümünde `parse`
kullanarak bir `String`'i bir sayısala dönüştürdüğümüzde, şu şekilde bir tip
bildirimi eklemeliyiz:

```rust
let tahmin: u32 = "42".parse().expect("Bir sayı değil!");
```

Eğer bu kodda `: u32` tip bildirimini eklemezsek Rust, aşağıdaki hatayı
görüntüleyecektir, ki bu, derleyicinin hangi tipi kullanmak istediğimiz hakkında
daha fazla bilgiye ihtiyaç duyduğu anlamına gelir:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Diğer veri tipleri için farklı tip bildirimleri göreceksiniz.

### Skaler Tipler

Bir *skaler* tip, tek bir değeri ifade eder. Rust dört birincil skaler tipe
sahiptir: tamsayılar, kayan noktalı sayılar, Boolean'lar ve karakterler. Bunları
diğer dillerden tanımış olabilirsiniz. Hadi bunların Rust'ta nasıl çalıştığına
bakalım.

#### Tamsayı Tipleri

Bir *tamsayı*, kesirli bir bileşeni olmayan bir sayıdır. 2. bölümde bir tamsayı
tipi kullanmıştık: `u32` tipi. Bu tip belirtimi, bunun ilişkilendirildiği
değerin işaretsiz bir tamsayı (işaretli tamsayı tipleri, `u` yerine `i` ile
başlar) olması gerektiğini ve 32 bit'e kadar alan kaplayacağını belirtir. Tablo
3-1, Rust'taki yerleşik tamsayı tiplerini göstermektedir. Bu varyantların
herhangi birini bir tamsayı değerinin tipini bildirmek için kullanabiliriz.

<span class="caption">Tablo 3-1: Rust'taki Tamsayı Tipleri</span>

| Uzunluk | İşaretli | İşaretsiz |
|---------|----------|-----------|
| 8 bit   | `i8`     | `u8`      |
| 16 bit  | `i16`    | `u16`     |
| 32 bit  | `i32`    | `u32`     |
| 64 bit  | `i64`    | `u64`     |
| 128 bit | `i128`   | `u128`    |
| mimari  | `isize`  | `usize`   |

Her varyant ya işaretli ya da işaretsizdir ve bir uzunluğa sahiptir. *İşaretli*
ve `işaretsiz`, sayının negatif olmasının mümkün olup olmadığına işaret eder,
bir diğer deyişle, sayının kendisiyle birlikte bir işarete ihtiyacı olup
olmadığını (işaretli) ya da sadece pozitif olabileceği için sayının bir işaret
olmadan ifade edilip edilemeyeceğini (işaretsiz) belirtir. Bu tıpkı sayıları
kağıda yazmak gibidir: işaret önemli olduğunda, sayı bir artı ya da eksi işareti
ile gösterilir; ama sayının pozitif olduğunu varsaymak güvenli olduğunda,
sayı işaretsiz gösterilir. İşaretli sayılar, [ikinin tümleyeni][twos-complement]
gösterimi kullanılarak saklanır.

Her işaretli varyant, -(2<sup>n - 1</sup>)'den 2<sup>n - 1</sup> - 1'e kadar
(dahil) sayı tutabilir, buradaki *n*, varyant kullandığı bit sayısıdır. Yani bir
`i8`, -(2<sup>7</sup>)'den 2<sup>7</sup> - 1'e kadar sayı tutabilir, ki bu
-128'den 127'ye kadar demektir. İşaretsiz varyantlar, 0'dan 2<sup>n</sup> -1'e
kadar sayı tutabilirler, yani bir `u8`, 0'dan 2<sup>8</sup> - 1'e kadar sayı
tutabilir, ki bu da 0'dan 255'e kadar demektir.

Ek olarak, `isize` ve `usize` tipleri programınızın koştuğu bilgisayarın
mimarisine bağımlıdır, ki bu, tabloda “mimari” olarak belirtilmiştir: eğer
64 bit mimari üzerindeyseniz, 64 bit ve eğer 32 bit mimari üzerindeyseniz, 32
bit.

Tamsayı literallerini Tablo 3-2'de gösterilen herhangi bir biçimde
yazabilirsiniz. Birden fazla sayısal tip olabilen sayı literallerinin, tipi
tayin etmek için bir tip sonekine izin verdiğine dikkat edin, mesela `57u8`
gibi. Sayı literalleri ayrıca `_`'yi görsel bir ayraç olarak da kullanabilir,
böylece sayı daha kolay okunur hale gelir, mesela `1_000` gibi, ki bu sayı,
`1000` ile aynı değere sahip olacaktır.

<span class="caption">Tablo 3-2: Rust'ta Tamsayı Literalleri</span>

| Sayı literalleri   | Örnek         |
|--------------------|---------------|
| Ondalık            | `98_222`      |
| Onaltılık          | `0xff`        |
| Sekizlik           | `0o77`        |
| İkilik             | `0b1111_0000` |
| Bayt (sadece `u8`) | `b'A'`        |

Peki hangi tamsayıyı kullanacağınızı nasıl bileceksiniz? Eğer emin değilseniz,
Rust'ın varsayılanları genelde başlamak için iyidir: tamsayı tipleri varsayılan
olarak `i32`'dir. `isize` ya da `usize`'ı kullanacağınız birincil durum, bir
çeşit koleksiyonu indekslemektir.

> ##### Tamsayı Taşması
>
> Diyelim ki 0 ile 255 arasındaki değerleri tutabilen `u8` tipinde bir
> değişkeniniz var. Eğer değişkeni, 256 gibi bu aralık dışında bir değere
> değiştirmeyi denerseniz, *tamsayı taşması* meydana gelecektir, ki bu, iki
> davranıştan biriyle sonuçlanır. Hata ayıklama modunda derlediğinizde Rust,
> tamsayı taşması için kontrolleri dahil edecek ve bu durumda program, çalışma
> zamanında *panikleyecektir*. Rust, *panikleme* terimini, bir program bir hata
> ile çıkış yaptığı zaman kullanır; Panikleri 9. bölümün [“`panic!` ile
> Düzeltilemez Hatalar”][unrecoverable-errors-with-panic] bölümünde ayrıntılı
> bir şekilde işleyeceğiz.
>
> `--release` bayrağı ile yayın modunda derlediğinizde, Rust, tamsayı taşması
> için paniğe neden olabilecek herhangi bir kontrol *eklemez*. Bunun yerine,
> eğer taşma gerçekleşirse, Rust, *ikinin tümleyeni sarma*yı uygular. Kısaca,
> tipin tutabileceği maksimum değerden büyük değerler, tipin tutabileceği
> minimum değerin “etrafını sarar”. `u8` durumunda, 256 değeri, 0 olur; 257
> değeri 1 olur; vb. Program paniklemez fakat değişken muhtemelen sizin olmasını
> beklediğiniz değerde olmayacak. Tamsayı taşmasının sarma davranışına güvenmek,
> bir hata olarak düşünülür.
>
> Taşma ihtimalini harici olarak ele almak için, ilkel sayısal tipler için
> standart kütüphanedeki şu metod ailelerini kullanabilirsiniz:
>
> * Tüm modları `wrapping_*` metodları ile sarın, mesela `wrapping_add`.
> * Eğer taşma varsa `checked_*` metodu ile `None` değeri döndürün.
> * `overflowing_*` metodları ile değeri ve bir taşma olup olmadığını belirten
>   bir boolean döndürün.
> * `saturating_*` metodları ile değerin minimum ya da maksimum değerinde
>   doyurmak

#### Kayan Noktalı Tipler

Rust ayrıca *kayan noktalı sayılar* (ondalık noktalı sayılar) için iki ilkel
tipe sahiptir. Bu tipler `f32` ve `f64`'tür, ve sırasıyla 32 bit ve 64 bit
büyüklüğe sahiptir. Varsayılan tip `f64`'tür çünkü bu tip, modern işlemcilerde
`f32` ile neredeyse aynı hızdadır ancak daha kesindir. Tüm kayan noktalı tipler
işaretlidir.

İşte kayan noktalı sayıları iş başında gösteren bir örnek:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Kayan noktalı sayılar, IEEE-754 standardına göre ifade edilirler. `f32` tipi,
tek kesinlikte kayandır ve `f64`, çift kesinliğe sahiptir.

#### Sayısal İşlemler

Rust, tüm sayı tipleri için bekleyeceğiniz tüm temel matematiksel işlemleri
destekler: ekleme, çıkarma, çarpma, bölme ve kalan. Tamsayı bölme, en yakın
tamsayıya doğru sıfırları keser. Aşağıdaki kod her sayısal işlemi nasıl
kullanacağınızı `let` ifadelerinde göstermektedir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Bu ifadelerdeki her deyim bir matematiksel işlem kullanır ve tek bir değere
değerlenir, sonrasında bu değer, bir değişkene bağlanır. [B eki][appendix_b],
Rust'ın sağladığı tüm işleçleri içermektedir.

#### Boolean Tipi

Birçok diğer programlama dilinde olduğu gibi, Rust'ta bir Boolean tipi iki
muhtemel değere sahiptir: `true` ve `false`. Boolean'lar, büyüklük olarak bir
bayttır. Rust'taki Boolean tipi, `bool` ile belirtilir. Örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Boolean değerlerini kullanmanın ana yolu, koşulsalları kullanmaktır, mesela
bir `if` deyimi. `if` deyimlerinin Rust ile nasıl çalıştığını [“Kontrol
Akışı”][control-flow] bölümünde inceleyeceğiz.

#### Karakter Tipi

Rust'ın `char` tipi, dilin en ilkel alfabetik tipidir. İşte `char` değerleri
bildirmenin bazı örnekleri:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

`char` literalleri tek tırnak ile belirttiğimize dikkat edin, string
literallerin aksine, ki onlar çift tırnak kullanır. Rust'ın `char` tipi büyüklük
olarak dört bayttır ve bir Unicude Skaler Değerini ifade eder, ki bu, bu tipin
ASCII'den çok daha fazlasını ifade edebileceğini gösterir. Rust'ta aksan
harfleri; Çin, Japonca ve Korece karakterler; emoji; ve sıfır genişlikte
boşluklar hep geçerli `char` değerleridir. Unicode Skaler Değerler, `U+0000`'dan
`U+D7FF`'ye kadar ve `U+E000`'dan `U+10FFFF`'ye kadar (dahil) bir aralıktadır.
Fakat Unicode'da bir “karakter” gerçekte bir kavram değildir, yani sizin
sezgisel olarak bir “karakter” hakkındaki yorumunuz, Rust'taki `char` ile
eşleşmeyebilir. Bu konuyu 8. bölümde [“String'ler ile UTF-8 ile Kodlanmış
Metinleri Saklamak”][strings] bölümünde ayrıntılı bir şekilde tartışacağız.

### Bileşik Tipler

*Bileşik tipler* birden çok değeri bir tipe gruplayabilir. Rust iki ilkel
bileşik tipe sahiptir: demetler ve diziler.

#### Demet Tipi

Bir *demet*, çeşitli tipteki değerleri bir bileşik tipe gruplamak için genel bir
yoldur. Demetler sabit bir büyüklüğe sahiptir: bir kez bildirildiler mi,
büyüyemez ya da küçülemezler.

Bir demeti, değerleri parantezler içine, virgüllerle ayrılmış şekilde yazarak
oluşturabiliriz. Demetteki her bir pozisyonun bir tipi vardır ve demetteki
farklı değerlerin tiplerinin birbirleri ile aynı olması gerekmez. Bu örnekte,
isteğe bağlı tip bildirimlerini ekledik:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

`demet` değişkeni, tüm demete bağlanır çünkü demet, tek bir bileşik öğe olarak
düşünülür. Bir demetteki bireysel değerleri almak için, desen eşlemeyi
kullanabiliriz, böylece bir demet değerini yıkarız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Bu program önce bir demet oluşturur ve bu demeti `demet` isimli değişkene
bağlar. Daha sonra `demet`i üç farklı değişkene (`x`, `y` ve `z`) ayırmak için
`let` ile bir desen kullanır. Buna *yıkma* denir çünkü bu, bir demeti üç
parçaya ayırır. Son olarak, program `y`'nin değerini yazdırır, ki burada `6.4`.

Bir demet öğesine ayrıca bir nokta (`.`) ve sonrasında erişmek istediğimiz
değerin indeksini kullanarak da erişebiliriz. Örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Bu program `a` demetini oluşturmakta, sonra demetin her öğesine, bu öğelerin
indekslerini kullanarak erişmektedir. Birçok programlama dilindeki gibi, bir
demetteki ilk indeks, 0'dır.

Değeri olmayan demet özel bir isme sahiptir: *birim*. Bu değer ve bu değerin
karşılık gelen tipi `()` olarak yazılır ve boş bir değeri ifade eder, ya da boş
bir döndürme tipini. Deyimler, eğer başka bir değer döndürmezlerse, dahili
olarak birim değerini döndürürler.

#### Dizi Tipi

Birden çok değerin bir koleksiyonuna sahip olmak için bir diğer yol, bir *dizi*
kullanmaktır. Demetin aksine, bir dizinin her öğesi aynı tipte olmalıdır. Bazı
diğer dillerdeki dizilerin aksine, Rust'taki diziler sabit uzunluğa sahiptir.

Bir dizideki değerleri, köşeli parantezler içine virgülle ayırarak yazarız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Diziler, verinizin heap'te değil de stack'te tahsis edilmesini (stack ve heap'i
[4. bölümde][stack-and-heap] tartışacağız) ya da her zaman sabit sayıda öğenizin
olduğuna emin olmak istiyorsanız, kullanışlıdır. Ancak bir dizi, vektör tipi
kadar esnek değildir. Bir *vektör*, standart kütüphane tarafından sağlanan
benzer bir koleksiyon tipidir ancak büyüyüp küçülmesine *izin verilir*. Bir dizi
mi yoksa bir vektör mü kullanacağınıza emin değilseniz, büyük ihtimalle bir
vektör kullanmalısınız. [8. bölüm][vectors], vektörleri ayrıntılı bir şekilde
tartışmaktadır.

Fakat, öğe sayısının değişmeye ihtiyacı olmadığını bildiğinizde, diziler daha
kullanışlıdır. Örneğin, eğer bir programda ayların isimlerini kullanacak
olsaydınız, muhtemelen bir vektör yerine bir dizi kullanırdınız çünkü her zaman
12 öğe olacağını bilirsiniz:

```rust
let aylar = ["Ocak", "Şubat", "Mart", "Nisan", "Mayıs", "Haziran", "Temmuz",
             "Ağustos", "Eylül", "Ekim", "Kasım", "Aralık"];
```

Bir dizinin tipini, köşeli parantezler içinde öğelerin tipini, bir noktalı
virgül ve sonra öğelerin sayısını yazarak belirtirsiniz:

```rust
let d: [i32; 5] = [1, 2, 3, 4, 5];
```

Burada `i32`, her öğenin tipidir. Noktalı virgülden sonraki `5`, dizinin 5 öğe
içerdiğini belirtir.

Ayrıca bir dizinin her öğe için aynı değeri ilklendirmesini de
sağlayabilirsiniz. Bunun için köşeli parantezler içine ilk değeri, sonra noktalı
virgülü, sonrasında da dizinin uzunluğunu yazarsınız:

```rust
let d = [3; 5];
```

`d` isimli dizi, `5` öğe içerecek ve bu öğelerin hepsi `3`'e ayarlanmış
olacaktır. Bu, `let d = [3, 3, 3, 3, 3];` yazmakla aynıdır ancak daha kısadır.

##### Dizi Öğelerine Erişmek

Bir dizi, stack'te ayrılabilecek bilinen sabit büyüklükte tek bir yığın
hafızadır. Bir dizinin öğelerine indeksleme ile erişebilirsiniz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

Bu örnekte `ilk`, `1` değerini alacaktır çünkü `[0]` indeksindeki değer budur.
`ikinci` değişkeni ise `[1]` indeksindeki `2` değerini alacaktır.

##### Geçersiz Dizi Öğesi Erişimi

Hadi dizinin sonunu aşan bir öğeye erişmeye çalıştığımızda ne olacağına bakalım.
Diyelim ki, 2. bölümdeki tahmin oyununa benzer şekilde, kullanıcıdan bir dizi
indeksi almak için şu kodu çalıştırdınız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Bu kod başarılı bir şekilde derlenir. Eğer bu kodu `cargo run` kullanarak
çalıştırdıysanız, ve `0`, `1`, `2`, `3` ve `4` girdiyseniz, program, bu
indekslerdeki değerleri yazdıracaktır. Eğer bunun yerine dizinin sonunu geçen
bir sayı (`10` gibi) girdiyseniz, şunun gibi bir çıktı göreceksiniz:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Program, indeksleme işleminde geçersiz bir değer kullandığınız noktada bir
*çalışma zamanı* hatası ile sonuçlanmıştır. Program bir hata mesajı ile çıkış
yaptı ve son `println!` ifadesini çalıştırmadı. İndeksleme kullanarak bir öğeye
erişmeye çalıştığınızda, Rust, belirttiğiniz indeksin dizinin uzunluğundan küçük
olduğunu kontrol eder. Eğer indeks, uzunluğa eşit ya da bundan büyükse, Rust
panikleyecektir. Bu kontrol, çalışma zamanında olmalıdır, özellikle bu durumda,
çünkü derleyici, kod sonradan çalıştırılınca kullanıcının hangi değeri
gireceğini bilemez.

Bu, Rust'ın hafıza güvenliği prensiplerinin iş başında olduğu bir örnektir.
Birçok düşük seviye dilde, bu şekilde bir kontrol yapılmaz, ve doğru olmayan bir
indeks verdiğinizde, geçersiz hafızaya erişilebilir. Rust, hafıza erişimine izin
verip devam etmektense, anında çıkış yaparak sizi bu tür hatalardan korur. 9.
bölüm Rust'ın hata ele alması hakkında daha fazlasını ve hem paniklemeyen hem de
geçersiz hafıza erişimine izin vermeyen okunabilir, güvenli kodu nasıl
yazabileceğinizi tartışacaktır.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#tahmin-ile-gizli-sayıyı-karşılaştırmak
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#kontrol-akışı
[strings]: ch08-02-strings.html#stringler-ile-utf-8-ile-kodlanmış-metinleri-saklamak
[stack-and-heap]: ch04-01-what-is-ownership.html#stack-ve-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
