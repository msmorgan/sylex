import Semantics.Authoring
import Semantics.Check.Card

/-!
# Semantics.CanonCheck

`check_canon_card` finds a generated card by its source file and checks its plain `Card` value
with `check_card%`. The source registry and the checker remain independent.
-/

namespace Semantics

/-- Check a registered canon card and emit its `ok_` theorem. -/
syntax (name := checkCanonCard) "check_canon_card " str : command

open Lean Elab Command in
@[command_elab checkCanonCard]
def elabCheckCanonCard : CommandElab := fun stx => do
  let file := stx[1].isStrLit?.getD ""
  let some entry := (Authoring.canonCardsExt.getState (← getEnv)).find? (·.file == file)
    | throwError "No canon card was generated from {file}"
  let name := entry.declaration
  let ok := mkIdent (`_root_ ++ name.getPrefix ++ Name.mkSimple ("ok_" ++ name.getString!))
  let card : Lean.Term := ⟨mkIdent (`_root_ ++ name)⟩
  elabCommand (← `(theorem $ok : $(mkCIdent ``Card.check) ($card) = [] := check_card% ($card)))

end Semantics
