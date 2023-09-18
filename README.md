# pdf-manager

Een kleine tool om gemakkelijk PDFs te openen,
bestaande uit een CLI `pdfmgr` geschreven in rust en
een shell script dat gebruik maakt van `rofi` om PDFs te openen.

# Installatie

## Prerequisites

Onderstaande software is vereist.
* De volledige [rust](https://github.com/rust-lang/rust) toolchain
* [rofi](https://github.com/davatorium/rofi)
* [zathura](https://github.com/pwmt/zathura)

## Compileren

Navigeer naar een directory waar de scripts terecht moeten komen,
en run de commands
```bash
git clone https://github.com/amber0534/pdf-manager.git```
cd pdf-manager/pdf_manager
```

Open `src/main.rs` en verander de `const FILENAME_PDFS` zodat
deze een correct pad naar het bestand `pdfs.txt` bevat.
Gebruik een absoluut pad.
Compileer nu het aangepaste rust bestand.

```
cargo build --release
cargo install --path .
```

## Runnen

Om het shell script te runnen moet de `PATH` environment
variabele ook `~/.cargo/bin` bevatten.
Navigeer nu naar het `launch.sh` bestand en pas de variabelen
`ROFI_THEME` en `DATABASE_FILE` aan zodat deze respectievelijk
naar het bestand `theme.rasi` en het eerder genoemde `pdfs.txt` bestand
wijzen.
Gebruik hierbij absolute paden.

Nu kan door het aanroepen van `launch.sh` een rofi menu geopend
worden waarin PDF bestanden geopend kunnen worden.
Met behulp van de command line interface `pdfmgr` kunnen
PDFs toegevoegt en verwijdert worden aan de database.
Run `pdfmgr -h` om alle opties te bekijken.

