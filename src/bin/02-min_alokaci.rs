#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::{self, BufRead, BufWriter, StdoutLock, Write};
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::new(stdout);

    // taky ale můžeme recyklovat stejný řetězec pro načítání řádek, čímž se vyhneme zbytečně častým
    // alokacím (paměť budeme alokovat jen ve chvíli, kdy načteme řádek, který se do stávajícího
    // řetězce nevejde)
    let mut radek = String::new();
    while stdin
        .read_line(&mut radek)
        .expect("chyba při čtení řádku") > 0
    {
        make_replacement(&mut stdout, &radek);
        // řetězec je po zpracování každého řádku zase potřeba vymazat
        radek.clear();
    }
    stdout.flush();
}

fn make_replacement(stdout: &mut BufWriter<StdoutLock>, radek: &str) {
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
    write!(stdout, "{}", radek).expect("chyba při výpisu řádku");
}
