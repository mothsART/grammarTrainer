#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate phonetic;

pub mod schema;
pub mod models;
pub mod verbs;

use dotenv::dotenv;
use std::env;

use diesel::{insert_into};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use phonetic::phonetics::phonex::phonex;

use schema::verbs::dsl::verbs as db_verbs;
use verbs::{ Verbe, Sentence };
use models::{ NewVerb };

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("GRAMMATRAINER_DATABASE_URL")
    .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct GrammaTrainerDataBaseStruct {
    pub connection: SqliteConnection
}

pub trait GrammaTrainerDataBase {
    fn new() -> GrammaTrainerDataBaseStruct;
    fn insert(&mut self, verbe: Verbe, sentence: Sentence);
}

impl GrammaTrainerDataBase for GrammaTrainerDataBaseStruct {
    fn new() -> GrammaTrainerDataBaseStruct {
        GrammaTrainerDataBaseStruct {
            connection: establish_connection()
        }
    }

    fn insert(&mut self, verbe: Verbe, sentence: Sentence) {
        let personne = verbe.inf.to_string();
        let verb = NewVerb {
            pronoun:                    sentence.personne as i32,
            infinitive:                 &personne,
            tense:                      sentence.temps as i32,
            conjugated:                 &sentence.verbe,
            phonex:                     &phonex(&sentence.verbe),
            infinitive_phonex:          &phonex(&personne),
            verb_group:                 verbe.group.gv as i32,
            type_verb:                  verbe.group.tv as i32,
            verbe_intransitif:          verbe.group.i  as i32,
            verbe_intransitif_direct:   verbe.group.td as i32,
            verbe_intransitif_indirect: verbe.group.ti as i32,
            verbe_pronomial:            verbe.group.p  as i32,
            verbe_impersonnel:          verbe.group.ip as i32,
            verbe_auxilliaire_etre:     verbe.group.ae as i32,
            verbe_auxilliaire_avoir:    verbe.group.aa as i32
        };
        insert_into(db_verbs)
            .values(&verb)
            .execute(&self.connection).unwrap();
    }
}
