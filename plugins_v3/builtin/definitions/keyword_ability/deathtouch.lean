/-- Deathtouch is a static ability: a creature with toughness greater than 0 dealt damage by a
source with deathtouch since the last state-based action check is destroyed as a state-based
action. [CR#702.2a,702.2b] -/
keyword deathtouch : Ability := {
  lexical := { lemma := "deathtouch" },
  spelling := "deathtouch",
  body :=
    []
}
