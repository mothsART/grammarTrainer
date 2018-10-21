#[macro_use]
extern crate nom;
extern crate grammatrainer;

use std::str;
use std::fs::File;
use std::io::Read;

use grammatrainer::{
    GrammaTrainerDataBaseStruct,
    GrammaTrainerDataBase
};

use grammatrainer::verbs::*;

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
        map!(tag!("e"), |_| VerbeAuxilliaireEtre::Oui)      |
        map!(tag!("x"), |_| VerbeAuxilliaireEtre::Non)      |
        map!(tag!("z"), |_| VerbeAuxilliaireEtre::Indefini) |
        map!(tag!("_"), |_| VerbeAuxilliaireEtre::Indefini)
    )
);

named!(extract_aa(&[u8]) -> VerbeAuxilliaireAvoir,
    alt!(
        map!(tag!("a"), |_| VerbeAuxilliaireAvoir::Oui)      |
        map!(tag!("x"), |_| VerbeAuxilliaireAvoir::Non)      |
        map!(tag!("z"), |_| VerbeAuxilliaireAvoir::Indefini) |
        map!(tag!("_"), |_| VerbeAuxilliaireAvoir::Indefini)
    )
);

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

named!(infinitive<&[u8], Verbe>, do_parse!( 
    inf: map_res!(take_until!("\t"), str::from_utf8) >>
    tag!("\t") >>
    group: extract_group >>
    (Verbe { 
        inf:   inf,
        group: group
    })
));

named!(extract_temps(&[u8]) -> Temps,
    alt!(
        map!(tag!("infi"), |_| Temps::Infinitif)            |
        map!(tag!("ppre"), |_| Temps::ParticipePresent)     |
        map!(tag!("ppas"), |_| Temps::ParticipePasse)       |
        map!(tag!("ipre"), |_| Temps::IndicatifPresent)     |
        map!(tag!("iimp"), |_| Temps::IndicatifImparfait)   |
        map!(tag!("ipsi"), |_| Temps::IndicatifPasseSimple) |
        map!(tag!("ifut"), |_| Temps::IndicatifFutur)       |
        map!(tag!("cond"), |_| Temps::Conditionnel)         |
        map!(tag!("spre"), |_| Temps::SubjonctifPresent)    |
        map!(tag!("simp"), |_| Temps::SubjonctifImparfait)  |
        map!(tag!("impe"), |_| Temps::Imperatif)
    )
);

named!(extract_personne(&[u8]) -> Personne,
    alt!(
        map!(tag!("1sg"),  |_| Personne::PremiereSingulier)                        |
        map!(tag!("1isg"), |_| Personne::PremiereSingulierInterrogatifAccentGrave) |
        map!(tag!("1jsg"), |_| Personne::PremiereSingulierInterrogatifAccentAigu)  |
        map!(tag!("2sg"),  |_| Personne::DeuxiemeSingulier)                        |
        map!(tag!("3sg"),  |_| Personne::TroisiemeSingulier)                       |
        map!(tag!("1pl"),  |_| Personne::PremierePluriel)                          |
        map!(tag!("2pl"),  |_| Personne::DeuxiemePluriel)                          |
        map!(tag!("3pl"),  |_| Personne::TroisiemePluriel)
    )
);

named!(sentence<&[u8], Sentence>, do_parse!( 
    tag!("_\t") >>
    temps: extract_temps >>
    tag!(" ") >>
    personne: extract_personne >>
    tag!("\t") >>
    verbe: take_until!("\n") >>
    (Sentence {
        temps: temps,
        personne: personne,
        verbe: str::from_utf8(verbe).unwrap().to_string()
    })
));

fn main() {
    let mut db = GrammaTrainerDataBaseStruct::new();
    let mut file = File::open("essai.txt").expect("Could not open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read file");
    let mut inf = infinitive(b"abaisser\t1_it_q__a").unwrap().1;
    for line in content.lines() {
        match infinitive(line.as_bytes()) {
            Ok(v)  => { inf = v.1; },
            Err(_e) => {
                match sentence(format!("{}\n", line).as_bytes()) {
                    Ok(s) => {
                        db.insert(inf, s.1);
                    },
                    Err(_e) => {}
                }
            }
        }
    }
}
