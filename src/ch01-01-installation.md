## Kurulum

İlk adım Rust'ı kurmaktır. Rust'ı `rustup` aracılığı ile kuracağız. `rustup`,
Rust sürümlerini ve ilgili araçları yönetmek için bir komut satırı aracıdır.
İndirme için bir internet bağlantısı gerekir.

> Not: Eğer herhangi bir sebepten dolayı `rustup`'ı kullanmak istemezseniz,
> diğer seçenekleri [Diğer Rust Kurulum Metodları sayfasında][otherinstall]
> bulabilirsiniz.

Aşağıdaki adımlar, Rust derleyicisinin en son kararlı sürümünü kurar. Rust'ın
kararlılık garantileri, bu kitaptaki tüm örneklerin daha yeni Rust sürümleri
ile derlenmeye devam edeceğinden emin olmamızı sağlar. Çıktı, sürümler arasında
biraz farklı olabilir çünkü Rust sık sık hata mesajlarını ve uyarıları
iyileştirir. Diğer bir deyişle, bu adımları takip ederek kurduğunuz Rust'ın
daha yeni herhangi bir kararlı sürümü, bu kitabın içeriği ile beklendiği gibi
çalışacaktır.

> ### Komut Satırı Gösterimi
>
> Bu bölümde ve kitap boyunca, terminalde kullanılan bazı komutlar
> göstereceğiz. Terminale girmeniz gereken satırlar `$` ile başlar. Bu
> karakteri sizin girmeniz gerekmez; bu, her komutun başlangıcını işaret etmek
> için gösterilen bir komut satırı istemidir. `$` ile başlamayan satırlar
> genelde bir önceki komutun çıktısını gösterir. Ek olarak, PowerShell özelinde,
> örnekler `$` yerine `>` kullanacaktır.

### `rustup`'ı Linux ya da macOS'ta Kurmak

Eğer Linux ya da macOS kullanıyorsanız, bir terminal açın ve aşağıdaki komutu
girin:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Bu komut bir betik indirecek ve `rustup` aracının kurulumuna başlayacaktır, bu
kurulum Rust'ın en son kararlı sürümünü kuracaktır. Şifrenizi girmeniz
istenebilir. Eğer kurulum başarılı ise, aşağıdaki satır görünecektir:

```text
Rust is installed now. Great!
```

Ayrıca bir *bağlayıcıya* ihtiyacınız olacak. Bağlayıcı Rust'ın, kendi derlenmiş
çıktılarını bir dosyaya birleştirmek için kullandığı bir programdır. Eğer
bağlayıcı hataları alıyorsanız, bir C derleyicisi kurmalısınız, bu derleyici
muhtemelen bir bağlayıcı ile gelecektir. Bir C derleyicisi kullanışlıdır çünkü
bazı yaygın Rust paketleri C koduna bağımlıdır ve bir C derleyicisine ihtiyaç
duyacaklardır.

macOS'ta şu komut ile bir C derleyicisi edinebilirsiniz:

```console
$ xcode-select --install
```

Linux kullanıcıları genellikle GCC ya da Clang'ı kurmalıdır, bunun için
dağıtımlarının belgelendirmesini takip edebilirler. Örneğin, Ubuntu
kullanıyorsanız, `build-essential` paketini kurabilirsiniz.

### `rustup`'ı Windows'ta Kurmak

Windows'ta [https://www.rust-lang.org/tools/install][install] adresine gidin ve
Rust kurmak için talimatları takip edin. Kurulumda bir noktada Visual Studio'yu
kurmanız istenecek. Bu, bir bağlayıcı ve programları derlemek için gerekli
yerel kütüphaneleri sağlar. Bu adım ile ilgili daha fazla yardım isterseniz,
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]
adresine bakın.

Bu kitabın geriye kalanı, hem *cmd.exe*'de hem de PowerShell'de çalışan
komutlar kullanır. Eğer belirli farklılıklar varsa, hangisini kullanmanız
gerektiğini açıklayacağız.

### Sorun Giderme

Rust'ın doğru bir şekilde kurulup kurulmadığını kontrol etmek için, bir shell
açın ve şu satırı girin:

```console
$ rustc --version
```

Sürüm numarası, işleme hash'i ve en son yayınlanan kararlı sürüm için işleme
tarihini şu biçimde görmeniz gerekiyor:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Eğer bu bilgiyi görüyorsanız, Rust'ı başarılı bir şekilde kurmuşsunuzdur. Eğer
görmüyorsanız, Rust'ın `%PATH%` sistem değişkeniniz içinde olup olmadığını
aşağıdaki gibi kontrol edin.

Windows CMD'de şunu kullanın:

```console
> echo %PATH%
```

PowerShell'de şunu kullanın:

```powershell
> echo $env:Path
```

Linux ve macOS'ta şunu kullanın:

```console
$ echo $PATH
```

Eğer bunda bir sorun yoksa ve Rust hala çalışmıyorsa, yardım alabileceğiniz
birkaç yer var. Diğer Rustacean'lar (kendimize koyduğumuz şapşal bir takma ad)
ile nasıl temasa geçeceğinizi [topluluk sayfasından][community] öğrenin.

### Güncelleme ve Kaldırma

Rust bir kez `rustup` ile kuruldu mu, yeni bir sürüme güncellemek basittir.
Shell'inizden aşağıdaki güncelleme betiğini çalıştırın:

```console
$ rustup update
```

Rust'ı ve `rustup`'ı kaldırmak için shell'inizden aşağıdaki kaldırma betiğini
çalıştırın:

```console
$ rustup self uninstall
```

### Yerel Belgelendirme

Rust kurulumu ayrıca belgelendirmenin yerel bir kopyasını içerir, böylece bunu
çevrimdışı okuyabilirsiniz. Yerel belgelendirmeyi tarayıcınızda açmak için
`rustup doc` komutunu verin.

Standart kütüphane tarafından sunulan bir tip ya da fonksiyonun ne yaptığından
ya da bunu nasıl kullanmanız gerektiğinden emin olmadığınız her zaman, cevabı
bulmak için uygulama programlama arayüzü belgelendirmesini kullanın!

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
