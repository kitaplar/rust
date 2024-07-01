## Değişkenler ve Değişebilirlik

[“Değerleri Değişkenler ile Saklamak”][storing-values-with-variables] bölümünde
bahsedildiği gibi, varsayılan olarak, değişkenler değişemezdir. Bu, Rust'ın
sunduğu güvenlik ve kolay eşzamanlılık avantajlarını alacak şekilde kod yazmanız
için Rust'ın size verdiği birçok dürtmeden biridir. Fakat değişkenleri
değişebilir yapma seçeneğiniz hala var. Hadi Rust'ın nasıl ve neden
değişemezliği tercih ettiğini ve bazen neden bundan vazgeçmek isteyebileceğinizi
keşfedelim.

Bir değişken değişemez olduğunda, bir değer bir isme bir kez bağlandığında, bu
değeri değiştiremezsiniz. Bunu gözünüzde canlandırmak için,
`cargo new değişkenler` komutunu kullanarak *projeler* dizininizde *değişkenler*
isminde bir proje üretin.

Sonra, *değişkenler* dizininizde *src/main.rs*'i açın ve içindeki kodu aşağıdaki
kod ile değiştirin (henüz bu kod derlenmeyecek):

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Dosyayı kaydedin ve programı `cargo run` kullanarak çalıştırın. Değişemezlik
hatasıyla ilgili bir hata mesajı alacaksınız, şu çıktıda gösterildiği gibi:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Bu örnek, derleyicinin programlarınızdaki hataları bulmanızda size nasıl yardım
edeceğini göstermektedir. Derleyici hataları can sıkıcı olabilir fakat gerçekte
sadece programınızın ne yapmasını istiyorsanız, onu henüz güvenli bir şekilde
yapmadığını söylemeye çalışır; yoksa sizin iyi bir programcı *olmadığınızı*
kastetmez. Deneyimli Rustacean'lar da derleyici hatası alırlar.

`` cannot assign twice to immutable variable `x` `` hata mesajını aldınız çünkü
değişemez `x` değişkenine ikinci bir değer atamaya çalıştınız.

Değişemez olarak belirlenmiş bir değeri değiştirmeye çalıştığımızda derleme
zamanı hataları almamız önemlidir çünkü tam da bu durum böceklere yol açabilir.
Eğer kodumuzun bir kısmı, bir değerin asla değişmeyeceği varsayımı üzerine işler
ve kodumuzun diğer kısmı bu değeri değiştirirse, kodun ilk kısmının yapmak üzere
tasarlandığı şeyi yapmama ihtimali vardır. Olaydan sonra bu tür bir böceğin
nedeninin izini sürmek zor olabilir, özellikle kodun ikinci kısmının değeri
sadece *bazen* değiştirdiği durumda. Rust derleyicisi, bir değerin
değişmeyeceğini ifade ettiğinizde, bu değerin gerçekten de değişmeyeceğini
garanti eder, böylece kendiniz bunu takip etmek zorunda kalmazsınız. Böylece
kodunuzdan sonuç çıkarmak daha kolay olur.

Fakat değişebilirlik de çok kullanışlı olabilir ve kodunuzu yazmayı daha
elverişli yapabilir. Değişkenler varsayılan olarak değişemez olsalar da, [2.
bölümde][storing-values-with-variables] yaptığınız gibi değişkenlerin başına
`mut` ekleyerek, bu değişkenleri değişebilir yapabilirsiniz. `mut` ekleme
ayrıca, kodun diğer kısımlarının bu değişkenin değerini değiştireceğini
belirterek gelecekteki okuyuculara kodun niyetini iletir.

Örneğin, *src/main.rs* dosyasını aşağıdaki gibi değiştirin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Şimdi bu programı çalıştırdığımızda, şunu elde ederiz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

`mut` kullanıldığında, `x`'e bağlanmış değeri `5`'ten `6`'ya değiştirmemize izin
verildi. Sonuç olarak değişebilirliği kullanıp kullanmamaya karar vermek size
bağlı bir durumdur ve bu belirli durumda neyin en temizi olduğunu düşündüğünüze
bağlıdır.

### Sabitler

Değişemez değişkenler gibi, *sabitler* de bir şsme bağlanan ve değişmesine izin
verilmeyen değerlerdir ancak sabitler ve değişkenler arasında birkaç farklılık
vardır.

İlk olarak `mut`'u sabitler ile kullanamazsınız. Sabitler sadece varsayılan
olarak değişemez değildir, her zaman değişemezdir. Sabitleri `let` anahtar
kelimesi yerine `const` anahtar kelimesi kullanarak bildirirsiniz ve değerin
tipi de belirtilmelidir. Tipleri ve tip belirtimlerini sonraki bölümde ([“Veri
Tipleri”][data-types]) inceleyeceğiz yani şu anda ayrıntılar konusunda endişe
etmeyin. Sadece tipi her zaman bildirmeniz gerektiğini bilin.

Sabitler, herhangi bir kapsamda bildirilebilir, global kapsam da dahil, ki bu
durumda, kodunuzun birçok kısmının bilmesi gereken değerler için kullanışlı
olurlar.

Son fark, sabitler sadece sabit deyimlere ayarlanabilir, sadece çalışma
zamanında hesaplanabilir bir değerin sonucuna ayarlanamaz.

İşte örnek bir sabit bildirimi:

