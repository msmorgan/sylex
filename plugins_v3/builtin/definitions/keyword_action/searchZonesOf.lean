/-- "search <whose> graveyard, hand, and library for <for_>", the owner written once
(`Definition`, [CR#701.23]). The `for_` parameter is deliberately unbound: its explicit
`inCaller` opts out of generated identity. Whether the search selection should be bound
is an open owner question; this preserves the existing expansion. -/
action searchZonesOf (whose : NounPhrase) (for_ : NounPhrase) : Action := {
  -- TODO(spelling): search <whose>'s graveyard, hand, and library for <for_>; string substitution
  -- cannot form possessives.
  lexical := { verbs := [{ lemma := "search", frames := [(TransitiveWithPP ["for"]), Monotransitive, (Prepositional ["for"])] }] },
  deed := "search",
  body :=
    Semantics.Templates.Action.actionSearch
      (Semantics.Templates.Location.and
        [Semantics.Templates.Location.zone Semantics.Zone.graveyard (Option.some whose),
          Semantics.Templates.Location.zone Semantics.Zone.hand (Option.some whose),
          Semantics.Templates.Location.zone Semantics.Zone.library (Option.some whose)])
      (Semantics.NounPhrase.inCaller for_)
}
