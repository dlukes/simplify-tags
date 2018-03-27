extern crate regex;

use std::borrow::Cow;
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
        // metoda `.replace_all()` vrací typ `Cow`, který umí obalit buď přímo vlastněný `String`
        // (pokud náhrada proběhla a byl tím pádem alokován řetězec s novým obsahem), nebo referenci
        // `&str` na původní řetězec (pokud náhrada neproběhla a alokovat nový řetězec by bylo
        // zbytečné). s jeho pomocí můžeme minimalizovat počet alokací i v naší kaskádě regulárních
        // výrazů, ale už je to trochu ošemetnější než předchozí varianta: musíme zajistit, aby
        // žádný z řetězců, které v mezikrocích mohou vzniknout, nezmizel, dokud na něj existují
        // reference (třeba skrz sérii neúspěšných nahrazení), jinak bychom měli problém s *null
        // pointer*.

        // (při měření je v tomto případě rozdíl v rychlosti oproti předchozí verzi minimální (pokud
        // vůbec nějaký...), mnohem důležitější je samotné větvení, které vlastně už samo o sobě
        // počet alokací výrazně snižuje, protože probíhají jen v případě, že řádek matchne daný
        // regulární výraz. tato nová verze navíc eliminuje alokace v situaci, že řádek matchne, ale
        // nakonec neproběhne žádná náhrada, ale takových případů je zdá se minimum.)

        // jinak kód vypadá víceméně podobně, jen do něj musíme vrátit vnořenou *scope*, protože
        // skrz `Cow` může *immutable* reference na původní řádek přežít až do konce cyklu (pokud
        // nedojde k žádné náhradě), takže si pak nemůžeme vypůjčit *mutable* referenci, abychom
        // mohli řádek vymazat metodou `.clear()`.
        {
            let replaced = if abbr_guard.is_match(&radek) {
                // tady nepotřebujeme `.into_owned()`, vrátíme přímo `Cow`, který dostaneme od
                // `.replace_all()`
                abbr.replace_all(&radek, r"$1------------8")
            } else {
                // tady musíme vytvořit `Cow` odkazující na původní řádek (náhrada neproběhla, ušetříme
                // si tedy alokaci nového řetězce)
                Cow::Borrowed(&radek[..])
            };
            // vzhledem k tomu, že v některých větvích je náhrad víc, může nastat situace, že první
            // náhrada uspěje a druhá ne. v takovém případě v té první vznikne mezivýsledek (nově
            // alokovaný řetězec) a ta druhá vrátí jen referenci na tento mezivýsledek. vzhledem k
            // tomu, že tuto referenci chceme pak uložit do proměnné `replaced`, musíme zajistit,
            // aby mezivýsledek existoval tak dlouho, jako existuje tato reference. je tedy potřeba
            // jej uložit do proměnné, která je deklarovaná ve stejné *scope* jako `replaced`.
            let intermediate_result;
            let replaced = if p76_guard.is_match(&replaced) {
                intermediate_result = p7.replace_all(&replaced, r"$1-$2");
                p6.replace_all(&intermediate_result, r"$1-$2")
            } else if p1sxf_guard.is_match(&replaced) {
                intermediate_result = p1sx.replace_all(&replaced, r"$1-$2");
                p1sf.replace_all(&intermediate_result, r"$1-$2")
            } else if vs_guard.is_match(&replaced) {
                vs.replace_all(&replaced, r"$1-")
            } else if vc_guard.is_match(&replaced) {
                vc.replace_all(&replaced, r"$1-$2-")
            } else if cnlr_guard.is_match(&replaced) {
                cnlr.replace_all(&replaced, r"$1=-------------")
            } else if crom.is_match(&replaced) {
                crom.replace_all(&replaced, r"$1=")
            } else if vbx_guard.is_match(&replaced) {
                vbx.replace_all(&replaced, r"$1XX-------------")
            } else if co_guard.is_match(&replaced) {
                co.replace_all(&replaced, r"$1---")
            } else {
                replaced
            };
            write!(stdout, "{}", replaced).expect("chyba při výpisu řádku");
        }
        radek.clear();
    }
}
