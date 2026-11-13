//! Buffered multi-row `INSERT` statements to amortize SQLite VDBE startup,
//! parameter binding, and B-tree cursor positioning across `BATCH` rows.

use std::fmt::Write;
use std::mem;

use anyhow::Context;
use anyhow::Result;
use rusqlite::Connection;
use rusqlite::Statement;
use rusqlite::types::Null;

/// One bindable SQLite column value without heap allocation.
#[derive(Clone, Copy)]
pub(super) enum Val<'a> {
    Null,
    Int(i64),
    Bool(bool),
    Text(&'a str),
    Static(&'static str),
}

/// Buffers rows of `COLS` columns and flushes them in chunks of `BATCH` via a
/// multi-row `INSERT`, falling back to a 1-row `INSERT` for the final `< BATCH`
/// remainder (and to pinpoint the offending row when a batch flush fails).
pub(super) struct BatchInsert<'t, const COLS: usize, const BATCH: usize> {
    table: &'static str,
    batch: Statement<'t>,
    single: Statement<'t>,
    /// Owned string storage for borrowed text whose lifetime ends before flush.
    /// Static strings, integers, booleans, and nulls need no copy.
    arena: String,
    rows: Vec<[Slot; COLS]>,
}

#[derive(Clone, Copy)]
enum Slot {
    Null,
    Int(i64),
    Bool(bool),
    StaticText(&'static str),
    ArenaText { start: usize, len: usize },
}

impl<'t, const COLS: usize, const BATCH: usize> BatchInsert<'t, COLS, BATCH> {
    pub(super) fn prepare(tx: &'t Connection, table: &'static str) -> Result<Self> {
        let one_tuple = |offset: usize| {
            let vars: Vec<String> = (1..=COLS).map(|c| format!("?{}", offset + c)).collect();
            format!("({})", vars.join(", "))
        };
        let single_sql = format!("INSERT INTO {table} VALUES {}", one_tuple(0));
        let tuples: Vec<String> = (0..BATCH).map(|r| one_tuple(r * COLS)).collect();
        let batch_sql = format!("INSERT INTO {table} VALUES {}", tuples.join(", "));
        let batch = tx
            .prepare(&batch_sql)
            .with_context(|| format!("preparing batch insert into `{table}`"))?;
        let single = tx
            .prepare(&single_sql)
            .with_context(|| format!("preparing single insert into `{table}`"))?;
        Ok(Self {
            table,
            batch,
            single,
            arena: String::with_capacity(64 * 1024),
            rows: Vec::with_capacity(BATCH),
        })
    }

