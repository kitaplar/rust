## Fonksiyonlar

Fonksiyonlar Rust kodunda yaygındır. Dildeki en önemli fonksiyonlardan birini
zaten gördünüz: `main` fonksiyonu, ki birçok programın giriş noktasıdır. Ayrıca
`fn` anahtar kelimesini de gördünüz, bu anahtar kelime, yeni fonksiyonlar
bildirmenize olanak sağlar.

Rust kodu, fonksiyon ve değişken isimleri için geleneksel olarak *yılan harf*
kullanır. Bu adette, tüm harfler küçüktür ve altçizgi, kelimeleri ayırır. İşte
örnek bir fonksiyon tanımı içeren bir program:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Rust'ta bir fonksiyonu, `fn` ve takip eden bir fonksiyon adı ve bir parantez
ikilisi ile tanımlarız. Süslü parantezler derleyiciye, fonksiyon gövdesinin
nerede başlayıp nerede bittiğini söyler.

Tanımladığımız herhangi bir fonksiyonu, bu fonksiyonun ismini ve bir parantez
ikilisi girerek çağırabiliriz. `bir_diğer_fonksiyon`, programda tanımlandığı
için, `main` fonksiyonu içinde çağrılabilir. `bir_diğer_fonksiyon`u, `main`
fonksiyonundan *sonra* tanımladığımıza dikkat edin; önce de tanımlayabilirdik.
Rust, fonksiyonları nerede tanımladığınıza pek bakmaz, sadece çağırıcı
tarafından görülebilen kapsamın bir yerinde tanımlı olup olmadığına bakar.

Hadi *fonksiyonlar* isminde yeni bir ikili proje başlatalım böylece
fonksiyonları daha fazla keşfedelim. `bir_diğer_fonksiyon` örneğini,
*src/main.rs*'ye yerleştirin ve çalıştırın. Aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Satırlar, `main` fonksiyonunda görünen sırada çalışır. Önce “Merhaba, dünya!”
mesajı yazdırılır, sonra da `bir_diğer_fonksiyon` çağrılır ve bu fonksiyonun
mesajı yazdırılır.

### Parametreler

Fonksiyonları *parametreler*e sahip olacakları şekilde tanımlayabiliriz.
Parametreler, fonksiyonun imzasının bir kısmı olan özel değişkenlerdir. Bir
fonksiyonun parametreleri olduğunda, bu fonksiyona bu parametreler için somut
değerler sağlayabilirsiniz. Teknik olarak, somut değerler, *argümanlar* olarak
isimlendirilir fakat günlük konuşmada, insanlar bir fonksiyonun tanımındaki
parametreler için de, bir fonksiyonu çağırdığınızda geçirilen somut değerler
için de *parametre* ve *argüman*ı birbirleri yerine kullanmaya eğilimlidirler.

`bir_diğer_fonksiyon`un bu sürümünde, bir parametre ekliyoruz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Bu programı çalıştırmayı deneyin; şu çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

`bir_diğer_fonksiyon`un tanımında bir parametre var: `x`. `x`'in tipi `i32`
olarak belirtilmiş. `bir_diğer_fonksiyon`a `5` geçirdiğimiz zaman, `println!`
makrosu, biçimlendirme string'inde süslü parantez ikilisinin `x` içerdiği yere
`5` koyar.

Fonksiyon imzalarında, her parametrenin tipini bildirmek *zorundasınız*. Bu,
Rust'ın tasarımında kasıtlı bir karardır: fonksiyon tanımlarında tip
bildirimlerini zorunlu kılmak, derleyicinin, sizin hangi tipi kastettiğinizi
anlaması için neredeyse hiçbir zaman bunları başka yerde kullanmanıza ihtiyacı
olmadığı anlamına gelir. Ayrıca derleyici, fonksiyonun beklediği tipi bilirse,
daha yardımcı hata mesajları gösterebilir.

Birden çok parametre tanımlayacağınızda, parametre bildirimlerini şunun gibi
virgüller ile ayırın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Bu örnek, `etiketli_ölçüyü_yazdır` isminde bir fonksiyon oluşturur ve bu
fonksiyon iki parametre alır. İlk parametrenin adı `değer`dir ve tipi `i32`'dir.
İkincisinin adı `birim_etiketi` ve tipi `char`'dır. Fonksiyon sonra hem `değer`i
hem de `birim_etiketi`ni içeren metni yazdırır.

Hadi bu kodu çalıştırmayı deneyelim. Şu anda *fonksiyonlar* projenizin
*src/main.rs* dosyasında yer alan programı, bir önceki örnek ile değiştirin ve
`cargo run` kullanarak çalıştırın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Fonksiyonu, `değer` için `5` değeri ve `birim_etiketi` için `'s'` değeri ile
çağırdığımız için, program çıktısı bu değerleri içermektedir.

### İfadeler ve Deyimler

Fonksiyon gövdeleri, isteğe bağlı olarak bir deyim ile biten bir kısım
ifadelerden oluşur. Şimdiye kadar üzerinde durduğumuz fonksiyonlar, sonda bir
deyime sahip değillerdi, fakat bir ifadenin bir parçası olarak bir deyim
gördünüz. Rust, deyim temelli bir dil olduğu için bu, önemli bir ayrımdır. Diğer
diller aynı ayrıma sahip olmayabilir, öyleyse hadi ifade ve deyimlerin ne
olduğuna ve bunların farklarının fonksiyonların gövdelerini nasıl etkilediğine
bir bakalım.

* **İfadeler**, bazı eylemleri yerine getiren ve hiçbir değer döndürmeyen
  talimatlardır.
* **Deyimler**, bir sonuç değerine değerlenir. Hadi bazı örneklere bakalım.

