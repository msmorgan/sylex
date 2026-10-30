/-- "search <whose> graveyard, hand, and library for <for_>", the owner written once
(`Definition`, [CR#701.23]). The `for_` parameter is bound like any other (there is no opt-out). -/
action searchZonesOf (whose : NounPhrase) (for_ : NounPhrase) : Action := {
  spelling := "search <whose>'s graveyard, hand, and library for <for_>",
  lexical := { verbs := [{ lemma := "search", frames := [(TransitiveWithPP ["for"]), Monotransitive, (Prepositional ["for"])] }] },
  deed := "search",
  body :=
    actionSearch
      (and
        [zone graveyard (some whose),
          zone hand (some whose),
          zone library (some whose)])
      for_
}
