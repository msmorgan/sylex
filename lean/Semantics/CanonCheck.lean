import Semantics.Registry
import Semantics.Check.Card

/-!
# Semantics.CanonCheck

`check_canon_card` finds a generated card by its source file and checks its plain `Card` value
with the `check_card` command. The source registry and the checker remain independent.
-/

namespace Semantics

/-- Check a registered canon card and emit its `ok_` theorem. -/
syntax (name := checkCanonCard) "check_canon_card " str : command

open Lean Elab Command in
@[command_elab checkCanonCard]
def elabCheckCanonCard : CommandElab := fun stx => do
  let file := stx[1].isStrLit?.getD ""
  let some entry := (Registry.canonCardsExt.getState (← getEnv)).find? (·.file == file)
    | throwError "No canon card was generated from {file}"
  let name := entry.declaration
  elabCommand (← `(check_card $(mkIdent (`_root_ ++ name))))

end Semantics
