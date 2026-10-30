import Lean

open Lean

namespace DumpValues

private def decode {α : Type} [FromJson α] (text : String) : IO α :=
  match Json.parse text >>= fromJson? with
  | .ok value => pure value
  | .error reason => throw (IO.userError s!"invalid dump/rename JSON: {reason}")

private def readRecords (dir : System.FilePath) : IO (Std.HashMap String String) := do
  let summary ← IO.FS.readFile (dir / "summary.txt")
  unless (summary.splitOn "\n").contains "unprintable items: 0" do
    throw (IO.userError s!"{dir}: incomplete dump; inspect summary.txt")
  let mut records := {}
  for line in (← IO.FS.readFile (dir / "values.txt")).splitOn "\n" do
    if line.isEmpty then continue
    let (key, value) : String × String ← decode line
    if records.contains key then throw (IO.userError s!"duplicate record key: {key}")
    records := records.insert key value
  return records

/-- Compare validated records after applying a simultaneous, one-to-one rename map. -/
def compareDumps (root : System.FilePath) (a b : String) (rename : Option System.FilePath) :
    IO UInt32 := do
  let old ← readRecords (root / a)
  let new ← readRecords (root / b)
  let mut renames : Std.HashMap String String := {}
  let mut targets : Std.HashSet String := {}
  if let some path := rename then
    let pairs : Array (String × String) ← decode (← IO.FS.readFile path)
    for (source, target) in pairs do
      unless old.contains source do throw (IO.userError s!"unknown rename source: {source}")
      if renames.contains source then throw (IO.userError s!"duplicate rename source: {source}")
      if targets.contains target then throw (IO.userError s!"duplicate rename target: {target}")
      renames := renames.insert source target
      targets := targets.insert target
  let mut renamed : Std.HashMap String String := {}
  for (key, value) in old do
    let key := renames.getD key key
    if renamed.contains key then throw (IO.userError s!"rename collision: {key}")
    renamed := renamed.insert key value
  let keys := (renamed.toArray.map (·.1) ++ new.toArray.map (·.1)).foldl
    (fun (keys : Std.HashSet String) key => keys.insert key) {}
  let keys := keys.toArray.qsort (· < ·)
  let mut added := 0
  let mut removed := 0
  let mut changed := 0
  for key in keys do
    match renamed[key]?, new[key]? with
    | none, some _ =>
      added := added + 1
      IO.println s!"added {key}"
    | some _, none =>
      removed := removed + 1
      IO.println s!"removed {key}"
    | some before, some after =>
      if before != after then
        changed := changed + 1
        IO.println s!"changed {key}"
    | none, none => pure ()
  IO.println s!"added: {added}\nremoved: {removed}\nchanged: {changed}"
  return if added + removed + changed == 0 then 0 else 1

end DumpValues
