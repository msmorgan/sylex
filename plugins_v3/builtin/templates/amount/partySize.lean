template partySize : Amount := {
  spelling := "the number of creatures in your party",
  body :=
    Semantics.Templates.Amount.partySizeOf Semantics.Templates.NounPhrase.you
}
