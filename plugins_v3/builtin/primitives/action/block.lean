/-- "<subject> blocks [<attacker>]" [CR#509.1a] -/
-- [CR#509.1a]
primitive block (attacker : NounPhrase := any) : Action := {
  -- TODO(spelling): Omit the object when attacker is any; otherwise name the blocked attacker.
  body :=
    Semantics.Primitives.Action.block attacker
}
