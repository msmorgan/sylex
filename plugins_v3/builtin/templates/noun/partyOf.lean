/-- "[a player]'s party" [CR#700.8]. -/
template partyOf (player : NounPhrase) : NounPhrase := {

  -- TODO(spelling): <player>'s party; string substitution cannot select possessive or subject
  -- agreement.
  body :=
    Semantics.Templates.NounPhrase.nounPhraseOneEachOf Semantics.Templates.partyRoles
      (Semantics.Templates.NounPhrase.allOf
        (Semantics.Templates.Predicate.and
          [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.hasPossessor Semantics.PossessorAxis.controller player]))
}
