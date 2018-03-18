#[derive(Queryable)]
pub struct Verb {
    pub id: i32,
    pub pronoun: i8,
    pub infinitive: String,
    pub tense: i8,
    pub conjugated: String,
    pub phonex: String,
    pub verb_group: i8
}
