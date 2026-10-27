/-- "<player>" as the quality of "protection from <player>": each object `player` controls, and
each object `player` owns that no player controls [CR#702.16k]. -/
-- [CR#702.16k]
template objectsOfPlayer (player : NounPhrase) : Predicate := {
  spelling := "<player>",
  body :=
    Semantics.Templates.Predicate.or
      [Semantics.Templates.Predicate.controlledBy player,
        Semantics.Templates.Predicate.and
          [Semantics.Templates.Predicate.ownedBy player,
            Semantics.Templates.Predicate.not
              (Semantics.Templates.Predicate.controlledBy (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer))]]
}
