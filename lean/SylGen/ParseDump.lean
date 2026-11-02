import SylGen.Parser

/-!
`sylParseDump ROOT OUTPUT`: parse every `ROOT/plugins_v3/**/*.syl` file with `SylGen.parse` (no
resolution) and write the declarations as one JSON object to `OUTPUT`, keyed by the path relative
to `ROOT`. Positions are left out. `cargo xtask dsl differential` compares the dump with
`deckmaste_dsl`'s trees.
-/

open Lean System

namespace SylGen.ParseDump

private partial def valueJson : Value → Json
  | .word _ name arguments => Json.mkObj [("word", name), ("args", Json.arr (arguments.map
      fun (key, value) => Json.mkObj [("name", match key with
        | some key => Json.str key
        | none => Json.null), ("value", valueJson value)]))]
  | .string _ text => Json.mkObj [("string", text)]
  | .integer _ text => Json.mkObj [("integer", text)]
  | .list _ items => Json.mkObj [("list", Json.arr (items.map valueJson))]
  | .record _ fields => Json.mkObj [("record", fieldsJson fields)]
  | .parameter _ name => Json.mkObj [("parameter", name)]
where
  fieldsJson (fields : Array (String × Value)) : Json :=
    Json.arr (fields.map fun (key, value) => Json.mkObj [("name", key), ("value", valueJson value)])

private partial def sortJson (s : SortRef) : Json :=
  Json.mkObj [("name", s.name), ("args", Json.arr (s.arguments.map sortJson))]

private def optional {α : Type} (f : α → Json) : Option α → Json
  | some a => f a
  | none => Json.null

private def declarationJson (d : Declaration) : Json :=
  Json.mkObj [
    ("kind", d.kind),
    ("name", d.name),
    ("parameters", Json.arr (d.parameters.map fun p => Json.mkObj [
      ("name", p.name), ("sort", sortJson p.sort), ("default", optional valueJson p.defaultValue)])),
    ("sort", sortJson d.sort),
    ("fields", valueJson.fieldsJson d.fields),
    ("constructor", optional Json.str d.constructor),
    ("cardValue", optional valueJson d.cardValue),
    ("documentation", d.documentation)]

end SylGen.ParseDump

open SylGen.ParseDump in
def main (args : List String) : IO UInt32 := do
  let [root, output] := args
    | IO.eprintln "usage: sylParseDump ROOT OUTPUT"; return 2
  let root : FilePath := root
  let paths := (← (root / "plugins_v3").walkDir).filter (·.extension == some "syl")
  let mut files : Array (String × Json) := #[]
  for path in paths do
    let relative := ((path.toString.drop (root.toString.length + 1)).toString)
    let result := match SylGen.parse relative (← IO.FS.readFile path) with
      | .ok declarations => Json.mkObj [("declarations", Json.arr (declarations.map declarationJson))]
      | .error message => Json.mkObj [("error", message)]
    files := files.push (relative, result)
  files := files.qsort (fun a b => a.1 < b.1)
  if let some parent := (output : FilePath).parent then IO.FS.createDirAll parent
  IO.FS.writeFile output ((Json.mkObj files.toList).compress ++ "\n")
  return 0
