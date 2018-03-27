extern crate lazy_static;
extern crate regex;

use std::io;
use regex::Regex;
use lazy_static::*;

fn main() {

    loop {
        let mut radek = String::new();
    
        match io::stdin().read_line(&mut radek) {
            Ok(0) => break,
            Ok(_) => make_replacement(radek),
            Err(e) => panic!(e)
        }
//        radek.clear();
    }
}

fn make_replacement(mut radek: String) {
    lazy_static! {
        static ref ABBR_REGEX:
        Regex = Regex::new(r"(?P<f><MMt[^>]*>..).{12}8").unwrap();
        static ref P7_REGEX:
        Regex = Regex::new(r"(?P<f><MMt[^>]*>P7-).(?P<s>[34]---)").unwrap();
        static ref P6_REGEX:
        Regex = Regex::new(r"(?P<f><MMt[^>]*>P6-).(?P<s>[23467])").unwrap();
        static ref P1SX_REGEX:
        Regex = Regex::new(r"(?P<f><MMt[^>]*>P[1S]...)[XZMIN](?P<s>[SP]3)")
            .unwrap();
        static ref P1SF_REGEX:
        Regex = Regex::new(r"(?P<f><MMt[^>]*>P[1S]...)F(?P<s>P3)").unwrap();
        static ref VS_REGEX:
        Regex = Regex::new(r"(?P<f><MMt[^>]*>Vs......)[FPRX]").unwrap();
        static ref VC_REGEX:
        Regex = Regex::new(r"(?P<f><MMt[^>]*>Vc.)X(?P<s>...)3").unwrap();
        static ref CNLR_REGEX:
        Regex = Regex::new(r"(?P<f>>([\-\+])?[0-9][^<]*<MMl[^>]*>[^<]+<MMt[^>]*>C)[nlr](?P<s>.{13})")
            .unwrap();
        static ref CROM_REGEX:
        Regex = Regex::new(r"(?P<f><MMt[^>]*>C)\}").unwrap();
        static ref VBX_REGEX:
        Regex = Regex::new(r"(?P<f><MMt[^>]*>)VB-X---X.{7}").unwrap();
        static ref CO_REGEX:
        Regex = Regex::new(r"(?P<f><MMl[^>]*>co<MMt[^>]*>P4)...").unwrap();
    }
    let mut after = ABBR_REGEX.replace_all(&radek, "$f------------8")
        .to_string();
    radek = after;
    after = P7_REGEX.replace_all(&radek, "$f-$s").to_string();
    radek = after;
    after = P6_REGEX.replace_all(&radek, "$f-$s").to_string();
    radek = after;
    after = P1SX_REGEX.replace_all(&radek, "$f-$s").to_string();
    radek = after;
    after = P1SF_REGEX.replace_all(&radek, "$f-$s").to_string();
    radek = after;
    after = VS_REGEX.replace_all(&radek, "$f-").to_string();
    radek = after;
    after = VC_REGEX.replace_all(&radek, "$f-$s-").to_string();
    radek = after;
    after = CNLR_REGEX.replace_all(&radek, "$f=-------------").to_string();
    radek = after;
    after = CROM_REGEX.replace_all(&radek, "$f=").to_string();
    radek = after;
    after = VBX_REGEX.replace_all(&radek, "$fXX-------------").to_string();
    radek = after;
    after = CO_REGEX.replace_all(&radek, "$f---").to_string();
    radek = after;
    after = P1SF_REGEX.replace_all(&radek, "$f-$s").to_string();
    radek = after;
    print!("{}",radek);
}