```rust
const SANİYE_OLARAK_ÜÇ_SAAT: u32 = 60 * 60 * 3;
```

Sabitin adı `SANİYE_OLARAK_ÜÇ_SAAT`'tir ve değeri, 60'ın (bir dakikadaki saniye
sayısı) 60 (bir saatteki dakika sayısı) ile ve sonra 3 (bu programda saymak
istediğimiz saat sayısı) ile çarpılmasının sonucuna ayarlanır. Rust'ın sabitler
için isimlendirme adeti, kelimeler arasında altçizgiler ile hepsini büyük harf
yapmaktır. Derleyici kısıtlı sayıda işlemi derleme zamanında yapabilmektedir,
böylece anlaması ve doğrulaması daha kolay olan bir şekilde sabitlerin
değerlerini yazabiliriz, doğrudan 10.800'ü atamak yerine. Sabitleri bildirirken
hangi işlemlerin kullanılabildiğini öğrenmek için, [Rust Referansının sabit
değerlendirme bölümüne][const-eval] bakın.

Sabitler bir programın çalıştığı tüm süre boyunca geçerlidir, tabii ki
bildirildikleri kapsam içinde. Bu nitelik, uygulama alanınızda programınızın
birçok kısmının bilmesi gereken değerler için sabitleri kullanışlı yapar,
örneğin bir oyundaki herhangi bir oyuncunun kazanmasına izin verilen maksimum
puan ya da ışığın hızı.

Programınız boyunca kullanılan sabit kodlanmış değerleri, sabitler olarak
isimlendirmek, kodun gelecek sürdürücülerine bu değerin anlamını bildirmek için
kullanışlıdır. Bu ayrıca, eğer sabit kodlanmış değerin gelecekte güncellenmesi
gerekirse, kodunuzda değişmesi gereken sadece bir yer olması konusunda yardımcı
olur.

### Gölgeleme

[2. bölümdeki][comparing-the-guess-to-the-secret-number] oyun kılavuzunda
gördüğümüz gibi, yeni bir değişkeni önceki bir değişken ile aynı isimle
bildirebilirsiniz. Rustacean'lar, ilk değişkenin ikincisi tarafından
*gölgelendiğini* söyler, bu da değişkenin adını kullandığınızda derleyicinin,
ikinci değişkeni göreceği anlamına gelir. Gerçekte ikinci değişken ilkini
gölgede bırakır, yani ya kendisi başka bir değişken tarafından gölgelenene kadar
ya da kapsam sona erene kadar isminin kullanımını kendi üstüne alır. Bir
değişkeni, aşağıdaki gibi aynı ismi kullanarak ve `let` anahtar kelimesinin
kullanımını yineleyerek gölgeleyebilirsiniz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Bu program önce `x`'i `5` değerine bağlar. Sonra `let x =`'i yineleyerek yeni
bir değişken oluşturur, bu değişken için orijinal değeri alır ve buna `1` ekler
yani `x`'in değeri `6` olur. Sonra süslü parantezler ile oluşturulmuş içteki
kapsam içinde, üçüncü `let` ifadesi de `x`'i gölgeler ve yeni bir değişken
oluşturur, bu sefer önceki değeri `2` ile çarpar ve `x`'e 12 değerini verir. Bu
kapsam sona erdiğinde, içteki gölgeleme de sona erer ve `x`, `6` değerine geri
döner. Bu programı çalıştırdığımızda, şu çıktı üretilir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Gölgeleme, bir değişkeni `mut` ile işaretlemekten farklıdır çünkü `let` anahtar
kelimesini kullanmadan yanlışlıkla bu değişkene yeniden atama yapmaya
çalışırsak, bir derleme zamanı hatası alırız. `let` kullanarak, bir değer
üzerinde bazı değişimler yapabiliriz ancak bu dönüşümler tamamlandıktan sonra
değişken hala değişmezdir.

`mut` ve gölgeleme arasındaki bir diğer fark, `let` anahtar kelimesini tekrar
kullanarak etkili bir şekilde yeni bir değişken oluşturduğumuz için, aynı ismi
kullanıp değerin tipini değiştirebiliriz. Örneğin, programımız kullanıcıdan,
boşluk karakterlerini girerek bazı yazılar arasında ne kadar boşluk
istediğini göstermesini istemiş olsun, ve sonrasında bu girdiyi sayı olarak
saklamak isteyelim:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

İlk `boşluklar` değişkeni bir string tipidir ve ikinci `boşluklar` değişkeni bir
sayı tipidir. Yani gölgeleme bizi farklı isimler bulmak zorunda kalmaktan
kurtarır, mesela `boşluklar_str` ve `boşluklar_sayı` gibi; bunun yerine daha
basit `boşluklar` ismini yeniden kullanabiliriz. Fakat eğer bunun için `mut`
kullanmaya çalışırsak, burada gösterildiği gibi bir derleme zamanı hatası
alırız:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

Hata, bir değişkenin tipini değiştirmeye iznimiz olmadığını söylemektedir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Değişkenlerin nasıl çalıştığını keşfettiğimize göre, hadi sahip olabilecekleri
diğer veri tiplerine bakalım.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#tahmin-ile-gizli-sayıyı-karşılaştırmak
[data-types]: ch03-02-data-types.html#veri-tipleri
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#değerleri-değişkenler-ile-saklamak
[const-eval]: https://doc.rust-lang.org/reference/const_eval.html
