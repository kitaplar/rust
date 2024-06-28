## Merhaba, Cargo!

Cargo, Rust'ın inşa sistemi ve paket yöneticisidir. Çoğu Rustacean, Rust
projelerini yönetmek için bu aracı kullanır çünkü Cargo sizin için birçok
görevi ele alır, mesela kodunuzu inşa etme, kodunuzun bağımlı olduğu
kütüphaneleri indirme ve bu kütüphaneleri inşa etme. (Kodunuzun bağımlı olduğu
kütüphanelere *bağımlılıklar* diyoruz.)

En basit Rust programlarının, mesela şimdiye kadar yazdığımız gibi, herhangi bir
bağımlılığı yoktur. Eğer “Merhaba, dünya!” projesini Cargo ile inşa etseydik,
sadece Cargo'nun kodu inşa etme yeteneğini kullanacaktık. Daha karmaşık Rust
programları yazdıkça, bağımlılıklar ekleyeceksiniz, ve eğer bir projeyi Cargo
ile başlatırsanız, bağımlılıklar eklemek daha kolay olacaktır.

Rust projelerinin büyük çoğunluğu Cargo kullandığı için, bu kitabın geriye
kalanı sizin de Cargo kullandığınızı varsaymaktadır. Eğer
[“Kurulum”][installation] kısmında belirtilen resmi kurucuları kullandıysanız,
Cargo Rust ile birlikte kurulmuştur. Eğer Rust'ı başka bir şekilde kurduysanız,
Cargo'nun kurulu olup olmadığını terminale şunu girerek kontrol edin:

```console
$ cargo --version
```

Eğer bir sürüm numarası görüyorsanız, kurulu demektir! Eğer `command not found`
gibi bir hata görüyorsanız, Cargo'yu ayrı olarak nasıl kuracağınızı anlamak
için kurulum yönteminizin belgelendirmesine bakın.

### Cargo ile bir Proje Oluşturmak

Hadi Cargo kullanarak bir proje oluşturalım ve bu projenin orijinal “Merhaba,
dünya!”'dan nasıl farklılaştığına bakalım. *projeler* dizinize (ya da kodunuzu
nerede saklamaya karar verdiyseniz, oraya) geri gidin. Sonra, herhangi bir
işletim sisteminde, aşağıdakini çalıştırın:

```console
$ cargo new merhaba_cargo
$ cd merhaba_cargo
```

İlk komut bir dizin ve *merhaba_cargo* isimli bir proje oluşturur. Projemizi
*merhaba_cargo* olarak isimlendirdik ve Cargo projenin dosyalarını aynı isimli
bir dizin içinde oluşturdu.

*merhaba_cargo* dizinine gidin ve dosyaları listeleyin. Cargo'nun bizim için
iki dosya ve bir dizin oluşturduğunu göreceksiniz: *Cargo.toml* dosyası ve
içinde *main.rs* dosyası olan bir *src* dizini.

Cargo ayrıca bir *.gitignore* dosyası ile yeni bir Git deposu ilklendirdi. Eğer
var olan bir Git deposunun içinde `cargo new` komutunu verirseniz, Git
dosyaları oluşturulmaz; bu davranışı `cargo new --vcs=git` komutu ile geçersiz
kılabilirsiniz.

> Not: Git, yaygın bir sürüm kontrol sistemidir. `--vcs` bayrağını kullanarak
> `cargo new`'i farklı bir sürüm kontrol sistemi kullanacak ya da hiçbir sürüm
> kontrol sistemi kullanmayacak şekilde değiştirebilirsiniz. Mevcut seçenekleri
> görmek için `cargo new --help` komutunu çalıştırın.

*Cargo.toml*'u editörünüzde açın. Listing 1-2'dekine benzer şekilde görünmelidir.

<Listing number="1-2" file-name="Cargo.toml" caption="`cargo new` tarafından oluşturulan *Cargo.toml*'un içeriği">

