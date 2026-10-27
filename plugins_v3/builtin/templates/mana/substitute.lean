/-- "For each <pip> mana …, you may <action> rather than pay that mana": a Mana Substitute
[CR#702.51a,702.66a,702.126a,701.67a]. -/
-- [CR#702.51a,702.66a,702.126a,701.67a]
template substitute (action : Action) (pays : List PipPaid) : ManaSubstitute := {
  -- TODO(spelling): Context-dependent wording: For each <pip> mana …, you may <action> rather than pay that mana; string substitution cannot express its omitted or inflected parts.
  body :=
    { action := action, pays := pays }
}
