CREATE TABLE verbs (
  id         INTEGER NOT NULL PRIMARY KEY,
  pronoun    INTEGER(1),
  infinitive VARCHAR(255),
  tense      INTEGER(1),
  conjugated VARCHAR(255),
  phonex     VARCHAR(255),
  verb_group INTEGER(1)
)
