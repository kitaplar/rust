## Yorumlar

Her programcı, programının kolayca okunması için çabalar ancak bazen fazladan
açıklama gerekir. Bu durumlarda programcı, koduna *yorumlar* bırakır. Bu
yorumlar, derleyici tarafından boşverilir ancak kodu okuyan insanlar tarafından
kullanışlı bulunabilir.

İşte basit bir yorum:

```rust
// merhaba, dünya
```

Rust'ta deyimsel yorum stili, iki eğik çizgi ile yorumu başlatmaktır. Yorum,
satırın sonuna kadar devam eder. Tek bir satırdan daha fazla olan yorumlar için,
`//` karakterlerini her satıra koymalısınız, aynı şöyle:

```rust
// Burada karmaşık bir şey yapıyoruz, bu sebeple birçok satıra yayılmış bir
// yoruma ihtiyacımız var! Vay be! İnşallah bu yorum, neler olduğunu bize
// açıklayacak.
```

Yorumlar ayrıca kod içeren satırların sonunda da yer alabilir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Ama genel yorumları şu biçimde, kodun üzerinde ayrı bir satırda göreceksiniz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust ayrıca bir diğer yorum çeşidine sahiptir, bunlara belgelendirme yorumları
denir. Bunları 14. bölümün [“Bir Crate'i Crates.io'ya yayınlamak”][publishing]
bölümünde tartışacağız.

[publishing]: ch14-02-publishing-to-crates-io.html
