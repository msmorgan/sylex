/-- "You may play this card from <zone> <timing>": the land type's play permission [CR#305.1]. -/
-- [CR#305.1]
template playableFrom (zone : Location) (timing : Timing) : Conferral := {
  spelling := "You may play this card from <zone> <timing>",
  body :=
    Semantics.Conferral.property
      (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
        (Semantics.Templates.Action.procedure Semantics.Procedure.play Semantics.Templates.NounPhrase.this_ Semantics.Templates.PlayPayment.itsOwnCost
          Option.none (Option.some zone) Option.none)
        (Option.some timing))
}
