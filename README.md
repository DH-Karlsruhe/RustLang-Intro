# RustLang 🦀 - Intro

Wilkommen und hereinspaziert zu einer [RustLang](https://www.rust-lang.org/)-Einführung.  
Du fragst dich sicherlich, **was Rust?**  
Kurz gesagt handelt es sich bei der Rust um eine moderne System-Programmiersprache,  
wie sie u.a. für die Betriebssysteme, die Web-Entwicklung  
oder praktisch alles eingesetzt werden kann (vgl. [C](https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html)/[C++](https://www.codecademy.com/resources/cheatsheets/language/c-plus-plus)/[Zig](https://ziglang.org/)).  

Falls **dein Ziel** ist, _zuverlässige und effiziente Software_ zu bauen   
und Du _bereit_ bist _zu lesen und zu verstehen_, versuchs doch direkt!  

Einer von vielen Einstiegen ist das [Beispiel-Buch](#rustByExample).

Wenn Du **einfach nur probieren** möchtest, probier vielleicht die [Tour of Rust](https://tourofrust.com/)  
oder ein paar Tools, die mit Rust gebaut worden sind: [Unofficial Awesome Rust](https://github.com/rust-unofficial/awesome-rust)

Alternativ findest Du einen guten Startpunkt auf der [rust-lang.org](https://www.rust-lang.org/learn/get-started) Website.  
_Wo u.a. das standard [Rust-Buch](https://doc.rust-lang.org/book/), uvm. zu finden ist (Die Sprache sprudelt vor Dokumentation)._

Die Tradeoffs werden neben anderen Themen unter anderem in der Präsentation behandelt..

## Präsentation 

> **F**: Macht was?

> **A**: Klärt die Sprach-**Geschichte**, **Enterprise**-Referenzen, Sprach-Features, **Core**-Syntax und **Tradeoffs**.

Auf Github-Pages oder direkt hier in dem Unterverzeichnis `präsentation/`.

_Shotout an den YouTuber `Let's get Rusty` für die schöne Animation im Hintergrund auf der Titel-Folie zum Thema `Sprachkonzepte.`._


## Rust by Example 📖 (en)  

`Rust by Example` gibt es bislang nur auf Englisch und ein paar anderen Sprachen.  
Praktisch handelt es sich dabei um ein offizielles Lern-Buch,  
das ohne weiteres online besucht, gelesen und ausgeführt werden kann: https://doc.rust-lang.org/rust-by-example/.  

Alternativ besteht die Möglichkeit das Buch lokal zu betreiben.  
_Bei Bedarf geht das auch offline, allerdings ohne das Ausführen der Beispiele,_  
da diese auf den [Rust-Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4978bfd9878bbc91aa661bd7b1be6e8d) zugreifen._

### Vorgehen für lokal/offline

 1. Installiere Rust oder ziehe Dir einen [Docker-Container](https://docs.docker.com/language/rust/) (benötigt `docker-cli`)
 2. Öffne das Verzeichnis `rust-by-example/` (git submodule) 
 3. Installiere den MDbook-Crate: `cargo install mdbook`
 4. Um anschließend mit diesem das Rust-Beispiel-Buch zu bauen: `mdbook build` oder bereitzustellen: `mdbook serve`.  

