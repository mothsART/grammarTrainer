use super::schema::{verbs};

#[derive(Queryable)]
pub struct Verb {
    pub id: i32,
    pub pronoun: i8,
    pub infinitive: String,
    pub tense: i8,
    pub conjugated: String,

    pub phonex: String,
    pub infinitive_phonex: String,

    pub verb_group: i8,

    pub type_verb: i8,
    pub verbe_intransitif: i8,
    pub verbe_intransitif_direct: i8,
    pub verbe_intransitif_indirect: i8,
    pub verbe_pronomial: i8,
    pub verbe_impersonnel: i8,
    pub verbe_auxilliaire_etre: i8,
    pub verbe_auxilliaire_avoir: i8
}

#[derive(Insertable)]
#[table_name="verbs"]
pub struct NewVerb<'a> {
    pub pronoun: i32,
    pub infinitive: &'a str,
    pub tense: i32,
    pub conjugated: &'a str,

    pub phonex: &'a str,
    pub infinitive_phonex: &'a str,

    pub verb_group: i32,

    pub type_verb: i32,
    pub verbe_intransitif: i32,
    pub verbe_intransitif_direct: i32,
    pub verbe_intransitif_indirect: i32,
    pub verbe_pronomial: i32,
    pub verbe_impersonnel: i32,
    pub verbe_auxilliaire_etre: i32,
    pub verbe_auxilliaire_avoir: i32
}
