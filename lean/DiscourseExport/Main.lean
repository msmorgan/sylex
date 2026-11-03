import DiscourseExport.Bindings
import DiscourseExport.Fixtures
import Semantics.Registry

/-!
# discourseExport

Exports Lean's discourse bindings per canon card and the closed checker equations of
`Semantics.Proofs` as an oracle for the Rust `discourse` crate. See `DiscourseExport/README.md`.
Disposable: deleted with Lean (`lean-retirement`).
-/

open Lean Meta Semantics.Registry

namespace DiscourseExport

private def line (j : Json) : String := j.compress ++ "\n"

private def refusalKind (r : Semantics.Refusal) : String :=
  let text := toString (repr r)
  let head := (text.takeWhile (fun c => c != ' ' && c != '\n')).toString
  ((head.splitOn ".").getLast!)

private unsafe def exportBindings (dir : System.FilePath) : MetaM (Array String × String) := do
  let env ← getEnv
  let mut records : Array (String × String) := #[]
  let mut errors : Array String := #[]
  let mut totals : Alignment := {}
  let mut cards := 0
  let mut flagged := 0
  let mut withBindings := 0
  let pairType := mkApp2 (mkConst ``Prod [Level.zero, Level.zero]) (mkConst ``Semantics.Card)
    (mkApp (mkConst ``List [Level.zero]) (mkConst ``USize))
  let mut declared : Std.HashSet Name := {}
  for entry in canonCardsExt.getState env do
    let name := entry.declaration.getString!
    -- The proofs also import the `sylGen` test fixtures' cards (`SylFixture.…`).
    unless (`Semantics.Cards).isPrefixOf entry.declaration do continue
    if declared.contains entry.declaration then continue
    declared := declared.insert entry.declaration
    try
      let (term, positions, slots) ← walk (mkConst entry.declaration)
      let (card, addresses) ← withOptions (fun o => o.set `compiler.extract_closed false) <|
        evalExpr (Semantics.Card × List USize) pairType term (safety := .unsafe)
      let (refusals, events) ← Semantics.Trace.collect fun _ => Semantics.Card.check card
      let a := align env positions slots addresses events
      if (← IO.getEnv "DISCOURSE_DEBUG") == some name then
        for (slot, addr) in slots.toList.zip addresses do
          IO.eprintln s!"pos {slot} {addr} {positions[slot]!.type} {positions[slot]!.ctor} {(toJson positions[slot]!.path).compress}"
        for e in events do IO.eprintln (reprStr e)
      cards := cards + 1
      if !a.bindings.isEmpty then withBindings := withBindings + 1
      flagged := flagged + (a.bindings.filter (·.flagDefect)).size
      totals := { totals with
        bindings := totals.bindings ++ a.bindings, frames := totals.frames + a.frames
        aligned := totals.aligned + a.aligned, scalars := totals.scalars + a.scalars
        scalarsByOrder := totals.scalarsByOrder + a.scalarsByOrder
        mentionsByOrder := totals.mentionsByOrder + a.mentionsByOrder
        synthesized := totals.synthesized + a.synthesized
        rewritten := totals.rewritten + a.rewritten
        mintsViaSynthesized := totals.mintsViaSynthesized + a.mintsViaSynthesized
        outOfRegion := totals.outOfRegion + a.outOfRegion }
      for f in a.failures.toList.eraseDups do errors := errors.push s!"card {name}: {f}"
      let kinds := (refusals.map refusalKind).eraseDups.toArray.qsort (· < ·)
      records := records.push (name, line <| Json.mkObj [("card", toJson name),
        ("refusals", toJson kinds),
        ("bindings", Json.arr (a.bindings.map (·.toJson))),
        ("alignmentFailures", toJson a.failures.toList.eraseDups)])
    catch e => errors := errors.push s!"card {name}: {← e.toMessageData.toString}"
  records := records.qsort (fun a b => a.1 < b.1)
  for i in [1:records.size] do
    if records[i-1]!.1 == records[i]!.1 then errors := errors.push s!"duplicate card: {records[i]!.1}"
  let sourceEntries ← (System.FilePath.mk "../plugins_v3/canon/cards").readDir
  let cardFiles ← sourceEntries.filterM fun entry => do
    return (entry.fileName.endsWith ".lean" || entry.fileName.endsWith ".syl") &&
      !(← entry.path.isDir)
  if cardFiles.size != cards then
    errors := errors.push s!"card completeness mismatch: {cardFiles.size} source files, {cards} cards"
  IO.FS.writeFile (dir / "bindings.jsonl") (String.join (records.toList.map (·.2)))
  let summary := s!"cards exported: {cards}\ncards with bindings: {withBindings}\n" ++
    s!"bindings: {totals.bindings.size}\nFLAG DEFECT bindings: {flagged}\n" ++
    s!"frames: {totals.frames}\naligned by address: {totals.aligned}\n" ++
    s!"aligned scalar nodes: {totals.scalars}\n" ++
    s!"  of which placed by reading order: {totals.scalarsByOrder}\n" ++
    s!"    of which mentions: {totals.mentionsByOrder}\n" ++
    s!"  of which over a rewritten node: {totals.rewritten}\n" ++
    s!"synthesized frames: {totals.synthesized}\n" ++
    s!"mints in synthesized frames: {totals.mintsViaSynthesized}\n" ++
    s!"frames outside their parent's node: {totals.outOfRegion}\n" ++
    s!"card errors and alignment failures: {errors.size}\n"
  return (errors, summary)

