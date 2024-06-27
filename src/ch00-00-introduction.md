# Giriş

> Not: Kitabın bu baskısı, [No Starch Yayınlarından][nsp] basılı ve e-kitap
biçiminde mevcut olan [Rust Programlama Dili][nsprust] ile aynıdır.

[nsprust]: https://nostarch.com/rust-programming-language-2nd-edition
[nsp]: https://nostarch.com/

*Rust Programlama Diline* hoş geldiniz, bu kitap Rust hakkında bir giriş
kitabıdır. Rust programlama dili, size daha hızlı ve güvenilir yazılım
yazmanızda yardımcı olur. Yüksek seviye ergonomi ve düşük seviye kontrol
genelde programlama dili tasarımında anlaşmazlığa düşerler; Rust bu çakışmaya
meydan okur. Güçlü teknik kapasite ve muhteşem bir geliştirici deneyimini
dengeleyerek, Rust size düşük seviye ayrıntıları (hafıza kullanımı gibi) kontrol
etme seçeneği verir, geleneksel olarak bu kontrol ile ilişkili tüm o güçlükler
olmadan.

## Rust Kimin İçin

Rust birçok nedenden dolayı birçok insan için idealdir. Gelin en önemli
gruplara birlikte bakalım.

### Geliştirici Ekipleri

Rust, çeşitli seviyelerde sistem programlama bilgisine sahip geliştiricilerden
oluşan geniş ekipler arasındaki işbirliği için üretken bir araç olduğunu
kanıtlıyor. Düşük seviye kod çeşitli ince hatalara yatkındır, bu hatalar diğer
birçok dilde sadece geniş test etme ve deneyimli geliştiriciler tarafından
dikkatli kod gözden geçirmesi ile yakalanabilir. Rust'ta derleyici,
eşzamanlılık hataları da dahil olmak üzere, bu yakalanması zor hatalara sahip
kodu derlemeyi reddederek bir bekçi rolü üstlenir. Derleyici ile birlikte
çalışarak ekip, hataları kovalamak yerine, zamanını programın mantığına
odaklanarak geçirebilir.

Rust ayrıca sistem programlama dünyasına modern geliştirici araçları getirir:

* Cargo, dahili bağımlılık yöneticisi ve inşa aracı, bağımlılıkları eklemeyi,
  derlemeyi ve yönetmeyi acısız ve Rust ekosistemi ile tutarlı kılar.
* Rustfmt biçimlendirme aracı, geliştiriciler arasında tutarlı bir kodlama
  stili olduğundan emin olur.
* rust-analyzer, kod tamamlama ve satır içi hata mesajları için Entegre
  Geliştirme Ortamı entegrasyonunu sağlar.

Rust ekosistemindeki bu ve diğer araçları kullanarak, geliştiriciler sistem
seviyesindeki kodları yazarken üretken olabilirler.

### Öğrenciler

Rust, sistem kavramlarını öğrenmekle ilgilenenler ve öğrenciler içindir. Rust
kullanarak birçok insan işletim sistemi geliştirme gibi konuları öğrendi.
Topluluk çok misafirperverdir ve öğrenci sorularını cevaplamaktan mutlu
olacaktır. Bu kitap gibi çalışmalar ile Rust ekipleri, sistem kavramlarını daha
çok insan için ulaşılabilir yapmak istemektedir, özellikle programlamaya
yeni başlayan kişiler için.

### Şirketler

Küçük-büyük yüzlerce şirket Rust'ı birçok görev için üretim ortamında
kullanmaktadır, bu görevlerden bazıları şunlardır: komut satırı araçları, web
servisleri, DevOps araçları, gömülü cihazlar, ses ve vidyo çözümleme ve
kodlama, kripto paralar, biyoinformatik, arama motorları, Nesnelerin İnterneti
uygulamaları, makine öğrenmesi ve hatta Firefox web tarayıcısının önemli
kısımları.

### Açık Kaynak Geliştiricileri

Rust, Rust programlama dilini, topluluğunu, geliştirici araçlarını ve
kütüphanelerini inşa etmek isteyenler içindir. Rust diline katkıda bulunmanızı
çok isteriz.

### Hız ve Kararlılığa Değer Veren İnsanlar

