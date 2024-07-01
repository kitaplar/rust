## Kontrol Akışı

Bir şartın `true` olup olmamasına göre bir kodu çalıştırma yeteneği ve bir koşul
doğru iken bir kodu üst üste çalıştırma, birçok programlama dilinde temel yapı
taşlarıdır. Rust kodunun çalıştırım akışını kontrol etmemizi sağlayan en yaygın
yapı, `if` deyimleri ve döngülerdir.

### `if` Deyimleri

`if` deyimi, koşullara göre kodunuzu dallandırmanızı sağlar. Bir koşul
sağlarsınız ve “Eğer bu koşul karşılanıyorsa, bu kod bloğunu çalıştır. Eğer bu
koşul karşılanmıyorsa, bu kod bloğunu çalıştırma.” dersiniz.

*projeler* dizininizde *dallar* isimli bir proje oluşturun ve `if` deyimini
keşfetmeye başlayalım. *src/main.rs* dosyasına şunu girin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Tüm `if` deyimleri, `if` anahtar kelimesi ile başlar ve bir koşul ile devam
eder. Bu durumda koşul, `sayı` değişkeninin 5'ten küçük bir değerde olup
olmadığını kontrol etmektedir. Koşulun `true` olduğu durumda koşacak kodu,
koşuldan hemen sonra süslü parantezler içine koyduk. `if` deyimlerinde koşullar
ile ilişkilendirilmiş kod blokları bazen *kollar* olarak isimlendirilir, tıpkı
`match` deyimlerindeki kollar gibi, ki bunu 2. bölümün [“Tahmin ile Gizli Sayıyı
Karşılaştırmak”][comparing-the-guess-to-the-secret-number] bölümünde
tartışmıştık.

İsteğe bağlı olarak ayrıca bir `else` deyimi ekleyebiliriz, aynı burada
yaptığımız gibi. Böylece koşul `false` olduğunda çalıştırılacak alternatif bir
kod bloğu sağlamış oluruz. Eğer bir `else` deyimi vermezseniz ve koşul `false`
ise, program sadece `if` bloğunu boşverecek ve bir sonraki koda geçecektir.

Bu kodu çalıştırmayı deneyin; aşağıdaki gibi bir çıktı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Hadi `sayı`nın değerini, koşulun `false` olacağı bir değere değiştirmeyi
deneyelim ve ne olacağına bakalım:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Programı tekrar koşun ve çıktıya bakın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Bu koddaki koşulun bir `bool` olması *gerektiğine* dikkat edin. Eğer koşul bir
`bool` değilse, bir hata alırız. Örneğin, aşağıdaki kodu koşmayı deneyin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

Bu sefer `if` koşulu `3` değerine değerlenir ve Rust bir hata fırlatır:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Hata, Rust'ın `bool` beklediğini ancak bir tamsayı bulduğunu söylemektedir. Ruby
ve JavaScript gibi dillerin aksine Rust, Boolean olmayan değerleri Boolean'a
otomatik olarak dönüştürmeyi denemeyecektir. Açık olmalısınız ve her zaman
`if`'e koşul olarak bir Boolean vermelisiniz. Eğer `if` kod bloğunun sadece bir
sayı `0`'a eşit olmadığında çalışmasını istiyorsak, `if` deyimini mesela şunun
gibi yapabiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Bu kodu çalıştırmak `sayı sıfırdan başka bir şey` yazdıracaktır.

#### `else if` ile Birden Çok Koşulu Ele Almak

`if` ve `else`'i bir `else if` deyimi içinde birleştirerek, birden çok koşul
kullanabilirsiniz. Mesela:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Bu program, girebileceği dört muhtemel yola sahiptir. Bunu çalıştırdıktan sonra
aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Bu program çalıştığında, her `if` deyimini sırayla kontrol eder ve koşulun
`true` olduğu ilk gövdeyi çalıştırır. 6, 2'ye bölünebilir olsa da, `sayı, 2'ye
bölünebilir` çıktısını görmediğimize dikkat edin. Ayrıca `else` bloğundan `sayı,
4, 3 ya da 2'ye bölünemez` metnini de görmedik. Bu, Rust'ın sadece ilk `true`
koşulundaki bloğu çalıştırması sebebiyledir. Bu koşulu bulduğu zaman, gerisine
bakmaz.

