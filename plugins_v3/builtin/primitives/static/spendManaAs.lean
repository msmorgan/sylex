/-- "spend mana as though it were mana of <as_> [to <purpose>]" [CR#609.4b] -/
-- [CR#609.4b]
primitive spendManaAs (what : Option ColorOrColorless) (as_ : ManaMatch)
    (purpose : Option SpendPurpose) : AsThough := {
  -- TODO(spelling): Context-dependent wording: spend mana as though it were mana of <as_> [to <purpose>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.AsThough.mana what as_ purpose
}
