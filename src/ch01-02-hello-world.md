## Merhaba, Dünya!

Rust'ı kurduğunuza göre ilk Rust programınızı yazmanın zamanı geldi. Yeni bir
dil öğrenirken ekrana `Merhaba, dünya!` yazdıran küçük bir program yazmak bir
gelenektir, bu sebeple biz de aynısını yapacağız!

> Not: Bu kitap komut satırını temel düzeyde bilmenizi gerektirir. Rust,
> editörünüz, araçlarınız ya da kodunuzun nerede bulunduğu hakkında bir talepte
> bulunmaz, yani komut satırı yerine bir entegre geliştirme ortamı (IDE)
> kullanmak isterseniz, gözde IDE'nizi kullanmakta özgürsünüz. Birçok IDE şu
> sıralar bir nebze Rust desteği sağlıyor; ayrıntılar için IDE'nin
> belgelendirmesini kontrol edin. Rust ekibi, `rust-analyzer` ile mükemmel bir
> IDE desteği sağlamaya odaklanmış durumda. Daha fazla ayrıntı için
> [D ekine][devtools] bakın.

### Bir Proje Dizini Oluşturmak

Rust kodunuzu saklamak için bir dizin oluşturarak başlayacaksınız. Rust için
kodunuzun nerede olduğu önemli değildir ama bu kitaptaki alıştırma ve projeler
için, ev dizininizde *projeler* isminde bir dizin oluşturmanızı ve tüm
projelerinizi orada tutmanızı tavsiye ederiz.

Bir terminal açın ve, *projeler* dizinini ve bu dizin içinde “Merhaba, dünya!”
projesi için bir dizin oluşturmak için aşağıdaki komutları girin.

Linux, macOS ve Windows'ta PowerShell için şunları girin:

```console
$ mkdir ~/projeler
$ cd ~/projeler
$ mkdir merhaba_dünya
$ cd merhaba_dünya
```

Windows CMD için şunları girin:

```cmd
> mkdir "%USERPROFILE%\projeler"
> cd /d "%USERPROFILE%\projeler"
> mkdir merhaba_dünya
> cd merhaba_dünya
```

### Bir Rust Programı Yazmak ve Çalıştırmak

Şimdi *main.rs* isminde bir kaynak dosyası oluşturun. Rust dosyaları her zaman
*.rs* eklentisi ile biter. Eğer dosya adınızda birden fazla kelime
kullanıyorsanız, gelenek bunları ayırmak için alt çizgi kullanmaktır. Örneğin,
*merhabadünya.rs* yerine *merhaba_dünya.rs* kullanın.

Şimdi henüz oluşturduğunuz *main.rs* dosyasını açın ve Listing 1-1'deki kodu girin.

<Listing number="1-1" file-name="main.rs" caption="`Merhaba, dünya!` yazdıran bir program">

```rust
fn main() {
    println!("Merhaba, dünya!");
}
```

</Listing>

Dosyayı kaydedin *~/projeler/merhaba_dünya* dizinindeki terminalinize geri
dönün. Linux ya da macOS'ta, dosyayı derleyip çalıştırmak için aşağıdaki
komutları girin:

```console
$ rustc main.rs
$ ./main
Merhaba, dünya!
```

Windows'ta `./main` yerine `.\main.exe` komutunu girin:

```powershell
> rustc main.rs
> .\main.exe
Merhaba, dünya!
```

İşletim sisteminizden bağımsız olarak, `Hello, world!` string'i terminale
yazdırılmalıdır. Eğer bu çıktıyı göremiyorsanız, yardım bulmanın yolları için
Kurulum kısmının [“Sorun Giderme”][troubleshooting] bölümüne geri dönün.

Eğer `Merhaba, dünya!` yazdırıldıysa, tebrikler! Resmi olarak bir Rust programı
yazdınız. Bu da sizi bir Rust programcısı yapar; hoş geldiniz!

### Bir Rust Programının Anatomisi

Hadi şu “Merhaba, dünya!” programını ayrıntılı bir şekilde inceleyelim.
Yapbozun ilk parçası şu:

```rust
fn main() {

}
```

Bu satırlar `main` isminde bir fonksiyon tanımlar. `main` fonksiyonu özeldir:
çalıştırılabilir her Rust programında çalışan ilk kod her zaman budur. Burada
ilk satır, parametresi olmayan ve hiçbir şey döndürmeyen `main` isminde bir
fonksiyon tanımlar. Eğer parametreler olsaydı, bunlar `()` ayraçları içine
girecekti.

Fonksiyon gövdesi, `{}` ile çevrilidir. Rust, süslü ayraçların tüm fonksiyon
gövdelerinin etrafında olmasını gerektirir. İkisi arasına bir boşluk ekleyerek
açan süslü ayracı fonksiyon tanımı ile aynı satıra koymak iyi bir tarzdır.

