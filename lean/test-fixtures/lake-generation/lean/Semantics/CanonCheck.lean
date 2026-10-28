import Semantics.Definitions

/-! Stub of `Semantics.CanonCheck`: see `Semantics/Definitions.lean` here. -/

macro "check_canon_card " file:str : command =>
  `(theorem $(Lean.mkIdent (.mkSimple ("ok_" ++ file.getString))) :
      $(Lean.mkIdent (.mkSimple file.getString)) = $(Lean.mkIdent (.mkSimple file.getString)) :=
    rfl)
