import Semantics.Authoring
import Semantics.Check.Card

/-!
# Semantics.CanonCheck

`check_canon_card`, the command the generated check modules `Semantics.Cards.Check.*` are written
in. It joins the authoring side (the card a data module generated from a file) to the checker
(`check_card%`), so neither imports the other.
-/

namespace Semantics

/-- The `ok_` theorem of the canon card generated from a card file: `genCanon`
(`lean/lakefile.lean`) writes one per card into the check module over its data module. -/
syntax (name := checkCanonCard) "check_canon_card " str : command

open Lean Elab Command in
@[command_elab checkCanonCard]
def elabCheckCanonCard : CommandElab := fun stx => do
  let file := stx[1].isStrLit?.getD ""
  let some entry := (Authoring.canonCardsExt.getState (← getEnv)).find? (·.file == file)
    | throwError "No canon card was generated from {file}"
  let name := entry.declaration
  let ok := mkIdent (`_root_ ++ name.getPrefix ++ Name.mkSimple ("ok_" ++ name.getString!))
  let card ← `(term| $(mkCIdent ``Spelled.card) $(mkIdent (`_root_ ++ name)))
  elabCommand (← `(theorem $ok : $(mkCIdent ``Card.check) ($card) = [] := check_card% ($card)))

end Semantics
