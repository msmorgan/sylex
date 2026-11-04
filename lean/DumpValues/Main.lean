import DumpValues.Sampling
import DumpValues.Compare
import Generated.Canon
import Generated.Builtin
import Semantics.Primitives

open Lean Meta Semantics.Registry

namespace DumpValues

private def key (parts : List String) : String := (toJson parts).compress

private partial def invocations (value : Expr) (ordinal : Nat := 0) :
    MetaM (Array (Array (String × Expr)) × Expr) := do
  let ty ← inferType value
  match ty with
  | .forallE name field body bi =>
    if bi.isInstImplicit then
      return ← invocations (mkApp value (← synthInstance field)) ordinal
    if !bi.isExplicit then
      unless (← whnf field).isSort do throwError "unsupported implicit parameter {name}: {field}"
      return ← invocations (mkApp value (mkConst ``Nat)) ordinal
    if body.hasLooseBVars then throwError "dependent explicit parameter {name}"
    let default? := field.getOptParamDefault?
    let field := if default?.isSome then field.getAppArgs[0]! else field
    let mut choices := #[]
    if let some d := default? then choices := choices.push ("default", d)
    for v in ← variants field ordinal do
      let label ← render v
      if !choices.any (·.1 == label) then choices := choices.push (label, v)
    let (rest, _) ← invocations (mkApp value choices[0]!.2) (ordinal + 1)
    return (#[choices] ++ rest, value)
  | _ => return (#[], value)

private partial def canonical (name : Name) : MetaM Name := do
  if let .defnInfo info ← getConstInfo name then
    if let .abbrev := info.hints then
      if let .const target _ := info.value then return ← canonical target
  return name

private def dump (label : String) : MetaM UInt32 := do
  let dir : System.FilePath := "../docs/evidence/lean-dumps" / label
  IO.FS.createDirAll dir
  -- Invalidate an older successful summary before replacing either output file.
  IO.FS.writeFile (dir / "summary.txt") "unprintable items: pending\n"
  let env ← getEnv
  let mut records : Array (String × String) := #[]
  let mut errors : Array String := #[]
  let mut caps : Array String := #[]
  let mut cards := 0
  let mut declarations := 0
  let mut samples := 0
  let mut unsampled := 0
  let mut omitted := 0
  let mut aliases := 0
  let mut cardDispatch := 0
  let mut seen : Array (Name × Name × Name) := #[]
  let cardEntries := canonCardsExt.getState env
  for entry in cardEntries do
    let name := entry.declaration.getString!
    try
      let value := mkConst entry.declaration
      records := records.push (key ["card", name], ← render value)
      cards := cards + 1
    catch e => errors := errors.push s!"card {name}: {← e.toMessageData.toString}"
  for (_, entries) in (expectedTypeDispatchExt.getState env) do
    for entry in entries do
      let declaration ← canonical entry.declaration
      if (`Semantics.Cards).isPrefixOf declaration then
        cardDispatch := cardDispatch + 1
        continue
      let identity := (entry.bareName, entry.returnType, declaration)
      if seen.contains identity then
        aliases := aliases + 1
        continue
      seen := seen.push identity
      let entry := { entry with declaration }
      declarations := declarations + 1
      let id := key [entry.bareName.toString, entry.returnType.toString]
      try
        let info ← getConstInfo entry.declaration
        let value := mkConst entry.declaration (info.levelParams.map fun _ => Level.zero)
        let (params, value) ← invocations value
        let (indices, missing) ← match combinations (params.map (·.size)) with
          | .ok r => pure r
          | .error e => throwError "{e}"
        omitted := omitted + missing
        if missing > 0 then caps := caps.push s!"{id}: {missing} combinations omitted"
        for ix in indices do
          let args := ix.mapIdx fun i j => params[i]![j]!
          let sample := if args.all (fun a => a.1 == "default") then "unsampled"
            else key (args.toList.map (·.1))
          let k := key ["builtin", entry.bareName.toString, entry.returnType.toString, sample]
          let result ← render (mkAppN value (args.map (·.2)))
          -- Registry declarations name values; their companion holds the rules.
          let companion := entry.declaration.getPrefix ++
            Name.mkSimple (entry.declaration.getString! ++ "_definition")
          let result ← if env.contains companion then do
              pure (result ++ " rules=" ++ (← render (mkConst companion)))
            else pure result
          records := records.push (k, result)
          if sample == "unsampled" then unsampled := unsampled + 1 else samples := samples + 1
      catch e => errors := errors.push s!"{id}: {← e.toMessageData.toString}"
  let sourceEntries ← (System.FilePath.mk "../plugins/canon/cards").readDir
  let cardFiles ← sourceEntries.filterM fun entry => do
    return (entry.fileName.endsWith ".lean" || entry.fileName.endsWith ".syl") &&
      !(← entry.path.isDir)
  if cardFiles.size != cards then
    errors := errors.push (s!"card completeness mismatch: {cardFiles.size} source files, " ++
      s!"{cards} card records; check the generated canon imports")
  unless env.header.moduleNames.contains `Generated.Canon do
    errors := errors.push "card completeness mismatch: missing Generated.Canon import"
  records := records.qsort (fun a b => a.1 < b.1)
  for i in [1:records.size] do
    if records[i-1]!.1 == records[i]!.1 then
      errors := errors.push s!"duplicate key: {records[i]!.1}"
  IO.FS.writeFile (dir / "values.txt") (String.join (records.toList.map fun (k,v) =>
    (toJson [k,v]).compress ++ "\n"))
  let summary := s!"records: {records.size}\ncards: {cards}\ndeclarations: {declarations}\n" ++
    s!"unsampled: {unsampled}\nsamples: {samples}\nomitted combinations: {omitted}\n" ++
    s!"unprintable items: {errors.size}\n" ++
    s!"dispatch aliases: {aliases}\n" ++
    s!"card declarations skipped (covered by card records): {cardDispatch}\n" ++
    s!"card source files: {cardFiles.size}\n" ++
    String.join ((caps.qsort (· < ·) ++ errors.qsort (· < ·)).toList.map (· ++ "\n"))
  IO.FS.writeFile (dir / "summary.txt") summary
  IO.print summary
  return if errors.isEmpty then 0 else 2

end DumpValues

private def validateLabel (label : String) : IO Unit := do
  unless !label.isEmpty && label.toList.all (fun c => c.isAlphanum || c == '-' || c == '_') do
    throw (IO.userError "label must contain only letters, digits, hyphens or underscores")

private unsafe def run (args : List String) : IO UInt32 := do
  unless ← (System.FilePath.mk "Semantics/Registry.lean").pathExists do
    throw (IO.userError "run dumpValues from lean/")
  let root : System.FilePath := "../docs/evidence/lean-dumps"
  match args with
  | ["compare", a, b] | ["compare", a, b, "--rename", _] =>
    validateLabel a
    validateLabel b
    return ← DumpValues.compareDumps root a b (args[4]?.map System.FilePath.mk)
  | ["dump", label] =>
    validateLabel label
    enableInitializersExecution
    initSearchPath (← findSysroot)
    let imports := #[`Generated.Canon, `Generated.Builtin, `Semantics.Primitives]
    let env ← importModules (imports.map fun n => { module := n }) {} (loadExts := true)
    let (code, _, _) ← (DumpValues.dump label).toIO
      { fileName := "dumpValues", fileMap := default,
        options := ({} : Options).set `maxRecDepth (10000 : Nat) |>.set `maxHeartbeats (0 : Nat) }
      { env }
    return code
  | _ => throw (IO.userError
      "usage: dumpValues dump <label> | compare <labelA> <labelB> [--rename <file.json>]")

unsafe def main (args : List String) : IO UInt32 := do
  try run args
  catch e =>
    (← IO.getStderr).putStrLn s!"dumpValues: {e}"
    return 2
