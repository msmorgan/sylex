/-- "transform <subject> [into <face>]" (`Definition`, [CR#701.27]): `into` is what a "transforms
into" trigger watches, a characteristic the object has immediately after [CR#701.27e]. -/
action transform (subject : NounPhrase) (into : Option Predicate := none) : Action := {
  -- TODO(spelling): transform <subject> [into <into>]; string substitution cannot branch on
  -- optional fields or supply contextual agreement and morphology.
  lexical := { verbs := [{ lemma := "transform", frames := [Monotransitive, Intransitive, (Prepositional ["into"])] }], adjectives := ["transformed"] },
  body :=
    turnOver subject into
}
