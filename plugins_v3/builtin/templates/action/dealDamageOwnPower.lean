/-- "<source> deals damage equal to its power to <recipient>": a Handoff of the damage to
`source` [CR#120.1], whose "its" is an ordinary possessive pronoun that `D′` resolves to the
clause subject, `source`. -/
-- [CR#120.1]
template dealDamageOwnPower (source : NounPhrase) (recipient : NounPhrase) :
    Action := {
  spelling := "<source> deals damage equal to its power to <recipient>",
  body :=
    Semantics.Templates.Action.instruct source
      (Semantics.Templates.Action.dealDamage (Semantics.Templates.Amount.powerOf Semantics.Templates.NounPhrase.it) recipient)
}