    #[inline]
    pub(super) fn push(&mut self, values: [Val<'_>; COLS]) -> Result<()> {
        let mut row = [Slot::Null; COLS];
        for (i, val) in values.into_iter().enumerate() {
            row[i] = match val {
                Val::Null => Slot::Null,
                Val::Int(n) => Slot::Int(n),
                Val::Bool(b) => Slot::Bool(b),
                Val::Static(s) => Slot::StaticText(s),
                Val::Text(s) => {
                    let start = self.arena.len();
                    let len = s.len();
                    self.arena.push_str(s);
                    Slot::ArenaText { start, len }
                }
            };
        }
        self.rows.push(row);
        if self.rows.len() == BATCH {
            self.flush_full()?;
        }
        Ok(())
    }

    fn flush_full(&mut self) -> Result<()> {
        debug_assert_eq!(self.rows.len(), BATCH);
        let mut idx: usize = 1;
        for row in &self.rows {
            for &slot in row {
                bind_slot(&mut self.batch, idx, slot, &self.arena).with_context(|| {
                    format!("binding parameter {idx} for batch insert into `{}`", self.table)
                })?;
                idx += 1;
            }
        }
        match self.batch.raw_execute() {
            Ok(_) => {
                self.rows.clear();
                self.arena.clear();
                Ok(())
            }
            Err(batch_err) => {
                // SQLite rolled back the failed multi-row statement without
                // inserting any of `self.rows`; replay them one by one so the
                // error names the exact offending row instead of the row that
                // happened to fill the batch.
                self.flush_single()?;
                Err(batch_err)
                    .with_context(|| format!("executing batch insert into `{}`", self.table))
            }
        }
    }

    pub(super) fn finish(&mut self) -> Result<()> {
        debug_assert!(self.rows.len() < BATCH);
        if !self.rows.is_empty() {
            self.flush_single()?;
        }
        Ok(())
    }

    fn flush_single(&mut self) -> Result<()> {
        let rows = mem::take(&mut self.rows);
        let arena = mem::take(&mut self.arena);
        for row in &rows {
            for (idx, &slot) in (1_usize..).zip(row.iter()) {
                bind_slot(&mut self.single, idx, slot, &arena).with_context(|| {
                    format!(
                        "binding column {idx} for `{}` row {}",
                        self.table,
                        format_row(row, &arena)
                    )
                })?;
            }
            self.single.raw_execute().with_context(|| {
                format!("inserting into `{}` row {}", self.table, format_row(row, &arena))
            })?;
        }
        Ok(())
    }
}

#[inline]
fn bind_slot(
    stmt: &mut Statement<'_>,
    idx: usize,
    slot: Slot,
    arena: &str,
) -> rusqlite::Result<()> {
    match slot {
        Slot::Null => stmt.raw_bind_parameter(idx, Null),
        Slot::Int(n) => stmt.raw_bind_parameter(idx, n),
        Slot::Bool(b) => stmt.raw_bind_parameter(idx, b),
        Slot::StaticText(s) => stmt.raw_bind_parameter(idx, s),
        Slot::ArenaText { start, len } => stmt.raw_bind_parameter(idx, &arena[start..start + len]),
    }
}

fn format_row(row: &[Slot], arena: &str) -> String {
    let mut out = String::from("[");
    for (i, &slot) in row.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        match slot {
            Slot::Null => out.push_str("NULL"),
            Slot::Int(n) => {
                let _ = write!(out, "{n}");
            }
            Slot::Bool(b) => {
                let _ = write!(out, "{b}");
            }
            Slot::StaticText(s) => {
                let _ = write!(out, "{s:?}");
            }
            Slot::ArenaText { start, len } => {
                let _ = write!(out, "{:?}", &arena[start..start + len]);
            }
        }
    }
    out.push(']');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flushes_batches_and_remainder_in_order() -> Result<()> {
        let mut db = Connection::open_in_memory()?;
        db.execute_batch(
            "CREATE TABLE items (
                id INTEGER PRIMARY KEY,
                flag INTEGER NOT NULL,
                label TEXT,
                tag TEXT NOT NULL
            );",
        )?;
        let tx = db.transaction()?;
        let mut insert: BatchInsert<'_, 4, 3> = BatchInsert::prepare(&tx, "items")?;
        for i in 0..8_i64 {
            let owned = format!("row-{i}");
            insert.push([
                Val::Int(i),
                Val::Bool(i % 2 == 0),
                if i == 5 {
                    Val::Null
                } else {
                    Val::Text(&owned)
                },
                Val::Static("kind"),
            ])?;
        }
        insert.finish()?;
        drop(insert);
        tx.commit()?;

        let rows: Vec<(i64, bool, Option<String>, String)> = db
            .prepare("SELECT id, flag, label, tag FROM items ORDER BY id")?
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
            .collect::<rusqlite::Result<_>>()?;
        assert_eq!(rows.len(), 8);
        assert_eq!(rows[0], (0, true, Some("row-0".into()), "kind".into()));
        assert_eq!(rows[5], (5, false, None, "kind".into()));
        assert_eq!(rows[7], (7, false, Some("row-7".into()), "kind".into()));
        Ok(())
    }

    #[test]
    fn batch_failure_attributes_the_offending_buffered_row() -> Result<()> {
        let mut db = Connection::open_in_memory()?;
        db.execute_batch(
            "CREATE TABLE positive (
                unit INTEGER NOT NULL,
                n INTEGER NOT NULL CHECK (n > 0),
                note TEXT NOT NULL
            );",
        )?;
        let tx = db.transaction()?;
        let mut insert: BatchInsert<'_, 3, 4> = BatchInsert::prepare(&tx, "positive")?;
        insert.push([Val::Int(10), Val::Int(1), Val::Static("ok")])?;
        // Bad row at unit 11, buffered until unit 13 fills the batch of 4.
        insert.push([Val::Int(11), Val::Int(0), Val::Text("bad-row")])?;
        insert.push([Val::Int(12), Val::Int(2), Val::Static("ok")])?;
        let err = insert
            .push([Val::Int(13), Val::Int(3), Val::Static("trigger")])
            .expect_err("CHECK violation must fail the flush");
        let chain = format!("{err:#}");
        assert!(
            chain.contains("inserting into `positive` row [11, 0, \"bad-row\"]"),
            "unexpected error chain: {chain}"
        );
        assert!(
            chain.contains("CHECK constraint failed"),
            "missing SQLite message in error chain: {chain}"
        );
        Ok(())
    }
}
