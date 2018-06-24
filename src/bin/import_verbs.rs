#[macro_use]
extern crate nom;

use std::str;
use std::fs::File;
use std::io::Read;

use nom::IResult;

// ÉTIQUETTES DE GRAMMALECTE

#[derive(Debug)]
enum GroupeVerbal {
    EtreOuAvoir,
    PremierGroupe,
    DeuxiemeGroupe,
    TroisiemeGroupe
}

#[derive(Debug)]
enum TypeDeVerbe {
    Etre,
    Avoir,
    Autre
}

#[derive(Debug)]
enum VerbeIntransitif {
    Intransitif,
    X,
    Indefini
}

#[derive(Debug)]
enum VerbeTransitifDirect {
    Transitif,
    X,
    Indefini
}

#[derive(Debug)]
enum VerbeTransitifInDirect {
    Intransitif,
    X,
    Indefini
}

#[derive(Debug)]
enum VerbePronomial {
    Toujours,
    Reciproque,
    AvecPronomEN,
    Accord,
    Oui,
    Jamais,
    Possiblement,
    X,
    Indefini
}

#[derive(Debug)]
enum VerbeImpersonnel {
    Impersonnel,
    X,
    Indefini
}

#[derive(Debug)]
enum VerbeAuxilliaireEtre {
    Oui,
    Non,
    Indefini
}

#[derive(Debug)]
enum VerbeAuxilliaireAvoir {
    Oui,
    Non,
    Indefini
}

#[derive(Debug)]
enum Temps {
    Infinitif,
    ParticipePresent,
    ParticipePasse,
    IndicatifPresent,
    IndicatifImparfait,
    IndicatifPasseSimple,
    IndicatifFutur,
    SubjonctifPresent,
    SubjonctifImparfait,
    Conditionnel,
    Imperatif
}

named!(extract_gv(&[u8]) -> GroupeVerbal,
    alt!(
        map!(tag!("0"), |_| GroupeVerbal::EtreOuAvoir)    |
        map!(tag!("1"), |_| GroupeVerbal::PremierGroupe)  |
        map!(tag!("2"), |_| GroupeVerbal::DeuxiemeGroupe) |
        map!(tag!("3"), |_| GroupeVerbal::TroisiemeGroupe)
    )
);

named!(extract_tv(&[u8]) -> TypeDeVerbe,
    alt!(
        map!(tag!("e"), |_| TypeDeVerbe::Etre)  |
        map!(tag!("a"), |_| TypeDeVerbe::Avoir) |
        map!(tag!("_"), |_| TypeDeVerbe::Autre)
    )
);

named!(extract_i(&[u8]) -> VerbeIntransitif,
    alt!(
        map!(tag!("i"), |_| VerbeIntransitif::Intransitif) |
        map!(tag!("x"), |_| VerbeIntransitif::X)           |
        map!(tag!("_"), |_| VerbeIntransitif::Indefini)
    )
);

named!(extract_td(&[u8]) -> VerbeTransitifDirect,
    alt!(
        map!(tag!("t"), |_| VerbeTransitifDirect::Transitif) |
        map!(tag!("x"), |_| VerbeTransitifDirect::X)         |
        map!(tag!("_"), |_| VerbeTransitifDirect::Indefini)
    )
);

named!(extract_ti(&[u8]) -> VerbeTransitifInDirect,
    alt!(
        map!(tag!("n"), |_| VerbeTransitifInDirect::Intransitif) |
        map!(tag!("x"), |_| VerbeTransitifInDirect::X)           |
        map!(tag!("_"), |_| VerbeTransitifInDirect::Indefini)
    )
);

