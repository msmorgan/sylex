/-- "spend mana as though it were mana of <as_> [to <purpose>]" [CR#609.4b] -/
-- [CR#609.4b]
primitive spendManaAs (what : Option ColorOrColorless) (as_ : ManaMatch)
    (purpose : Option SpendPurpose) : AsThough := {
  -- TODO(spelling): Include or omit the clauses supplied by what, purpose.
  body :=
    Semantics.Primitives.AsThough.mana what as_ purpose
}