private unsafe def exportFixtures (dir : System.FilePath) : MetaM (Array String × String) := do
  let catalog ← IO.FS.readFile "../docs/semantics-spec-catalog.md"
  let flagged := flaggedNames catalog
  let (fixtures, checklist) ← collectFixtures
  let mut seen : Std.HashSet String := {}
  let mut fixtureLines := #[]
  let mut flaggedFixtures := 0
  for f in fixtures do
    let cname := catalogName f.module f.theoremName
    let flag := flagged.contains cname
    if flag then flaggedFixtures := flaggedFixtures + 1
    seen := seen.insert cname
    fixtureLines := fixtureLines.push <| line <| Json.mkObj [("theorem", toJson cname),
      ("declaration", toJson f.theoremName.toString), ("entry", toJson f.entry.toString),
      ("args", Json.arr (f.args.map fun (n, v) => Json.arr #[toJson n, toJson v])),
      ("accepted", toJson f.accepted), ("refusals", toJson f.refusals),
      ("flagDefect", toJson flag)]
  let mut checklistLines := #[]
  let mut flaggedChecklist := 0
  let mut reasons : Std.HashMap String Nat := {}
  for c in checklist do
    let cname := catalogName c.module c.theoremName
    let flag := flagged.contains cname
    if flag then flaggedChecklist := flaggedChecklist + 1
    seen := seen.insert cname
    let reason := if c.reason.startsWith "not an entry point" then "not an entry point"
      else if c.reason.startsWith "unrenderable" then "unrenderable" else c.reason
    reasons := reasons.insert reason (reasons.getD reason 0 + 1)
    checklistLines := checklistLines.push <| line <| Json.mkObj [("theorem", toJson cname),
      ("declaration", toJson c.theoremName.toString), ("reason", toJson c.reason),
      ("flagDefect", toJson flag)]
  let missing := flagged.toList.filter (!seen.contains ·)
  IO.FS.writeFile (dir / "fixtures.jsonl") (String.join fixtureLines.toList)
  IO.FS.writeFile (dir / "checklist.jsonl") (String.join checklistLines.toList)
  let errors := (missing.toArray.qsort (· < ·)).map
    (s!"catalog FLAG DEFECT name matches no theorem: {·}")
  let reasonLines := (reasons.toArray.qsort (fun a b => a.1 < b.1)).map fun (r, n) =>
    s!"  {r}: {n}\n"
  let summary := s!"fixtures: {fixtures.size}\nFLAG DEFECT fixtures: {flaggedFixtures}\n" ++
    s!"checklist: {checklist.size}\nFLAG DEFECT checklist entries: {flaggedChecklist}\n" ++
    String.join reasonLines.toList ++
    s!"catalog FLAG DEFECT names: {flagged.size}\n"
  return (errors, summary)

unsafe def exportAll (label : String) (what : String) : MetaM UInt32 := do
  let dir : System.FilePath := "../docs/evidence/lean-discourse" / label
  IO.FS.createDirAll dir
  IO.FS.writeFile (dir / "summary.txt") "errors: pending\n"
  let mut errors := #[]
  let mut summary := ""
  if what == "all" || what == "fixtures" then
    let (e, s) ← exportFixtures dir
    errors := errors ++ e
    summary := summary ++ s
  if what == "all" || what == "bindings" then
    let (e, s) ← exportBindings dir
    errors := errors ++ e
    summary := summary ++ s
  summary := summary ++ s!"errors: {errors.size}\n" ++
    String.join ((errors.qsort (· < ·)).toList.map (· ++ "\n"))
  IO.FS.writeFile (dir / "summary.txt") summary
  IO.print summary
  return if errors.isEmpty then 0 else 2

end DiscourseExport

private def validateLabel (label : String) : IO Unit := do
  unless !label.isEmpty && label.toList.all (fun c => c.isAlphanum || c == '-' || c == '_') do
    throw (IO.userError "label must contain only letters, digits, hyphens or underscores")

private unsafe def run (args : List String) : IO UInt32 := do
  unless ← (System.FilePath.mk "Semantics/Registry.lean").pathExists do
    throw (IO.userError "run discourseExport from lean/")
  let (what, label) ← match args with
    | [label] => pure ("all", label)
    | ["bindings", label] => pure ("bindings", label)
    | ["fixtures", label] => pure ("fixtures", label)
    | _ => throw (IO.userError "usage: discourseExport [bindings|fixtures] <label>")
  validateLabel label
  enableInitializersExecution
  initSearchPath (← findSysroot)
  let imports := #[`Generated.Canon, `Semantics.Proofs, `Semantics.Check, `DiscourseExport.Paths]
  let env ← importModules (imports.map fun n => { module := n }) {} (loadExts := true)
  let (code, _, _) ← (DiscourseExport.exportAll label what).toIO
    { fileName := "discourseExport", fileMap := default,
      options := ({} : Options).set `maxRecDepth (10000 : Nat) |>.set `maxHeartbeats (0 : Nat) }
    { env }
  return code

unsafe def main (args : List String) : IO UInt32 := do
  try run args
  catch e =>
    (← IO.getStderr).putStrLn s!"discourseExport: {e}"
    return 2