Çok fazla `else if` deyimi kullanmak kodunuzu dağınık yapabilir, bu sebeple eğer
birden fazla `else if` deyiminiz varsa, kodunuzu yeniden düzenlemek
isteyebilirsiniz. 6. bölüm, bu durumlar için `match` isminde güçlü bir
dallandırma yapısını anlatacaktır.

#### `let` İfadesinde `if` Kullanmak

`if` bir deyim olduğu için, bunu bir `let` ifadesini sağ tarafında
kullanabilir, böylece çıktıyı bir değişkene atayabiliriz, Listing 3-2'deki gibi.

<Listing number="3-2" file-name="src/main.rs" caption="`if` deyiminin sonucunu bir değişkene atamak">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

`sayı` değişkeni, `if` deyiminin çıktısına göre bir değere bağlanacaktır. Ne
olacağını görmek için bu kodu çalıştırın:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Kod bloklarının, içlerindeki son deyime değerleneceğini ve sayıların da birer
deyim olduğunu hatırlayın. Bu durumda, tüm `if` deyiminin değeri, hangi kod
bloğunun çalıştığına bağlıdır. Bu, `if`'in her kolundaki sonuç olma potansiyeli
olan değerlerin aynı tipte olması gerektiği anlamına gelir; Listing 3-2'de hem
`if` kolunun hem de `else` kolunun sonucu `i32` tamsayısıydı. Eğer tipler
aşağıdaki gibi eşleşmeseydi, bir hata alacaktık:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Bu kodu derlemeye çalıştığımızda, bir hata alırız. `if` ve `else` kolları,
uyumsuz değer tiplerine sahip ve Rust, problemi tam olarak nerede bulacağımızı
söylüyor:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`if` bloğundaki deyim, bir tamsayıya değerlenmektedir, `else` bloğundaki deyim
ise bir string'e. Bu çalışmayacaktır çünkü değişkenler tek bir tipe sahip
olmalıdır ve Rust, derleme zamanında `number` değişkeninin hangi tipte olacağını
kesin olarak bilmek zorundadır. `number` değişkeninin tipini bilmek,
derleyicinin bu değişkeni kullandığımız her yerde bu tipin geçerli olduğunu
onaylamasına izin verir. Eğer `number` değişkeninin tipi sadece çalışma
zamanında belirlenebilseydi, Rust bunu yapamazdı; eğer derleyici, her değişken
için birden çok varsayımsal tipi takip etmek zorunda kalsaydı, daha karmaşık
olurdu ve kodunuz hakkında daha az garanti verirdi.

### Döngüler ile Tekrarlama

Sıklıkla bir code bloğunu birden çok kez çalıştırmak kullanışlıdır. Bu görev
için, Rust birçok *döngü* sağlar, bunlar döngü gövdesi içindeki kodu sonuna
kadar çalıştırır ve hemen sonra başa dönerek tekrar başlarlar. Döngüleri
deneyimlemek için, *döngüler* isimli bir proje oluşturalım.

Rust'ta üç çeşit döngü vardır: `loop`, `while` ve `for`. Hadi her birini
deneyelim.

#### `loop` ile Kodu Tekrarlama

`loop` anahtar kelimesi Rust'a bir kod bloğunu sonsuza kadar ya da siz durmasını
söyleyene kadar tekrar tekrar çalıştırmasını söyler.

