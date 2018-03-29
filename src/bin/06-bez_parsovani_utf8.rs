extern crate regex;

use std::borrow::Cow;
use std::io::{self, BufRead, BufWriter, Write};
// je potřeba použít regulární výrazy pro matchování na bajtech (API je identické)
use regex::bytes::Regex;

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

    // místo `String` použijeme vektor bajtů, tj. `Vec<u8>`. tomu je jedno, zda obsahuje validní
    // UTF-8, a nijak to nekontroluje.
    let mut radek = vec![];
    while stdin
        .read_until(b'\n', &mut radek)
        .expect("chyba při čtení řádku") > 0
    {
        {
            let replaced = if abbr_guard.is_match(&radek) {
                abbr.replace_all(&radek, &br"$1------------8"[..])
            } else {
                Cow::Borrowed(&radek[..])
            };
            let intermediate_result;
            let replaced = if p76_guard.is_match(&replaced) {
                intermediate_result = p7.replace_all(&replaced, &br"$1-$2"[..]);
                p6.replace_all(&intermediate_result, &br"$1-$2"[..])
            } else if p1sxf_guard.is_match(&replaced) {
                intermediate_result = p1sx.replace_all(&replaced, &br"$1-$2"[..]);
                p1sf.replace_all(&intermediate_result, &br"$1-$2"[..])
            } else if vs_guard.is_match(&replaced) {
                vs.replace_all(&replaced, &br"$1-"[..])
            } else if vc_guard.is_match(&replaced) {
                vc.replace_all(&replaced, &br"$1-$2-"[..])
            } else if cnlr_guard.is_match(&replaced) {
                cnlr.replace_all(&replaced, &br"$1=-------------"[..])
            } else if crom.is_match(&replaced) {
                crom.replace_all(&replaced, &br"$1="[..])
            } else if vbx_guard.is_match(&replaced) {
                vbx.replace_all(&replaced, &br"$1XX-------------"[..])
            } else if co_guard.is_match(&replaced) {
                co.replace_all(&replaced, &br"$1---"[..])
            } else {
                replaced
            };
            stdout
                .write_all(&replaced)
                .expect("chyba při výpisu řádku");
        }
        radek.clear();
    }
    stdout.flush();
}