named!(extract_p(&[u8]) -> VerbePronomial,
    alt!(
        map!(tag!("p"), |_| VerbePronomial::Toujours)     |
        map!(tag!("q"), |_| VerbePronomial::Reciproque)   |
        map!(tag!("r"), |_| VerbePronomial::AvecPronomEN) |
        map!(tag!("u"), |_| VerbePronomial::Accord)       |
        map!(tag!("v"), |_| VerbePronomial::Oui)          |
        map!(tag!("e"), |_| VerbePronomial::Jamais)       |
        map!(tag!("x"), |_| VerbePronomial::Possiblement) |
        map!(tag!("_"), |_| VerbePronomial::Indefini)
    )
);

named!(extract_ip(&[u8]) -> VerbeImpersonnel,
    alt!(
        map!(tag!("m"), |_| VerbeImpersonnel::Impersonnel) |
        map!(tag!("x"), |_| VerbeImpersonnel::X)           |
        map!(tag!("_"), |_| VerbeImpersonnel::Indefini)
    )
);

named!(extract_ae(&[u8]) -> VerbeAuxilliaireEtre,
    alt!(
        map!(tag!("e"), |_| VerbeAuxilliaireEtre::Oui) |
        map!(tag!("x"), |_| VerbeAuxilliaireEtre::Non) |
        map!(tag!("z"), |_| VerbeAuxilliaireEtre::Indefini) |
        map!(tag!("_"), |_| VerbeAuxilliaireEtre::Indefini)
    )
);

named!(extract_aa(&[u8]) -> VerbeAuxilliaireAvoir,
    alt!(
        map!(tag!("a"), |_| VerbeAuxilliaireAvoir::Oui) |
        map!(tag!("x"), |_| VerbeAuxilliaireAvoir::Non) |
        map!(tag!("z"), |_| VerbeAuxilliaireAvoir::Indefini) |
        map!(tag!("_"), |_| VerbeAuxilliaireAvoir::Indefini)
    )
);

#[derive(Debug)]
struct GroupeDuVerbe {
  gv: GroupeVerbal,
  tv: TypeDeVerbe,
  i:  VerbeIntransitif,
  td: VerbeTransitifDirect,
  ti: VerbeTransitifInDirect,
  p:  VerbePronomial,
  ip: VerbeImpersonnel,
  ae: VerbeAuxilliaireEtre,
  aa: VerbeAuxilliaireAvoir
}

#[derive(Debug)]
struct Verb {
  inf: String,
  group: GroupeDuVerbe
}

named!(extract_group<&[u8], GroupeDuVerbe>, do_parse!(
    gv: extract_gv >>
    tv: extract_tv >>
    i:  extract_i >>
    td: extract_td >>
    ti: extract_ti >>
    p:  extract_p >>
    ip: extract_ip >>
    ae: extract_ae >>
    aa: extract_aa >>
    (GroupeDuVerbe { 
        gv: gv, 
        tv: tv,
        i:  i,
        td: td,
        ti: ti,
        p:  p,
        ip: ip,
        ae: ae,
        aa: aa
    })
));

named!(infinitive<&[u8], Verb>, do_parse!( 
    inf: map_res!(take_until!("\t"), str::from_utf8) >>
    tag!("\t") >>
    group: extract_group >>
    (Verb { 
		inf:   inf.to_string(),
		group: group
	})
));

fn main() {
    /*
    println!("{:?}", extract_gv(b"1"));
    println!("{:?}", extract_tv(b"_"));
    println!("{:?}", extract_i( b"i"));
    println!("{:?}", extract_td(b"t"));
    println!("{:?}", extract_ti(b"_"));
    println!("{:?}", extract_p( b"q"));
    println!("{:?}", extract_ip(b"_"));
    println!("{:?}", extract_ae(b"_"));
    println!("{:?}", extract_aa(b"a"));
    
    println!("{:?}", extract_group(b"1_it_q__a"));
    println!("{:?}", infinitive(b"abaisser\t1_it_q__a"));
    */
    
    let mut file = File::open("dictConj.txt").expect("Could not open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read file");
    for line in content.lines() {
        println!("{:?}", infinitive(line.as_bytes()));
    }
}