Bir örnek olarak, *döngüler* dizininizdeki *src/main.rs* dosyasını, şu şekilde
değiştirin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Bu programı çalıştırdığınızda, `tekrar!` satırlarının tekrar tekrar
yazdırıldığını göreceksiniz, ta ki programı elle durdurana kadar. Çoğu terminal,
süreğen bir döngüde takılı kalmış bir programı kesmek için
<kbd>ctrl</kbd>-<kbd>c</kbd> klavye kısayolunu destekler. Bir deneyelim:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling döngüler v0.1.0 (file:///projects/döngüler)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/döngüler`
tekrar!
tekrar!
tekrar!
tekrar!
^Ctekrar!
```

`^C` sembolü nerede <kbd>ctrl</kbd>-<kbd>c</kbd>'ye bastığınızı ifade eder.
`^C`'den sonra `tekrar!` sözcüğünü görebilirsiniz ya da görmeyebilirsiniz. Bu,
kesme sinyalini aldığında kodun döngünün neresinde olduğuna bağlıdır.

İyi ki Rust ayrıca, kod kullanarak bir döngüyü kırmak için bir yol sağlar.
Programa döngüyü ne zaman durduracağını söylemek için döngü içine `break`'i
yerleştirebilirsiniz. Bunu 2. bölümün [“Doğru bir Tahminden Sonra
Çıkma”][quitting-after-a-correct-guess] bölümünde, tahmin oyununda zaten
yaptığımızı hatırlayın, böylece kullanıcı doğru sayıyı tahmin edip oyunu
kazandığında, oyundan çıkabilmişti.

Tahmin oyununda ayrıca `continue`'yu kullandık. `continue` döngülerde o anki
yinelemede kalan kodu atlamayı ve bir sonraki yinelemeye geçmeyi söyler.

#### Döngülerden Değerler Döndürmek

`loop`'un bir diğer kullanımı, başarısız olabileceğini bildiğiniz bir işlemi
tekrarlamaktır, örneğin bir thread'in işini bitirip bitirmediğini kontrol etmek
gibi. Ayrıca bu işlemin sonucunu döngünün dışına almaya da ihtiyacınız olabilir.
Bunu yapmak için döndürülmesini istediğiniz değeri `break` deyiminden sonra
ekleyebilirsiniz; bu değer döngüden döndürülecektir yani bunu kodunuzda
kullanabilirsiniz:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Döngüden önce, `sayaç` isminde bir değişken bildirip değerini `0` yaptık. Sonra
döngüden dönen değeri tutmak için `sonuç` isimli bir değişken bildirdik.
Döngünün her yinelemesinde, `sayaç` değişkenine `1` ekledik ve sonra `sayaç`,
`10`'a eşit mi diye kontrol ettik. Eşit olduğunda, `break` anahtar kelimesini
'sayaç * 2' değeri ile kullandık. Döngüden sonra, bu değeri `sonuç`'a atayan
ifadeyi sona erdirmek için noktalı virgül kullandık. Son olarak `sonuç`'taki
değeri yazdırdık, ki burada `20`.

Ayrıca döngünün içinden de `return` edebilirsiniz. `break` sadece mevcut
döngüden çıksa da, `return` her zaman mevcut fonksiyondan çıkar.

#### Birden Fazla Döngü Arasında Belirsizliği Gidermek için Döngü Etiketleri

Eğer döngü içinde döngüleriniz varsa, `break` ve `continue` sadece bulundukları
döngü için etkili olurlar. İsteğe bağlı olarak bir döngü için bir `döngü
etiketi` belirtebilirsiniz. Daha sonra bu etiketi `break` ya da `continue` ile
kullanarak, bunların ilgili döngüye etki etmesini sağlayabilirsiniz. Döngü
etiketleri, tek tırnak ile başlamalıdır. İşte iki iç içe döngüden oluşan bir
örnek:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Dıştaki döngü `'yukarı_sayma` etiketine sahiptir ve 0'dan 2'ye kadar
saymaktadır. Etiketsiz olan içteki döngü, 10'dan 9'a kadar saymaktadır. Etiketi
olmayan ilk `break` sadece içteki döngüden çıkacaktır. `break 'yukarı_sayma;`
ifadesi, dıştaki döngüden çıkacaktır. Bu kod şunu yazdırır:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### `while` ile Koşulsal Döngüler

Bir program genellikle bir döngü içindeki bir koşulu değerlendirmeye ihtiyaç
duyar. Koşul, `true` iken döngü çalışır. Ne zaman ki koşul `true` olmayı
bıraktı, program `break`'i çağırır ve döngüyü durdurur. Bunun gibi bir
davranışı, `loop`, `if`, `else` ve `break` kullanarak gerçeklemek mümkündür;
isterseniz bunu şimdi bir programda deneyebilirsiniz. Fakat bu desen o kadar
yaygındır ki, Rust bunun için yerleşik bir dil yapısına sahiptir: `while`
döngüsü. Listing 3-3'te programı üç kez döngüye sokmak için `while` döngüsünü
kullanıyoruz, her defasında bir aşağı sayıyor ve döngüden sonra, bir mesaj
yazdırıp çıkıyoruz.

<Listing number="3-3" file-name="src/main.rs" caption="Bir koşul true iken kod koşmak için `while` kullanmak">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

Bu yapı, `loop`, `if`, `else` ve `break` kullanarak aynısını yapmak isteseydiniz
gerekli olacak birçok yuvalamadan sizi kurtarır ve daha açıktır. Bir koşul
`true`'ya değerlendiği sürece kod çalışır, diğer türlü döngüden çıkar.

#### `for` ile bir Koleksiyon Üzerinde Döngüleme

`while` yapısını, bir koleksiyonun öğeleri üzerinde dönmek için de
kullanabilirsiniz, bir dizi gibi. Örneğin, Listing 3-4'teki döngü, `d`
dizisindeki her öğeyi yazdırır.

<Listing number="3-4" file-name="src/main.rs" caption="`while` döngüsü kullanarak bir koleksiyonun öğeleri üzerinde dönme">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

Burada kod, dizideki öğeler boyunca yukarı doğru sayar. `0` indeksinden başlar,
sonra dizideki son indekse erişene kadar döner. Bu, `indeks < 5`'in artık `true`
olmadığı zamandır. Bu kodu çalıştırmak dizideki her öğeyi yazdıracaktır.

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Dizinin tüm değerleri terminalde görünür, beklendiği gibi. `indeks` bir noktada
`5`'e ulaşacak olsa da döngü, diziden altıncı öğeyi çekmeden önce çalışmayı
bırakacaktır.

Fakat bu yaklaşım hataya açıktır; indeks değeri ya da test koşulu yanlış
olsaydı, programın paniklemesine neden olabilirdik. Örneğin, `d` dizisini, dört
öğeye sahip olacak şekilde değiştirirseniz ancak `while indeks < 4` koşulunu
güncellemeyi unutursanız, kod panikleyecektir. Bu ayrıca yavaş, çünkü derleyici,
döngü boyunca her yinelemede indeksin dizinin sınırları içinde olup olmadığını
kontrol etmek için çalışma zamanı kodu ekleyecektir.

Daha özlü bir alternatif olarak, `for` döngüsünü de kullanabilirsiniz. Bu döngü
ile bir koleksiyonun her öğesi için birtakım kodlar çalıştırabilirsiniz. Bir
`for` döngüsü, Listing 3-5'teki kod gibi görünür.

<Listing number="3-5" file-name="src/main.rs" caption="`for` döngüsü kullanarak bir koleksiyonun her öğesi üzerinde dönmek">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

Bu kodu koştuğumuzda, Listing 3-4'teki ile aynı çıktıyı görürüz. Daha önemlisi,
şimdi kodun güvenliğini artırdık ve dizinin daha ötesine geçip hatayla
karşılaşmaktan da, ya da yeterince ilerlemeyip bazı öğeleri kaçırmaktan da
kurtulduk.

`for` döngüsü kullanarak, eğer dizideki değerlerin sayısını değiştirirseniz,
herhangi bir diğer kodu değiştirmeyi hatırlamanıza gerek kalmaz, ama Listing
3-4'te kullanılan yöntemde bunu hatırlamanız gerekirdi.

`for` döngülerinin güvenli ve kısa oluşu, bu döngülerin Rust'ta en çok
kullanılan döngüler olmasını sağlamıştır. Bir kodu belirli kere çalıştırmak
istediğinizde, mesela Listing 3-3'te `while` döngüsü kullanılarak yapılan aşağı
sayma örneğindeki gibi, çoğu Rustacean bir `for` döngüsü kullanırdı. Bunu
yapmanın yolu, standart kütüphane tarafından sağlanan bir `Range` kullanmaktır.
Bu, bir sayıdan başlayan ve bir diğer sayıdan önce biten tüm sayıları sırasıyla
üretir.

İşte bir aşağı sayma örneğinin `for` döngüsü kullanarak nasıl görüneceği.
Buradaki `rev` metodu hakkında daha önce konuşmadık; kısaca aralığı tersine
çevirir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Bu kod daha hoş, değil mi?

## Özet

Başardınız! Bu, nispeten geniş bir bölümdü: değişkenler, skaler ve bileşik
tipler, fonksiyonlar, yorumlar, `if` deyimleri ve döngüleri öğrendiniz! Bu
bölümde tartışılan kavramları uygulamak için, şunları yapan programlar inşa
etmeyi deneyin:

* Sıcaklıkları Fahrenhayt ile Santigrat arasında çevirin.
* `n`'inci Fibonacci sayısını üretin.
* Barış Manço'nun “Arkadaşım Eşşek” şarkısının sözlerini yazdırın, şarkıdaki
  tekrarlar için döngü kullanmayı deneyin.

İlerlemeye hazır olduğunuzda, diğer programlama dillerinde yaygın olarak var
*olmayan* bir kavram ile ilgili konuşacağız: sahiplik.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#tahmin-ile-gizli-sayıyı-karşılaştırmak
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#doğru-bir-tahminden-sonra-çıkma
