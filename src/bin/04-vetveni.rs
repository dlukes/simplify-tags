extern crate regex;

use std::io::{self, BufRead, BufWriter, Write};
use regex::Regex;

fn main() {
    let abbr_guard = Regex::new(r"<MMt[^>]*>.{14}8").unwrap();
    let abbr = Regex::new(r"(<MMt[^>]*>..).{12}8").unwrap();
    let p76_guard = Regex::new(r"<MMt[^>]*>P[67]").unwrap();
    let p7 = Regex::new(r"(<MMt[^>]*>P7-).([34]---)").unwrap();
    let p6 = Regex::new(r"(<MMt[^>]*>P6-).([23467])").unwrap();
    let p1sxf_guard = Regex::new(r"<MMt[^>]*>P[1S]").unwrap();
    let p1sx = Regex::new(r"(<MMt[^>]*>P[1S]...)[XZMIN]([SP]3)").unwrap();
    let p1sf = Regex::new(r"(<MMt[^>]*>P[1S]...)F(P3)").unwrap();
    let vs_guard = Regex::new(r"<MMt[^>]*>Vs").unwrap();
    let vs = Regex::new(r"(<MMt[^>]*>Vs......)[FPRX]").unwrap();
    let vc_guard = Regex::new(r"<MMt[^>]*>Vc").unwrap();
    let vc = Regex::new(r"(<MMt[^>]*>Vc.)X(...)3").unwrap();
    let cnlr_guard = Regex::new(r"<MMt[^>]*>C[nlr]").unwrap();
    let cnlr = Regex::new(r"(>([\-\+])?[0-9][^<]*<MMl[^>]*>[^<]+<MMt[^>]*>C)[nlr](.{13})").unwrap();
    // `crom` je sám sobě `crom_guard`
    let crom = Regex::new(r"(<MMt[^>]*>C)\}").unwrap();
    let vbx_guard = Regex::new(r"<MMt[^>]*>VB-X").unwrap();
    let vbx = Regex::new(r"(<MMt[^>]*>)VB-X---X.{7}").unwrap();
    let co_guard = Regex::new(r"<MMl[^>]*>co<").unwrap();
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
        // výrazného urychlení se docílí pomocí větvení, jak už jsi psala v původním mailu. jen se
        // tím trochu komplikuje situace, protože metoda `.replace_all()` vrací buď `String` (pokud
        // nějaká náhrada proběhla), nebo referenci `&str` na původní řetězec (pokud neproběhla),
        // takže musíme nějak zajistit, aby tato reference zůstala platná i vně dané větve *if*.
        // nejjednodušeji to uděláme tak, že prostě alokujeme nový `String` a obsah řetězce za
        // referencí do něj překopírujeme.

        // vytvoříme si za tímto účelem novou proměnnou, kterou inicializujeme...
        let replaced = if abbr_guard.is_match(&radek) {
            // ... buď jako výsledek první náhrady...
            abbr.replace_all(&radek, r"$1------------8").into_owned()
        } else {
            // ... nebo jako kopii řádku
            radek.clone()
        };
        let replaced = if p76_guard.is_match(&replaced) {
            let replaced = p7.replace_all(&replaced, r"$1-$2").into_owned();
            p6.replace_all(&replaced, r"$1-$2").into_owned()
        } else if p1sxf_guard.is_match(&replaced) {
            let replaced = p1sx.replace_all(&replaced, r"$1-$2").into_owned();
            p1sf.replace_all(&replaced, r"$1-$2").into_owned()
        } else if vs_guard.is_match(&replaced) {
            vs.replace_all(&replaced, r"$1-").into_owned()
        } else if vc_guard.is_match(&replaced) {
            vc.replace_all(&replaced, r"$1-$2-").into_owned()
        } else if cnlr_guard.is_match(&replaced) {
            cnlr.replace_all(&replaced, r"$1=-------------")
                .into_owned()
        } else if crom.is_match(&replaced) {
            crom.replace_all(&replaced, r"$1=").into_owned()
        } else if vbx_guard.is_match(&replaced) {
            vbx.replace_all(&replaced, r"$1XX-------------")
                .into_owned()
        } else if co_guard.is_match(&replaced) {
            co.replace_all(&replaced, r"$1---").into_owned()
        } else {
            replaced
        };
        write!(stdout, "{}", replaced).expect("chyba při výpisu řádku");
        radek.clear();
    }
}
