/-- "[a player]'s party" [CR#700.8]. -/
template partyOf (player : NounPhrase) : NounPhrase := {

  spelling := "<player>'s party",
  body :=
    oneEachOf (allOf
        (and
          [isCreature, hasPossessor .controller player])) partyRoles
}
