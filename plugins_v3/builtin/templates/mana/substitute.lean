/-- "For each <pip> mana …, you may <action> rather than pay that mana": a Mana Substitute
[CR#702.51a,702.66a,702.126a,701.67a]. -/
-- [CR#702.51a,702.66a,702.126a,701.67a]
template substitute (action : Action) (pays : List PipPaid) : ManaSubstitute := {
  -- TODO(spelling): Format and coordinate the pays list entries, including the empty case.
  body :=
    { action := action, pays := pays }
}
