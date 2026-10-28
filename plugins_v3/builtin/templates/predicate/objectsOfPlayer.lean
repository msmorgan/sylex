/-- "<player>" as the quality of "protection from <player>": each object `player` controls, and
each object `player` owns that no player controls [CR#702.16k]. -/
-- [CR#702.16k]
template objectsOfPlayer (player : NounPhrase) : Predicate := {
  spelling := "<player>",
  body :=
    or
      [controlledBy player,
        and
          [ownedBy player,
            not
              (controlledBy (a isPlayer))]]
}
