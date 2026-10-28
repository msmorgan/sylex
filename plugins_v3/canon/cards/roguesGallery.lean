/-- Rogues' Gallery -/
card roguesGallery : Spelled :=
  { name := "Rogues' Gallery", cost := [generic 2, pip .black], types := [.sorcery],
    text :=
      [ spell
          (doForEachKind .color none .color (returnToHand
                (targets (upTo 1) (and [isCreature, ofChosen .color, in_ yourGraveyard]))
                yourGraveyard)) ] }
