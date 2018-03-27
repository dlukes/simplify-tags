extern crate regex;

use std::io::{self, BufRead, BufWriter, Write};
use regex::Regex;

fn main() {
    // taky se můžeme vyhnout použití `lazy_static!` a dát všechno do jedné funkce. výsledek není
    // zdá se nějak dramaticky rychlejší, jen si ušetříme externí závislost na *crate* `lazy_static`
    // a kód je vizuálně podobnější skriptu v Perlu, působí jednodušeji (?).
    let abbr = Regex::new(r"(<MMt[^>]*>..).{12}8").unwrap();
    let p7 = Regex::new(r"(<MMt[^>]*>P7-).([34]---)").unwrap();
    let p6 = Regex::new(r"(<MMt[^>]*>P6-).([23467])").unwrap();
    let p1sx = Regex::new(r"(<MMt[^>]*>P[1S]...)[XZMIN]([SP]3)").unwrap();
    let p1sf = Regex::new(r"(<MMt[^>]*>P[1S]...)F(P3)").unwrap();
    let vs = Regex::new(r"(<MMt[^>]*>Vs......)[FPRX]").unwrap();
    let vc = Regex::new(r"(<MMt[^>]*>Vc.)X(...)3").unwrap();
    let cnlr = Regex::new(r"(>([\-\+])?[0-9][^<]*<MMl[^>]*>[^<]+<MMt[^>]*>C)[nlr](.{13})").unwrap();
    let crom = Regex::new(r"(<MMt[^>]*>C)\}").unwrap();
    let vbx = Regex::new(r"(<MMt[^>]*>)VB-X---X.{7}").unwrap();
    let co = Regex::new(r"(<MMl[^>]*>co<MMt[^>]*>P4)...").unwrap();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::new(stdout);

    let mut radek = String::new();
    while stdin
        .read_line(&mut radek)
        .expect("chyba při čtení řádku") > 0
    {
        // náhrady je potřeba uzavřít do zvláštní *scope*; důvod viz chyba při kompilaci, když se ty
        // následující složené závorky dají pryč. ve zkratce...
        {
            // ... tady si vypůjčíme immutable odkaz na `radek`...
            let radek = abbr.replace_all(&radek, "$1------------8");
            let radek = p7.replace_all(&radek, "$1-$2");
            let radek = p6.replace_all(&radek, "$1-$2");
            let radek = p1sx.replace_all(&radek, "$1-$2");
            let radek = p1sf.replace_all(&radek, "$1-$2");
            let radek = vs.replace_all(&radek, "$1-");
            let radek = vc.replace_all(&radek, "$1-$2-");
            let radek = cnlr.replace_all(&radek, "$1=-------------");
            let radek = crom.replace_all(&radek, "$1=");
            let radek = vbx.replace_all(&radek, "$1XX-------------");
            let radek = co.replace_all(&radek, "$1---");
            let radek = p1sf.replace_all(&radek, "$1-$2");
            write!(stdout, "{}", radek).expect("chyba při výpisu řádku");
        }
        // ... který by bez té zvláštní *scope* platil ještě tady, ve chvíli, kdy si kvůli metodě
        // `.clear()` potřebujeme vypůjčit *mutable* odkaz na `radek`, a obojí zároveň nejde.
        radek.clear();
    }
}
