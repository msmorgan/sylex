/-- "<object>'s color" [CR#105.2]: what a Mana Substitute's colored pip reads back from the
acted-on object ("mana of that creature's color" [CR#702.51a]). -/
primitive colorOf (object : NounPhrase) : ColorTerm := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.ColorTerm.colorOf object
}
