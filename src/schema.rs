table! {
    verbs (id) {
        id -> Integer,
        pronoun -> Nullable<Integer>,
        infinitive -> Nullable<Text>,
        tense -> Nullable<Integer>,
        conjugated -> Nullable<Text>,
        
        phonex -> Nullable<Text>,
        infinitive_phonex -> Nullable<Text>,
        
        verb_group -> Nullable<Integer>,
    
        type_verb -> Nullable<Integer>,
        verbe_intransitif -> Nullable<Integer>,
        verbe_intransitif_direct -> Nullable<Integer>,
        verbe_intransitif_indirect -> Nullable<Integer>,
        verbe_pronomial -> Nullable<Integer>,
        verbe_impersonnel -> Nullable<Integer>,
        verbe_auxilliaire_etre -> Nullable<Integer>,
        verbe_auxilliaire_avoir -> Nullable<Integer>,
    }
}
