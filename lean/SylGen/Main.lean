import SylGen.Emit
import Lean.Util.SCC

open Lean Meta System
namespace SylGen

private def hashName (names : List Name) : String :=
  let text := String.intercalate ":" (names.map Name.toString)
  let hash := text.toUTF8.foldl (fun (h : UInt64) b => (h ^^^ b.toUInt64) * 1099511628211)
    14695981039346656037
  "C" ++ toString hash

private def writeChanged (path : FilePath) (text : String) : IO Unit := do
  if ← path.pathExists then
    if (← IO.FS.readFile path) == text then return
  if let some parent := path.parent then IO.FS.createDirAll parent
  IO.FS.writeFile path text

private def generate (input output : FilePath) (root : Name) (modulePrefix : String) (fallback : Registry) : MetaM Unit := do
  let paths := (← input.walkDir).filter (·.extension == some "syl")
    |>.qsort (fun a b => a.toString < b.toString)
  let mut ds := #[]
  for path in paths do
    let source ← IO.FS.readFile path
    match parse path.toString source with
    | .error error => throwError error
    | .ok declarations =>
      for d in declarations do
        let sort ← normalizeSort d.pos d.sort
        let parameters ← d.parameters.mapM fun p => do
          return { p with sort := ← normalizeSort p.pos p.sort }
        ds := ds.push { d with sort, parameters }
  -- Register signatures before resolving bodies so only references at the expected sort
  -- contribute dependency edges.
  let catalog := fun d => d.kind ∈ ["counter", "designation", "subtype", "card_type", "relation"]
  ds := ds.filter catalog ++ ds.filter (fun d => d.kind == "primitive") ++
    ds.filter (fun d => !catalog d && d.kind != "primitive")
  let ctors ← constructors
  ds := ds.map fun d => { d with parameters := d.parameters.map fun p =>
    if (ctors.contains (leanWord p.name, p.sort.text)) then
      { p with binderName := leanWord p.name ++ "Value" }
    else p }
  let registry ← loadDeclarations root ds fallback
  let mut outputs := #[]
  let mut errors : Array String := #[]
  for d in ds do
    try
      -- Temporary exception for the separately owned Ring modeling correction. Compare
      -- canonical paths, so another bundle cannot opt in by copying the basename.
      let legacyFile := input / "builtin/definitions/keyword_action/theRingTemptsYou.syl"
      let legacyScope ← try
        pure ((← IO.FS.realPath d.pos.file) == (← IO.FS.realPath legacyFile))
      catch _ => pure false
      let (out, _) ← ((emit d).run {
        constructors := ctors
        declarations := registry
        fallback
        legacyScope
        parameters := d.parameters }).run {}
      outputs := outputs.push out
    catch error =>
      errors := errors.push (d.pos.error (← error.toMessageData.toString))
  unless errors.isEmpty do throwError "{String.intercalate "\n" errors.toList}"
  let owners := outputs.foldl (init := ({} : Std.HashMap Name Name)) fun map out =>
    (map.insert out.name out.name).insert (out.name ++ `label) out.name
      |>.insert (out.name ++ `deed) out.name
  let byName := outputs.foldl (init := ({} : Std.HashMap Name Output)) fun m o => m.insert o.name o
  let dependencies := fun (name : Name) =>
    (byName[name]!.dependencies.toList.filterMap (owners[·]?)).eraseDups.filter (· != name)
  let registered := ds.filter catalog |>.map fun d => registry[(d.name, d.sort.text)]!.constant
  let bodyDependencies := fun (name : Name) =>
    byName[name]!.dependencies.toList.filter fun dep =>
      byName.contains dep && !registered.contains dep
  for component in Lean.SCC.scc (outputs.toList.map (·.name)) bodyDependencies do
    if component.length > 1 || (bodyDependencies component.head!).contains component.head! then
      for d in ds do
        if registry[(d.name, d.sort.text)]!.constant == component.head! then
          throwError d.pos.error s!"Dependency cycle in bundle: {component}"
  let components := Lean.SCC.scc (outputs.toList.map (·.name)) dependencies
  let mut modules : Std.HashMap Name String := {}
  for component in components do
    let module := hashName (component.mergeSort (fun a b => a.toString ≤ b.toString))
    for name in component do modules := modules.insert name module
  let mut expected : Array FilePath := #[]
  let mut umbrella := ""
  let mut builtinImports := ""
  let mut cardImports := ""
  let mut cardChecks := ""
  let mut checks : Array FilePath := #[]
  for component in components do
    let module := modules[component.head!]!
    let component := (Lean.SCC.scc component fun name =>
      (bodyDependencies name).filter (component.contains ·)).flatten
    let mut imports := if fallback.isEmpty then "import Semantics.Rules\nimport Semantics.CallerScope\n"
      else "import Semantics.Definitions\n"
    imports := imports ++ "import SylGen.Registration\n"
    if component.any (fun n => byName[n]!.checksRules) then
      imports := imports ++ "import Semantics.Check.Rules\n"
    let deps := (component.flatMap dependencies).filterMap (modules[·]?)
      |>.filter (· != module) |>.eraseDups
    for dep in deps do imports := imports ++ s!"import {modulePrefix}.{dep}\n"
    let text := imports ++ "\n" ++ String.intercalate "\n" (component.map fun n => byName[n]!.prelude) ++
      "\n" ++ String.intercalate "\n" (component.map fun n => byName[n]!.text)
    let path := output / (modulePrefix.replace "." "/") / (module ++ ".lean")
    expected := expected.push path
    writeChanged path text
    umbrella := umbrella ++ s!"import {modulePrefix}.{module}\n"
    if component.any (fun n => (`Semantics.Cards).isPrefixOf n) then
      cardImports := cardImports ++ s!"import {modulePrefix}.{module}\n"
    else builtinImports := builtinImports ++ s!"import {modulePrefix}.{module}\n"
    let checkText := String.intercalate "\n" ((component.map fun n => byName[n]!.check).filter (!·.isEmpty))
    if !checkText.isEmpty then
      let path := output / (modulePrefix.replace "." "/") / "Check" / (module ++ ".lean")
      checks := checks.push path
      writeChanged path (s!"import {modulePrefix}.{module}\nimport Semantics.Check.Card\n\n" ++ checkText)
      umbrella := umbrella ++ s!"import {modulePrefix}.Check.{module}\n"
      cardChecks := cardChecks ++ s!"import {modulePrefix}.Check.{module}\n"
  let dir := output / (modulePrefix.replace "." "/")
  if root.isAnonymous then
    for name in ["Builtin", "Canon", "CanonChecks"] do
      expected := expected.push (dir / (name ++ ".lean"))
  if ← dir.pathExists then
    for entry in ← dir.readDir do
      if entry.path.extension == some "lean" && !expected.contains entry.path then
        IO.FS.removeFile entry.path
  let checkDir := dir / "Check"
  if ← checkDir.pathExists then
    for entry in ← checkDir.readDir do
      if entry.path.extension == some "lean" && !checks.contains entry.path then
        IO.FS.removeFile entry.path
  writeChanged (output / ((modulePrefix.replace "." "/") ++ ".lean")) umbrella
  if root.isAnonymous then
    for (name, text) in [("Builtin", builtinImports), ("Canon", cardImports), ("CanonChecks", cardChecks)] do
      writeChanged (dir / (name ++ ".lean")) text
  IO.println s!"sylGen: {ds.size} declarations, {components.length} modules"

private partial def canonical (name : Name) : MetaM Name := do
  if let .defnInfo info ← getConstInfo name then
    if let .abbrev := info.hints then
      if let .const target _ := info.value then return ← canonical target
  return name

private unsafe def run (args : List String) : IO UInt32 := do
  let (fixture, args) := if args.head? == some "--fixture" then (true, args.tail!) else (false, args)
  let args := if args.length == 3 then args ++ ["SylFixtures"] else args
  let [input, output, root, modulePrefix] := args
    | throw (IO.userError "usage: sylGen [--fixture] CORPUS OUTPUT ROOT_NAMESPACE [MODULE_PREFIX]")
  enableInitializersExecution
  initSearchPath (← findSysroot)
  let imports := if fixture then #[`SylGen.Tests.Registry] else #[`SylGen.Environment]
  let env ← importModules (imports.map fun module => { module }) {} (loadExts := true)
  let entries ← if fixture then do
      let getEntries ← IO.ofExcept <| env.evalConst
        (Environment → Array (String × String × Name)) {} `SylGen.TestRegistry.entries
      pure (getEntries env)
    else pure #[]
  let task : MetaM Unit := do
    let mut fallback := {}
    for (name, sort, constant) in entries do
      let sig ← signature (← canonical constant)
      fallback := fallback.insert (name, sort) sig
    generate input output root.toName modulePrefix fallback
  discard <| task.toIO {
    fileName := "sylGen"
    fileMap := default
    options := ({} : Options).set `maxRecDepth (10000 : Nat) |>.set `maxHeartbeats (0 : Nat) } { env }
  return 0

unsafe def main (args : List String) : IO UInt32 := do
  try run args
  catch error =>
    (← IO.getStderr).putStrLn (toString error)
    return 1

end SylGen

unsafe def main := SylGen.main
