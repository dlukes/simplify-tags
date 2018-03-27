# Navrhované úpravy

Upravené verze kódu jsou k dispozici postupně očíslované v adresáři `src/bin` i
s komentáři (snad dostatečně vysvětlujícími, kdyby ne, jsem k dispozici!), zde
tedy jen přehledově:

- `00-orig.rs`: původní verze
- `01-drobnosti.rs`: drobné úpravy věcí, které mě trkly "na první pohled"
- `02-min_alokaci.rs`: recyklace řetězce, do něhož se načítají řádky (místo
  toho, aby se pokaždé alokoval nový)
- `03-jen_main.rs`: odstranění funkce `make_replacement()` (kvůli lepší
  srovnatelnosti s Perlem; tady nejde tak o rychlost jako spíš o to, aby kód co
  nejvíc připomínal skript)
- `04-vetveni.rs`: optimalizace, o které jsi psala už v e-mailu (má velký vliv)
  -- jednodušší varianta, která občas může zbytečně alokovat paměť
- `05-vetveni_bez_zbytecnych_alokaci.rs`: úspornější varianta předchozí verze
  (v praxi se to ale moc neprojeví)
- `06-bez_parsovani_utf8.rs`: naopak když se přestane hlídat, zda je vstup
  validní UTF-8, tak to ještě pár desítek ms zkrouhne
  
Paralelizovat jsem to nezkoušel, ale to by se asi stejně vyplatilo až na
větších vstupních datech, aby se amortizovaly náklady na orchestraci vláken...?

# Porovnání rychlosti

Shrnutí výsledků:

| Command                             | Mean [ms]     | Min…Max [ms]  |
|-------------------------------------|---------------|---------------|
| `00-orig`                           | 1099.1 ± 15.4 | 1081.3…1133.7 |
| `01-drobnosti`                      | 803.4 ± 11.5  | 787.7…829.3   |
| `02-min_alokaci`                    | 773.6 ± 8.0   | 764.5…785.6   |
| `03-jen_main`                       | 766.0 ± 5.2   | 754.6…774.1   |
| `04-vetveni`                        | 353.6 ± 3.9   | 348.8…360.2   |
| `05-vetveni_bez_zbytecnych_alokaci` | 352.4 ± 4.9   | 344.4…360.7   |
| `06-bez_parsovani_utf8`             | 315.0 ± 8.1   | 308.8…335.7   |
| `simplify_tags.pl`                  | 493.2 ± 5.2   | 487.3…500.1   |

Což není zas až tak o moc rychlejší než ten Perl, nevím, jestli je to pro vás
vůbec relevantní rozdíl.

Kdyby sis to chtěla vyzkoušet na vlastním stroji:

1. `cargo build --release`
2. `cargo install hyperfine`
3. `./benchmark.sh cesta/k/souboru/culik_jacijsme.txt`

V údajích pro původní verzi (`00-orig`) je nápadné, kolik času zabírají
systémová volání (položka *System: ...*), což je dané hlavně splachováním
STDOUT na každém řádku (viz komentáře v `01-drobnosti.rs`).

# Ke čtení

Pár zdrojů, co mi při používání Rustu v praxi (zejména za účely zpracování
textu) hodně pomohly:

- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook)
- [Rust Performance Pitfalls](http://llogiq.github.io/2017/06/01/perf-pitfalls.html)
- [Optimising string processing in Rust](https://lise-henry.github.io/articles/optimising_strings.html)