Rust, bir dilde hız ve kararlılığı arzulayan insanlar içindir. Hız diyerek, hem
Rust kodunun ne kadar hızlı çalıştığını hem de Rust'ın size sağladığı program
yazma hızını ifade etmek istiyoruz. Rust derleyicisinin kontrolleri, özellik
ekleme ve yeniden düzenleme sırasında kararlılıktan emin olur. Bu,
geliştiricilerin genellikle değiştirmeye korktukları, bu kontrollere sahip
olmayan diller ile yazılmış kırılgan eski kodlara zıttır. Sanki elle yazılmış
kadar hızlı düşük seviye koda derlenen yüksek seviye özelliklere yani sıfır
maliyetli soyutlamaya çabalayarak Rust, güvenli kodu ayrıca hızlı kod yapmak
için uğraşır.

Rust dili ayrıca birçok diğer kullanıcıyı da desteklemeyi umar; burada
bahsedilenler sadece bazı büyük hisse sahipleri. Genel olarak Rust'ın en büyük
tutkusu, güvenlik *ve* üretkenliği, hız *ve* ergonomiyi sağlayarak
programcıların onlarca yıldır kabul ettiği çıkmazları elemektir. Rust'a bir
şans verin ve bu dilin seçimlerinin sizin için çalışıp çalışmadığını görün.

## Bu Kitap Kimin İçin

Bu kitap başka bir programlama dilinde kod yazdığınızı varsayar ama hangi dil
olduğu konusunda bir varsayım yapmaz. Bu kaynağı, çeşitli programlama
arkaplanına sahip kişiler için genel olarak erişilebilir kılmak için çabaladık.
Programlamanın *ne olduğu* ya da hakkında nasıl düşünülmesi gerektiği üzerine
uzun uzun konuşmadık. Eğer programlamaya tamamen yeniyseniz, özellikle
programlamaya bir giriş sunan bir kitap okumanız daha mantıklı olacaktır.

## Bu Kitabı Nasıl Kullanmalı

Genel olarak bu kitap, kitabı önden arkaya sıralı bir şekilde okuduğunuzu
varsayar. Sonraki bölümler, önceki bölümlerdeki kavramlar üzerine inşa edilir,
ve önceki bölümler belirli bir konuda ayrıntıya girmemiş fakat sonraki bir
bölümde bu konuyu yeniden gündeme getirecek olabilir.

Bu kitapta iki çeşit bölüm bulacaksınız: kavram bölümleri ve proje bölümleri.
Kavram bölümlerinde, Rust'ın bir yönünü öğreneceksiniz. Proje bölümlerinde,
o zamana kadar öğrendiklerinizi uygulayarak küçük programlar inşa edeceğiz.
2., 12. ve 20. bölümler proje bölümleridir; diğerleri kavram bölümleridir.

Bölüm 1 Rust'ın nasıl kurulacağını, bir “Merhaba, dünya!” programının nasıl
yazılacağını ve Rust'un paket yöneticisi ve inşa aracı olan Cargo'nun nasıl
kullanılacağını açıklar. 2. bölüm Rust'ta bir program yazmaya uygulamalı bir
giriştir, bu bölümde bir tahmin oyunu inşa ediyorsunuz. Bu bölümde kavramları
yüksek seviyede açıkladık ve sonraki bölümlerde ek ayrıntılar sunduk. Eğer
elinizi hemen kirletmek istiyorsanız, 2. bölüm bunun için doğru bölümdür. 3.
bölüm, diğer programlama dillerinin özelliklerine benzer Rust özelliklerini
kapsar, ve 4. bölümde Rust'ın sahiplik sistemini öğreneceksiniz. Eğer bir
sonrakine geçmeden önce her ayrıntıyı öğrenmeyi tercih eden titiz bir öğrenici
iseniz, 2. bölümü es geçip 3. bölüme devam etmek isteyebilirsiniz, böylece
öğrendiğiniz ayrıntıları bir proje üzerinde uygulamak istediğinizde 2. bölüme
dönebilirsiniz.

Bölüm 5 struct ve metodları tartışır, ve 6. bölüm enum'ları, `match`
deyimlerini ve `if let` kontrol akışı yapılarını kapsar. Struct'ları ve
enum'ları Rust'ta özel tipler yapmak için kullanacağız.

Bölüm 7'de, Rust'ın modül sistemini ve, kodunuzu ve kodunuzun genel Uygulama
Programlama Arayüzünü organize etmek için gizlilik kurallarını öğreneceksiniz.
8. bölüm vektörler, stringler ve hash haritaları gibi standart kütüphanenin
sağladığı bazı yaygın koleksiyon veri yapılarını tartışır. 9. bölüm Rust'ın
hata ele alma felsefesi ve tekniklerini araştırır.