Aslında zaten ifadeleri ve deyimleri kullandık. `let` anahtar kelimesi ile bir
değişken oluşturmak ve bu değişkene bir değer atamak, bir ifadedir. Listing
3-1'de `let y = 6;` bir ifadedir.

<Listing number="3-1" file-name="src/main.rs" caption="Bir ifade içeren bir `main` fonksiyon bildirimi">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

Fonksiyon tanımları ayrıca ifadedir; önceki örneğin tümü bir ifadedir.

İfadeler, değer döndürmez. Bu sebeple bir değişkene bir `let` ifadesi
atayamazsınız. Aşağıdaki kod bunu denediği için bir hata alıyor:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Bu programı çalıştırdığınızda, alacağınız hata şunun gibidir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

`let b = 6` ifadesi bir değer döndürmez, yani `a`ya atanacak herhangi bir şey
yoktur. Bu, diğer dillerde olanlardan farklıdır, mesela C ve Ruby'de. Bu
dillerde atama, atamanın değerini döndürür. Bu dillerde `a = b = 6`
yazabilirsiniz ve hem `a`'nın hem de `b`'nin `6` değerine sahip olmasını
sağlayabilirsiniz; bu, Rust'ta geçerli değildir.

Deyimler bir değere değerlenirler ve Rust'ta yazacağınız kodun çoğunluğunu
oluştururlar. Mesela bir matematik işlemini düşünün, `5 + 6` gibi, ki bu, `11`'e
değerlendirilen bir deyimdir. Deyimler, ifadelerin bir parçası olabilir: Listing
3-1'de, `let b = 6;` ifadesindeki `6`, `6` değerine değerlendirilen bir
deyimdir. Bir fonksiyon çağırmak, bir deyimdir. Bir makro çağırmak bir deyimdir.
Süslü parantezler ile oluşturulan yeni kapsam bloğu bir deyimdir, mesela:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Bu deyim:

This expression:

```rust,ignore
{
    let a = 3;
    a + 1
}
```

bu durumda `4`'e değerlenen bir bloktur. Bu değer, `let` ifadesinden dolayı
`b`'ye bağlanır. `a + 1`'in en sonda bir noktalı virgül içermediğine dikkat
edin, ki bu, şimdiye kadar gördüğünüz satırlardakilerin aksine. Deyimler, sona
erdiren noktalı virgül içermezler. Eğer bir deyimin sonuna bir noktalı virgül
eklerseniz, bunu bir ifadeye dönüştürürsünüz, ve sonra artık bir değer geri
döndürmez. Bunu aklınızda bulundurun çünkü şimdi fonksiyon dönme değerlerini ve
deyimlerini keşfedeceksiniz.

### Dönme Değerleri ile Fonksiyonlar

Fonksiyonlar, kendilerini çağıran koda değerler dönebilir. Dönme değerlerini
isimlendirmiyoruz ancak bunların tiplerini bir oktan (`->`) sonra bildirmeliyiz.
Rust'ta fonksiyonun dönüş değeri, fonksiyon gövdesinin bloğundaki son deyim ile
eş anlamlıdır. `return` anahtar kelimesini kullanarak ve bir değer belirterek
daha erken dönebilirsiniz, fakat birçok fonksiyon son deyimi içsel olarak
döndürür. İşte bir değer döndüren bir fonksiyona örnek:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

`beş` fonksiyonunda, fonksiyon çağrısı, makro, hatta `let` ifadeleri bile yok,
sadece `5` sayısının kendisi var. Bu, Rust'ta gayet geçerli bir fonksiyondur.
Fonksiyonun döndürme tipinin de belirtildiğine dikkat edin, `-> i32` şeklinde.
Bu kodu çalıştırmayı deneyin; çıktı şu şekilde görünecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

`beş`teki `5`, fonksiyonun döndürme değeridir, bu sebeple döndürme tipi
`i32`'dir. Hadi bunu ayrıntılı bir şekilde inceleyelim. İki önemli konu var:
ilki, `let a = beş();`, bir değişkeni ilklendirmek için bir fonksiyonun dönen
değerini kullandığımızı göstermektedir. `beş` fonksiyonu `5` döndürdüğü için, bu
satır şununla aynıdır:

```rust
let a = 5;
```

İkincisi, `beş` fonksiyonunun herhangi bir parametresi yoktur ve döndürme
değerinin tipini tanımlamaktadır, ancak fonksiyonun gövdesi, noktalı virgülsüz,
yalnız bir `5`'tir çünkü bu, döndürmek istediğimiz değere sahip bir deyimdir.

Hadi bir diğer örneğe bakalım:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Bu kodu çalıştırmak `a'nın değeri: 6` yazdıracak. Ancak `a + 1`'in olduğu
satırın sonuna bir noktalı virgül koyarsak, bunu bir deyimden bir ifadeye
çevirmiş oluruz ve bir hata alırız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Bu kodu derlemek aşağıdaki gibi bir hata üretir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

Asıl hata mesajı olan `mismatched types` yani `eşleşmeyen tipler`, bu koddaki
asıl meseleyi ortaya çıkarır. `artı_bir` fonksiyonunun tanımı, bunun bir `i32`
döndürdüğünü söylemektedir ancak ifadeler bir değere değerlendirilmezler, ki bu
da `()` yani birim tip döndürülmesi demektir. Bundan dolayı hiçbir şey
döndürülmez ancak bu, fonksiyon tanımı ile çelişir ve bir hatayla sonuçlanır.
Bu çıktıda Rust, muhtemelen bu sorunun düzeltilmesine yardımcı olacak bir mesaj
sunmaktadır: noktalı virgülü silmeyi önermektedir, ki bu da hatayı
düzeltecektir.
