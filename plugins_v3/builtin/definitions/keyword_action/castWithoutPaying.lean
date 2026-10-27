/-- "<performer> casts <subject> without paying its mana cost [if the resulting spell is …]"
(`Definition`, [CR#701.5,601.2]). -/
action castWithoutPaying (subject : NounPhrase) (resulting : Option Predicate) :
    Action := {
  -- TODO(spelling): cast <subject> without paying its/their mana cost [<resulting>]; string
  -- substitution cannot branch on optional fields or supply contextual agreement and morphology.
  lexical := { verbs := [{ lemma := "cast", past := "cast", frames := [Monotransitive] }] },
  deed := "cast",
  body :=
    Semantics.Templates.Action.procedure Semantics.Procedure.cast subject Semantics.Templates.PlayPayment.withoutPaying resulting Option.none
      Option.none
}
