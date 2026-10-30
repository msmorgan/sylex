#!/usr/bin/env python3
"""Regression checks for the dump reader, comparator and rename validation.

Run from lean/ after `lake build -q dumpValues`.
"""
import json
import os
from pathlib import Path
import subprocess
import tempfile

os.chdir(Path(__file__).resolve().parents[1])
ROOT = Path('../docs/evidence/lean-dumps')
EXE = Path('.lake/build/bin/dumpValues').resolve()
ROOT.mkdir(parents=True, exist_ok=True)


def invoke(args, code, contains=''):
    result = subprocess.run([str(EXE), *args], text=True, capture_output=True, timeout=60)
    assert result.returncode == code, (args, result.returncode, result.stdout, result.stderr)
    assert contains in result.stdout + result.stderr, (args, result.stdout, result.stderr)


with tempfile.TemporaryDirectory(prefix='compare-tests-', dir=ROOT) as scratch:
    scratch = Path(scratch)
    # Labels must be direct children, so put the fixture directories beside scratch.
    with tempfile.TemporaryDirectory(prefix='compare-old-', dir=ROOT) as a, \
         tempfile.TemporaryDirectory(prefix='compare-new-', dir=ROOT) as b:
        a, b = Path(a), Path(b)

        def write(path, records, errors=0):
            (path / 'summary.txt').write_text(f'unprintable items: {errors}\n')
            (path / 'values.txt').write_text(''.join(json.dumps(row) + '\n' for row in records))

        def compare(code, contains='', renames=None):
            args = ['compare', a.name, b.name]
            if renames is not None:
                rename = scratch / 'renames.json'
                rename.write_text(json.dumps(renames))
                args += ['--rename', str(rename)]
            invoke(args, code, contains)

        write(a, [('same', '1'), ('changed', '2'), ('removed', '3')])
        write(b, [('same', '1'), ('changed', '4'), ('added', '3')])
        compare(1, 'added: 1\nremoved: 1\nchanged: 1')
        write(b, [('same', '1'), ('changed', '2'), ('added', '3')])
        compare(0, 'added: 0\nremoved: 0\nchanged: 0', [('removed', 'added')])
        compare(2, 'rename collision', [('removed', 'same')])
        compare(2, 'unknown rename source', [('absent', 'added')])
        compare(2, 'duplicate rename source', [('removed', 'added'), ('removed', 'x')])
        compare(2, 'duplicate rename target', [('removed', 'x'), ('same', 'x')])
        compare(2, 'invalid dump/rename JSON', [['removed']])
        # Renames apply simultaneously, permitting swaps.
        write(a, [('a', '1'), ('b', '2')])
        write(b, [('b', '1'), ('a', '2')])
        compare(0, 'changed: 0', [('a', 'b'), ('b', 'a')])
        write(a, [('same', '1')])
        write(b, [('same', '1')])
        (a / 'summary.txt').write_text('unprintable items: 0\nomitted: 4\nexcluded: 2\n')
        (b / 'summary.txt').write_text('unprintable items: 0\nomitted: 5\nexcluded: 3\n')
        compare(1, 'summary lines changed: 2')
        compare(1, '- omitted: 4\n+ omitted: 5')
        compare(1, '- excluded: 2\n+ excluded: 3')
        (a / 'summary.txt').write_text('unprintable items: 0\n')
        (b / 'summary.txt').write_text('unprintable items: 0\nexcluded: 3\n')
        compare(1, 'summary lines changed: 1')
        (a / 'summary.txt').write_text('unprintable items: 0\nexcluded: 3\n')
        (b / 'summary.txt').write_text('unprintable items: 0\n')
        compare(1, '- excluded: 3')
        write(b, [('a', '1'), ('a', '2')])
        compare(2, 'duplicate record key')
        write(b, [('a', '1')], errors=1)
        compare(2, 'incomplete dump')
        (b / 'summary.txt').write_text('unprintable items: 0\n')
        (b / 'values.txt').write_text('not json\n')
        compare(2, 'invalid dump/rename JSON')
        invoke(['dump', '../escape'], 2, 'label must contain')
print('17 compare/CLI regression checks passed')
