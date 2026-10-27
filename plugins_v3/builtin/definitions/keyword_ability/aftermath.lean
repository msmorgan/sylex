/-- "Aftermath": "You may cast this half of this split card from your graveyard," "This half of
this split card can't be cast from any zone other than a graveyard," and "If this spell was cast
from a graveyard, exile it instead of putting it anywhere else any time it would leave the stack"
[CR#702.127a]. The restriction excludes every zone but a graveyard, any player's, so another
player's graveyard permission still reaches the half; "your" in the permission is its would-be
caster [CR#109.5]. -/
keyword aftermath : Ability := {
  lexical := { lemma := "aftermath" },
  spelling := "aftermath",
  body :=
    [ static
        (can theActor (cast this_ (from_ := yourGraveyard))),
      static
        (cant theActor
          (cast this_ (from_ := some (not graveyard)))),
      static
        (replace (would thisSpell (leave stack))
          (doIf (is thisSpell (did any (cast which (from_ := graveyard)) thisObject))
            (instead (exile thisSpell)))) ]
}
