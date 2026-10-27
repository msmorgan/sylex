/-- "{G/P}": a Phyrexian mana symbol [CR#107.4f]. -/
-- [CR#107.4f]
template phyrexianPip (color : Color) : ManaSymbol := {
  -- TODO(spelling): Context-dependent wording: {G/P}; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.ManaSymbol.phyrexian color Option.none
}
