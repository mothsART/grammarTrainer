#[derive(Debug, Clone, Copy)]
pub enum GroupeVerbal {
    EtreOuAvoir     = 0,
    PremierGroupe   = 1,
    DeuxiemeGroupe  = 2,
    TroisiemeGroupe = 3
}

#[derive(Debug, Clone, Copy)]
pub enum TypeDeVerbe {
    Etre,
    Avoir,
    Autre
}

#[derive(Debug, Clone, Copy)]
pub enum VerbeIntransitif {
    Intransitif,
    X,
    Indefini
}

#[derive(Debug, Clone, Copy)]
pub enum VerbeTransitifDirect {
    Transitif,
    X,
    Indefini
}

#[derive(Debug, Clone, Copy)]
pub enum VerbeTransitifInDirect {
    Intransitif,
    X,
    Indefini
}

#[derive(Debug, Clone, Copy)]
pub enum VerbePronomial {
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

#[derive(Debug, Clone, Copy)]
pub enum VerbeImpersonnel {
    Impersonnel,
    X,
    Indefini
}

#[derive(Debug, Clone, Copy)]
pub enum VerbeAuxilliaireEtre {
    Oui,
    Non,
    Indefini
}

#[derive(Debug, Clone, Copy)]
pub enum VerbeAuxilliaireAvoir {
    Oui,
    Non,
    Indefini
}

#[derive(Debug, Clone, Copy)]
pub enum Temps {
    Infinitif            = 0,
    ParticipePresent     = 1,
    ParticipePasse       = 2,
    IndicatifPresent     = 3,
    IndicatifImparfait   = 4,
    IndicatifPasseSimple = 5,
    IndicatifFutur       = 6,
    SubjonctifPresent    = 7,
    SubjonctifImparfait  = 8,
    Conditionnel         = 9,
    Imperatif            = 10
}

#[derive(Debug)]
pub enum Personne {
    PremiereSingulier                        = 0,
    PremiereSingulierInterrogatifAccentGrave = 1,
    PremiereSingulierInterrogatifAccentAigu  = 2,
    DeuxiemeSingulier                        = 3,
    TroisiemeSingulier                       = 4,
    PremierePluriel                          = 5,
    DeuxiemePluriel                          = 6,
    TroisiemePluriel                         = 7
}

#[derive(Debug, Clone, Copy)]
pub struct GroupeDuVerbe {
  pub gv: GroupeVerbal,
  pub tv: TypeDeVerbe,
  pub i:  VerbeIntransitif,
  pub td: VerbeTransitifDirect,
  pub ti: VerbeTransitifInDirect,
  pub p:  VerbePronomial,
  pub ip: VerbeImpersonnel,
  pub ae: VerbeAuxilliaireEtre,
  pub aa: VerbeAuxilliaireAvoir
}

#[derive(Debug, Clone, Copy)]
pub struct Verbe<'a> {
  pub inf: &'a str,
  pub group: GroupeDuVerbe
}

#[derive(Debug)]
pub struct Sentence {
  pub temps: Temps,
  pub personne: Personne,
  pub verbe: String
}