Bölüm 10 size birden çok tipe uygulanacak kod tanımlama gücünü verecek olan
genelleyiciler, trait'ler ve yaşam sürelerine dalar. 11. bölüm tamamiyle test
etme ile ilgilidir, Rust ne kadar güvenli olsa da, programınızın mantığının
doğru olduğundan emin olmak için testler gereklidir. 12. bölümde, dosyalardaki
yazılarda arama yapan `grep` komut satırı aracının işlevselliğinin bir kısmını
kendimiz gerçekleyeceğiz. Bunun için, daha önceki bölümlerde tartıştığımız
birçok kavramı kullanacağız.

Bölüm 13 kapama ve yineleyicileri keşfeder: Rust'ın fonksiyonel programlama
dillerinden gelen özellikleri. 14. bölümde Cargo'yu daha derin inceleyeceğiz ve
kütüphanelerinizi diğerleri ile paylaşmanın en iyi yolları hakkında
konuşacağız. 15. bölümde standart kütüphanenin sağladığı ve trait'lerin
işlevselliklerini aktifleştirdiği akıllı pointer'ları konuşacağız.

Bölüm 16'da, eşzamanlı programlamanın farklı modelleri üzerinde duracağız ve
Rust'ın nasıl çoklu thread'lerle programlamayı korkusuzca yapmanıza yardımcı
olduğu konusunda konuşacağız. 17. bölüm, Rust deyimlerini aşina olabileceğiniz
nesne yönelimli programlamayla karşılaştırır.

Bölüm 18, desenler ve desen eşleme ile ilgili bir başvurudur. Bunlar Rust
programlarında fikirleri ifade etmenin güçlü yollarıdır. 19. bölüm, güvensiz
Rust, makrolar, yaşam süreleri, trait'ler, tipler, fonksiyonlar ve kapamalar
hakkında ileri konuların bir açık büfesidir.

Bölüm 20'de, düşük seviye çok thread'li bir web sunucusu gerçekleyeceğimiz bir
projeyi tamamlayacağız!

Son olarak, bazı ekler dil hakkında kullanışlı bilgileri daha başvuru tarzında
içerir. A eki Rust'ın anahtar kelimelerini kapsar, B eki Rust'ın işleçlerini ve
sembollerini kapsar, C eki standart kütüphane tarafından sağlanan türetilebilir
trait'leri kapsar, D eki bazı kullanışlı geliştirme araçlarını kapsar, E eki
ise Rust baskılarını açıklar. F ekinde kitabın çevirilerini bulabilirsiniz, ve
G ekinde Rust'ın nasıl yapıldığını ve gecelik Rust'ın ne olduğunu kapsayacağız.

Bu kitabı okumanın yanlış bir yolu yoktur: eğer bir bölümü geçmek isterseniz,
geçin! Eğer kafanız karışırsa, önceki bölümlere geri atlayabilirsiniz. Sizin
için her ne uygunsa, onu yapın.

<span id="ferris"></span>

Rust öğrenme sürecindeki önemli bir şey, derleyicinin gösterdiği hata
mesajlarının nasıl okunması gerektiğini öğrenmektir: bunlar sizi çalışan bir
koda götürecektir. Bunun için, derlenmeyen birçok örnek vereceğiz, bu
örneklerde derleyici size hata mesajları gösterecek. Yani rasgele bir örnek
girip çalıştırırsanız, derlenmeyebilir! Denediğiniz örneğin hata vermesi
gerekip gerekmediğini anlamak için etrafındaki metni okuyun. Ayrıca Ferris,
çalışmaması gereken kodu ayırt etmenizde size yardımcı olacaktır:



| Ferris                                                                                                                     | Anlamı                             |
|-----------------------------------------------------------------------------------------------------------------------------|-----------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Bir soru işaretiyle Ferris"/>                        | Bu kod derlenmez!                 |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris ellerini kaldırıyor"/>                                  | Bu kod panikler!                  |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Bir pençesi yukarıda ona Ferris omuz silkiyor"/> | Bu kod istenen davranışı üretmez. |

Her durumda derlenmeyen herhangi bir kodun doğru sürümüne sizi yönlendireceğiz.

## Kaynak Kodu

Bu kitabın oluşturulduğu kaynak dosyaları [GitHub][book]'da bulunabilir.

[book]: https://github.com/rust-lang/book/tree/main/src
