#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::{self, BufRead, BufWriter, StdoutLock, Write};
use regex::Regex;

fn main() {
    // STDIN je schovaný za mutexem (kvůli potenciálnímu přístupu z různých vláken), je výhodnější
    // mutex odemknout jen jednou na začátku a na závěr jednou zamknout
    let stdin = io::stdin();
    let stdin = stdin.lock();

    // STDOUT je taktéž schovaný za mutexem a navíc se defaultně splachuje po každém řádku, což
    // nechceme (Perl tohle myslím klasicky nějak kouzelně pozná, když tiskne do roury nebo do
    // souboru, a to splachování si přenastaví; tady je potřeba si o to říct explicitně)
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::new(stdout);

    // pokud nám nevadí, že pro každý řádek alokujeme novou paměť, tak lze použít tento přehlednější
    // zápis (jen pozor, `.lines()` vrací řádky bez CRLF):
    for radek in stdin.lines() {
        let radek = radek.expect("chyba při čtení řádku");
        // stačí reference na řádek; kromě ní ještě předáme referenci na zamčený bufferovaný STDOUT
        make_replacement(&mut stdout, &radek);
    }
}

// proměnná `radek` nemusí být `mut`, spíš je tu vhodné použít *shadowing* (= pomocí `let` vytvoříme
// sérii proměnných stejného jména, které postupně překryjí ty dřívější)
fn make_replacement(stdout: &mut BufWriter<StdoutLock>, radek: &str) {
    // v zájmu co největší podobnosti s Perlem jsem předělal pojmenované *capture groups* na
    // číslované, ale nevím, jestli to má nějaký zásadní vliv
    lazy_static! {
        static ref ABBR_REGEX:
        Regex = Regex::new(r"(<MMt[^>]*>..).{12}8").unwrap();
        static ref P7_REGEX:
        Regex = Regex::new(r"(<MMt[^>]*>P7-).([34]---)").unwrap();
        static ref P6_REGEX:
        Regex = Regex::new(r"(<MMt[^>]*>P6-).([23467])").unwrap();
        static ref P1SX_REGEX:
        Regex = Regex::new(r"(<MMt[^>]*>P[1S]...)[XZMIN]([SP]3)")
            .unwrap();
        static ref P1SF_REGEX:
        Regex = Regex::new(r"(<MMt[^>]*>P[1S]...)F(P3)").unwrap();
        static ref VS_REGEX:
        Regex = Regex::new(r"(<MMt[^>]*>Vs......)[FPRX]").unwrap();
        static ref VC_REGEX:
        Regex = Regex::new(r"(<MMt[^>]*>Vc.)X(...)3").unwrap();
        static ref CNLR_REGEX:
        Regex = Regex::new(r"(>([\-\+])?[0-9][^<]*<MMl[^>]*>[^<]+<MMt[^>]*>C)[nlr](.{13})")
            .unwrap();
        static ref CROM_REGEX:
        Regex = Regex::new(r"(<MMt[^>]*>C)\}").unwrap();
        static ref VBX_REGEX:
        Regex = Regex::new(r"(<MMt[^>]*>)VB-X---X.{7}").unwrap();
        static ref CO_REGEX:
        Regex = Regex::new(r"(<MMl[^>]*>co<MMt[^>]*>P4)...").unwrap();
    }
    // je zbytečné pokaždé volat `.to_string()` a vynucovat si alokaci paměti pro každý
    // mezivýsledek, i když třeba neproběhl match a tím pádem ani náhrada a řetězec se nezměnil
    let radek = ABBR_REGEX.replace_all(&radek, "$1------------8");
    let radek = P7_REGEX.replace_all(&radek, "$1-$2");
    let radek = P6_REGEX.replace_all(&radek, "$1-$2");
    let radek = P1SX_REGEX.replace_all(&radek, "$1-$2");
    let radek = P1SF_REGEX.replace_all(&radek, "$1-$2");
    let radek = VS_REGEX.replace_all(&radek, "$1-");
    let radek = VC_REGEX.replace_all(&radek, "$1-$2-");
    let radek = CNLR_REGEX.replace_all(&radek, "$1=-------------");
    let radek = CROM_REGEX.replace_all(&radek, "$1=");
    let radek = VBX_REGEX.replace_all(&radek, "$1XX-------------");
    let radek = CO_REGEX.replace_all(&radek, "$1---");
    let radek = P1SF_REGEX.replace_all(&radek, "$1-$2");
    // zde chceme tisknout na STDOUT pomocí našeho připraveného bufferovaného handlu
    writeln!(stdout, "{}", radek).expect("chyba při výpisu řádku");
}
