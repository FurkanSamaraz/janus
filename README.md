# Rust Verison Control

Bu proje, Rust dilinde geliştirilen bir versiyon yönetim aracıdır. Proje, bir projenin versiyonunu artırmak, Git etiketlemek ve değişiklikleri otomatik olarak taahhüt etmek için kullanılabilir.

## Başlangıç

Projeyi yerel makinenizde çalıştırmak ve geliştirmek için aşağıdaki adımları takip edebilirsiniz.

### Gereksinimler

Bu projeyi çalıştırmak için aşağıdaki yazılımların yüklü olması gereklidir:

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads)

### Kurulum

#### Projeyi klonlayın:

   ```sh
   git clone https://github.com/kullanici_adi/rust-ornek-proje.git
   ```

#### Proje klasörüne gidin:
```sh
cd rust-ornek-proje
```
#### Projeyi derleyin:
```sh
cargo build
```

### Kullanım
Projeyi çalıştırmak için aşağıdaki komutları kullanabilirsiniz:

#### Versiyonu artırmak için:

```sh
cargo run patch
```

#### Küçük bir versiyon yükseltmesi için:

```sh
cargo run minor
```
#### Büyük bir versiyon yükseltmesi için:

```sh
cargo run major
```