> Not: Eğer Rust projelerinizde standart stile bağlı kalmak isterseniz,
> kodunuzu belirli bir stilde biçimlendirmek için `rustfmt` isimli bir
> otomatik biçimlendirme aracını kullanabilirsiniz (`rustfmt` ile ilgili daha
> fazla bilgiyi [D ekinde][devtools] bulabilirsiniz). Rust ekibi bu aracı
> standart Rust dağıtımına dahil etmiştir, tıpkı `rustc`'yi dahil ettiği gibi,
> yani bilgisayarınızda zaten kurulu olmalı!

`main` fonksiyonunun gövdesi aşağıdaki kodu tutar:

```rust
    println!("Merhaba, dünya!");
```

Bu satır, bu küçük programdaki tüm işi yapar: ekrana yazı yazdırır. Burada
dikkat edilmesi gereken dört önemli ayrıntı vardır.

İlk olarak, Rust stili dört boşluk ile girintilemektir, bir sekme ile değil.

İkincisi, `println!` bir Rust makrosu çağırmaktadır. Eğer bir fonksiyon
çağırsaydı, `println` (`!` olmadan) şeklinde yazılması gerekirdi. Rust
makrolarını 19. bölümde daha ayrıntılı bir şekilde işleyeceğiz. Şimdilik sadece
`!` kullanmanın normal bir fonksiyon yerine bir makro çağrılmasına neden
olacağını ve bu makroların her zaman fonksiyonlar ile aynı kuralları takip
etmediğini bilin.

Üçüncüsü, `"Merhaba, dünya!"` string'ini görüyorsunuz. Bu string'i bir argüman
olarak `println!`'e geçirdik ve bu string ekrana yazdırıldı.

Dördüncüsü, satırı bir noktalı virgül (`;`) ile bitirdik. Bu, bu satırın
bittiğini ve sonraki satırın başlamaya hazır olduğunu belirtir. Rust kodundaki
birçok satır bir noktalı virgül ile biter.

### Derleme ve Çalıştırma Farklı Adımlardır

Yenice oluşturulmuş bir programı henüz çalıştırdınız, hadi işlemdeki her adımı
inceleyelim.

Bir Rust programını çalıştırmadan önce, `rustc` komutunu girerek ve kaynak
dosyasının adını bu komuta geçirerek bu programı derlemelisiniz, aynı şöyle:

```console
$ rustc main.rs
```

Eğer bir C ya da C++ arkaplanınız varsa, bunun `gcc` ya da `clang`'e
benzediğini fark edeceksiniz. Başarıyla derledikten sonra Rust, bir ikili
çalıştırılabilir çıktısı verir.

Linux, macOS ve Windows PowerShell'de, çalıştırılabiliri shell'inize `ls`
komutu girerek görebilirsiniz:

```console
$ ls
main  main.rs
```

Linux ve macOS'ta iki dosya göreceksiniz. Windows PowerShell ile, CMD'de
göreceğiniz aynı üç dosyayı göreceksiniz. Windows'ta CMD ile şunları
girmeniz gerekirdi:

```cmd
> dir /B %= /B seçeneği sadece dosya isimlerinin gösterilmesini söyler =%
main.exe
main.pdb
main.rs
```

Bu komut, *.rs* eklentisine sahip kaynak kod dosyasını, çalıştırılabilir
dosyayı (Windows'ta *main.exe* ama diğer tüm platformlarda *main*) ve, Windows
kullanıldığında, *.pdb* eklentisine sahip hata ayıklama bilgilerini içeren bir
dosyayı gösterir. Bundan sonra *main* ya da *main.exe* dosyasını şu şekilde
çalıştırırsınız:

```console
$ ./main # ya da Windows'ta .\main.exe
```

Eğer *main.rs*'iniz “Merhaba, dünya!” programınızsa, bu satır terminale
`Merhaba, dünya!` yazdıracaktır.

Eğer Ruby, Python ya da JavaScript gibi bir dinamik dile daha aşina iseniz,
muhtemelen bir programı derleme ve çalıştırmayı ayrı adımlarda yapmıyordunuz.
Rust bir *vaktinden önce derlenen* dildir, yani bir programı derleyip
çalıştırılabiliri bir başkasına verdiğinizde, bu kişi bu programı Rust kurulu
olmasa da çalıştırabilir. Eğer birisine bir *.rb*, *.py* ya da *.js* dosyası
verseydiniz, bu kişinin sırasıyla bir Ruby, Python ya da JavaScript
gerçeklemesine sahip olması gerekirdi. Fakat bu dillerde, programınızı derlemek
ve çalıştırmak için sadece bir komuta ihtiyacınız vardır. Dil tasarımında her
şey bir takastır.

Basit programlar için `rustc` ile derleyip geçmek uygundur fakat projeniz
büyüdükçe, tüm seçenekleri yönetmek ve kodunuzu paylaşmayı kolaylaştırmak
isteyeceksiniz. Bir sonraki kısımda, size Cargo aracını tanıtacağız. Bu araç
size gerçek dünya Rust programları yazmanızda yardımcı olacak.

[troubleshooting]: ch01-01-installation.html#sorun-giderme
[devtools]: appendix-04-useful-development-tools.html