```toml
[package]
name = "merhaba_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

</Listing>

Bu dosya [*TOML*][toml] (*Tom’s Obvious, Minimal Language* (Tom'un Aşikar,
Asgari Dili)) biçimindedir, bu biçim Cargo'nun yapılandırma biçimidir.

İlk satır, yani `[package]`, altındaki ifadelerin bir paketi yapılandırdığını
işaret eden bir bölüm başlığıdır. Bu dosyaya daha fazla bilgi ekledikçe, diğer
bölümleri de ekleyeceğiz.

Sonraki üç satır, Cargo'nun programınızı derlemek için ihtiyaç duyduğu
yapılandırma bilgilerini ayarlar: isim, sürüm ve kullanılacak Rust baskısı.
[E ekinde][appendix-e] `baskı` anahtarı hakkında konuşacağız.

Son satır, yani `[dependencies]`, projenizin bağımlılıklarını listelemeniz için
yeni bir bölümün başlangıcıdır. Rust'ta, kod paketlerine *crate* denir. Bu
proje için başka bir crate'e ihtiyacımız yok ancak 2. bölümdeki ilk projede
ihtiyacımız olacak, işte o zaman bu bağımlılıklar bölümünü kullanacağız.

Şimdi *src/main.rs*'i açın ve bir bakın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo sizin için bir “Hello, world!” programı oluşturmuş, tıpkı Listing
1-1'de yazdığımız gibi! Şimdiye kadar projemiz ve Cargo'nun oluşturduğu
arasındaki farklar, Cargo'nun kodu *src* dizinine koyması ve ana dizinde
*Cargo.toml* yapılandırma dosyasına sahip olmamızdır.

Cargo, kaynak dosyalarınızın *src* dizininde yaşamasını bekler. Ana seviye
proje dizini sadece README dosyaları, lisans bilgisi, yapılandırma dosyaları ve
kodunuzla ilgili olmayan diğer şeyler içindir. Cargo kullanmak, projelerinizi
düzenlemenize yardımcı olur. Her şey için bir yer vardır ve her şey kendi
yerindedir.

Eğer Cargo kullanmayan bir proje başlatmışsanız, “Merhaba, dünya!” projesinde
yaptığımız gibi bu projeyi Cargo kullanan bir projeye çevirebilirsiniz. Proje
kodunu *src* dizini altına taşıyın ve uygun bir *Cargo.toml* dosyası oluşturun.

### Bir Cargo Projesini İnşa Etmek ve Çalıştırmak

Şimdi de “Hello, world!” programını Cargo ile inşa edip çalıştırdığımızda
neyin farklı olduğuna bakalım! *merhaba_cargo* dizininizde aşağıdaki komutu
vererek projenizi inşa edin:

```console
$ cargo build
   Compiling merhaba_cargo v0.1.0 (file:///projects/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Bu komut, şu anki dizininizde değil de *target/debug/merhaba_cargo* yolunda (ya
da Windows'ta *target/debug/merhaba_cargo.exe* yolunda) çalıştırılabilir bir
dosya oluşturur. Varsayılan inşa, hata ayıklama inşası olduğu için, Cargo,
ikiliyi *debug* isimli bir dizine koyar. Çalıştırılabiliri şu komut ile
koşabilirsiniz:

```console
$ ./target/debug/merhaba_cargo # ya da Windows'ta .\target\debug\merhaba_cargo.exe
Hello, world!
```

Eğer her şey iyi giderse, terminale `Hello, world!` yazdırılmalıdır.
`cargo build`'ı ilk kez çalıştırmak Cargo'nun ayrıca ana dizinde yeni bir dosya
oluşturmasına neden olur: *Cargo.lock*. Bu dosya, projenizdeki bağımlılıkların
tam sürümlerinin takibini yapar. Bu projenin herhangi bir bağımlılığı yok, bu
sebeple dosya biraz seyrek. Bu dosyayı elle değiştirmeniz hiçbir zaman
gerekmeyecek; Cargo bu dosyanın içeriğini bizim için yönetir.

`cargo build` ile bir proje inşa ettik ve `./target/debug/merhaba_cargo` ile
çalıştırdık ama ayrıca `cargo run`'ı da, bir komutta kodu derlemek ve sonrasında
oluşan çalıştırılabiliri koşmak için kullanabiliriz.

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/merhaba_cargo`
Hello, world!
```

`cargo run`'ı kullanmak, `cargo build`'ı koşmayı ve sonrasında ikilinin tam
yolunu kullanmayı hatırlamak zorunda kalmaktan daha kullanışlıdır, bu sebeple
çoğu geliştirici `cargo run`'ı kullanır.

Dikkat edin, bu sefer Cargo'nun `merhaba_cargo`'yu derlediğini gösteren bir
çıktı görmedik. Cargo, dosyaların değişmediğini anladı, bu sebeple bunları
yeniden inşa etmedi; sadece ikiliyi çalıştırdı. Eğer kaynak kodunuzu
değiştirmiş olsaydınız, Cargo, projeyi çalıştırmadan önce onu yeniden inşa
edecekti ve şu çıktıyı görecektiniz:

```console
$ cargo run
   Compiling merhaba_cargo v0.1.0 (file:///projects/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/merhaba_cargo`
Hello, world!
```

Cargo ayrıca `cargo check` isminde bir komut da sağlar. Bu komut kodunuzun
derlendiğinden emin olmak için onu hızlıca kontrol eder ama bir
çalıştırılabilir üretmez:

```console
$ cargo check
   Checking merhaba_cargo v0.1.0 (file:///projects/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Neden bir çalıştırılabilir istemeyesiniz ki? Genellikle `cargo check`,
`cargo build`'dan çok daha hızlıdır çünkü bir çalıştırılabilir üretme
adımlarını atlar. Eğer kodunuzu yazarken işinizi sürekli kontrol ediyorsanız,
`cargo check` kullanmak, projenizin hala derlendiğini anlama sürecinizi
hızlandıracaktır! Zaten birçok Rustacean programlarını yazarken, bunların
derlendiğinden emin olmak için belirli aralıklarda `cargo check`'i çalıştırır.
Sonrasında çalıştırılabiliri kullanmaya hazır olduklarında `cargo build`'ı
koşarlar.

Hadi şimdiye kadar Cargo hakkında öğrendiklerimizin üzerinden geçelim:

* `cargo new` kullanarak bir proje oluşturabiliriz.
* `cargo build` kullanarak bir proje inşa edebiliriz.
* `cargo run` kullanarak bir projeyi tek adımda inşa edip koşabiliriz.
* `cargo check` kullanarak bir ikili oluşturmadan hataları kontrol etmek için
  projeyi inşa edebiliriz.
* İnşanın sonucunu kodumuz ile aynı dizine kaydetmek yerine, Cargo, bunu
  *target/debug* dizini altına kaydeder.

Cargo kullanmanın bir diğer avantajı, hangi işletim sisteminde çalışıyor
olursanız olun, komutların aynı olmasıdır. Bu sebeple bundan sonra artık Linux
ve macOS'a karşı Windows için özel talimatlar sağlamayacağız.

### Yayın için İnşa Etme

Projeniz sonunda yayın için hazır olduğunda, optimizasyonlarla birlikte
derlemek için `cargo build --release` komutunu kullanabilirsiniz. Bu komut
*target/debug* yerine *target/release* içinde bir çalıştırılabilir
oluşturacaktır. Optimizasyonlar Rust kodunuzun daha hızlı koşmasını sağlar
fakat derleme zamanını artırır. Bu sebeple iki farklı profil vardır: biri
geliştirme için, ki hızlıca yeniden inşa etmek istediğinizde kullanırsınız,
diğeri ise bir kullanıcıya vereceğiniz son programı inşa etmek için, ki bu
durumda tekrar inşa etmenize gerek yoktur ve programınızın olabildiğince hızlı
çalışmasını istersiniz. Eğer kodunuzun koşma zamanını kıyaslıyorsanız,
`cargo build --release` komutunu koştuğunuzdan ve *target/release* altındaki
çalıştırılabiliri kıyasladığınızdan emin olun.

### Ortak Düşünce olarak Cargo

Basit projeler için Cargo, sadece `rustc`'yi kullanmaya göre çok fazla bir
değer sağlamaz, fakat programlar daha karmaşık hale geldikçe değerini kanıtlar.
Bir kez programlar birden çok dosya haline geldiğinde ya da bir bağımlılığa
ihtiyaç duyduğunda, Cargo'nun inşayı koordine etmesine izin vermek çok daha
kolaydır.

`merhaba_cargo` projesi basit olsa da, Rust kariyeriniz boyunca kullanacağınız
gerçek araçların çoğunu kullanıyor. Aslında herhangi bir var olan proje
üzerinde çalışmak için, Git kullanarak kodu çeken, bu projenin dizinine geçen
ve inşa eden şu komutları kullanabilirsiniz:

```console
$ git clone example.org/bir_proje
$ cd bir_proje
$ cargo build
```

Cargo hakkında daha fazla bilgi için, [belgelendirmesine][cargo] bakın.

## Özet

Rust maceranıza harika bir başlangıç yaptınız! Bu bölümde şunları nasıl
yapacağınızı öğrendiniz:

* `rustup` kullanarak Rust'ın en son kararlı sürümünü kurma
* Daha yeni bir Rust sürümüne güncelleme
* Yerel olarak kurulu belgelendirmeyi açma
* Doğrudan `rustc` kullanarak bir “Merhaba, dünya!” programı yazıp koşmak
* Cargo'nun düzenlerini kullanarak bir proje oluşturup koşmak

Şimdi Rust kodu okuyup yazmaya alışmak için daha karmaşık bir program inşa
etmek için çok iyi bir zaman. Bu sebeple, 2. bölümde bir tahmin oyunu programı
inşa edeceğiz. Eğer bunun yerine Rust'ta yaygın programlama kavramlarının nasıl
çalıştığını öğrenmeye başlamak istiyorsanız, 3. bölüme bakın ve daha sonra 2.
bölüme geri dönün.

[installation]: ch01-01-installation.html#kurulum
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
