/-- "You may cast this card from <zone> <timing>": the instant type's cast permission
[CR#304.1]. -/
-- [CR#304.1]
template castableFrom (zone : Location) (timing : Timing) : Conferral := {
  spelling := "You may cast this card from <zone> <timing>",
  body :=
    Semantics.Conferral.property
      (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
        (Semantics.Templates.Action.procedure Semantics.Procedure.cast Semantics.Templates.NounPhrase.this_ Semantics.Templates.PlayPayment.itsOwnCost
          Option.none (Option.some zone) Option.none)
        (Option.some timing))
}
