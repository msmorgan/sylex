#!/bin/sh
set -eu
exe=$1
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
mkdir "$scratch/input"
check() {
  printf '%s\n' "$2" > "$scratch/input/bad.syl"
  if "$exe" "$scratch/input" "$scratch/out" Negative > "$scratch/log" 2>&1; then
    echo "unexpected success: $1" >&2
    exit 1
  fi
  if ! grep -F "$1" "$scratch/log" >/dev/null ||
     ! grep -E 'bad.syl:[0-9]+:[0-9]+:' "$scratch/log" >/dev/null; then
    cat "$scratch/log" >&2
    exit 1
  fi
}
check 'collides with constructor' 'template white: Color { body = blue }'
check 'Duplicate bundle declaration' 'template item: Nat { body = 1 } template item: Nat { body = 2 }'
check "parameter 'x' has sort" 'template mismatch(x: Color): Nat { body = x }'
check 'unsupported string escape' 'template bad: String { body = "\n" }'
check 'unknown word' 'template missing: Color { body = nonexistent }'
check 'negative integer at Nat' 'template negative: Nat { body = -1 }'
check 'empty call' 'template empty: Color { body = blue() }'
check 'missing argument' 'template missing: Amount { body = lit }'
check 'duplicate field' 'template duplicate: Nat { body = 1, body = 2 }'
check 'reserved parameter name' 'template bad(none: Int): Int { body = 1 }'
check 'reserved parameter name' 'template bad(true: Int): Int { body = 1 }'
check 'reserved parameter name' 'template bad(false: Int): Int { body = 1 }'
check 'unknown word' 'action bad { deed = foo(1), body = sequentially([]) }'
check 'none requires Option' 'action bad { deed = none(1), body = sequentially([]) }'
check 'Dependency cycle in bundle' 'template a: Int { body = b } template b: Int { body = a }'
check 'Dependency cycle in bundle' 'template a: Int { body = Int.b } template b: Int { body = Int.a }'
check 'Dependency cycle in bundle' 'ability mutualOne { body = [mutualTwo] } ability mutualTwo { body = [mutualOne] }'
printf 'sylGen negative cases: 17 passed\n'

printf '%s\n' 'template value: Int { body = 1 } template use: Int { body = value }' > "$scratch/input/bad.syl"
"$exe" "$scratch/input" "$scratch/custom-output" CustomRoot Custom.Generated > "$scratch/log" 2>&1
test -f "$scratch/custom-output/Custom/Generated.lean"
grep -F 'import Custom.Generated.' "$scratch/custom-output/Custom/Generated.lean" >/dev/null
grep -F '«CustomRoot».«Semantics».«Templates»' "$scratch/custom-output/Custom/Generated/"*.lean >/dev/null
printf 'sylGen configurable output: passed\n'

printf '%s\n' 'card { name = "Bad Creature", types = [creature] }' > "$scratch/input/bad.syl"
if ! "$exe" --fixture "$scratch/input" "$scratch/bad-output" BadFixture BadCards > "$scratch/log" 2>&1; then
  cat "$scratch/log" >&2
  exit 1
fi
export LEAN_PATH="$scratch/bad-output:$LEAN_PATH"
(
  cd "$scratch/bad-output"
  for data in BadCards/*.lean; do
    if grep -F 'check_card%' "$data" >/dev/null; then
      echo 'card check leaked into data module' >&2
      exit 1
    fi
    if ! lean -o "${data%.lean}.olean" "$data" > "$scratch/data.log" 2>&1; then
      cat "$scratch/data.log" >&2
      exit 1
    fi
  done
  for check in BadCards/Check/*.lean; do
    if lean "$check" > "$scratch/check.log" 2>&1; then
      echo 'invalid card check unexpectedly passed' >&2
      exit 1
    fi
    if ! grep -F 'Card semantic check failed:' "$scratch/check.log" >/dev/null; then
      cat "$scratch/check.log" >&2
      exit 1
    fi
  done
)
printf 'sylGen separate card refusal report: passed\n'
