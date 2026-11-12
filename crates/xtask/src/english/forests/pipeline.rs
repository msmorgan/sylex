//! Parallel workers computing units in a bounded window, streaming them to
//! one writer thread that receives them in unit order.
//!
//! Workers on the Rayon pool (`rayon::broadcast`) claim units from a shared
//! counter, so units are claimed in increasing order, and send each result
//! over a channel. The writer, a thread of its own, buffers results that
//! arrive early and hands them to its consumer from unit 0 upward. A unit is
//! computed only once it is inside the window: fewer than `window` units
//! past the next one the writer has yet to hand over. The window bounds the
//! results held at once; the channel itself is unbounded, so a send never
//! blocks.

use std::any::Any;
use std::collections::BTreeMap;
use std::panic::AssertUnwindSafe;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc;

use anyhow::Context;
use anyhow::Result;

/// Computes `work` for each of `units` units in parallel and passes the
/// results to `write` in unit order, through [`Ordered::next`]; returns what
/// `write` returns.
///
/// An error or panic in `work` (with `describe` of its unit as context), or
/// in `write`, is the returned error, and stops the other side: workers
/// claim no further unit, and `write` sees the error from `next`.
pub(super) fn run<T, R>(
    units: usize,
    window: usize,
    describe: impl Fn(usize) -> String + Sync,
    work: impl Fn(usize) -> Result<T> + Sync,
    write: impl FnOnce(&mut Ordered<'_, T>) -> Result<R> + Send,
) -> Result<R>
where
    T: Send,
    R: Send,
{
    assert!(window > 0, "a pipeline window holds at least one unit");
    let gate = Gate {
        state: Mutex::new(GateState {
            released: 0,
            stopped: false,
        }),
        changed: Condvar::new(),
        window,
    };
    let gate = &gate;
    let claimed = AtomicUsize::new(0);
    let (sender, receiver) = mpsc::channel();
    std::thread::scope(|scope| {
        let writer = scope.spawn(move || {
            let mut ordered = Ordered {
                receiver,
                gate,
                pending: BTreeMap::new(),
                next: 0,
                units,
            };
            write(&mut ordered)
        });
        rayon::broadcast(|_| {
            let _stop = StopOnPanic(gate);
            // Why this cannot deadlock: let `c` be the lowest unit the writer
            // has yet to receive in order (`Ordered::next`). Units are claimed
            // in increasing order, and every unit below `c` was claimed, so
            // `c` is claimed or is the next claim. Its worker passes `admit`
            // at once (`c < c + window`), and computing and sending never
            // wait on anything, so `c` reaches the writer, which hands it
            // over and moves the window. Only units at `c + window` or above
            // ever wait, and they wait for the writer alone, never for one
            // another.
            loop {
                let unit = claimed.fetch_add(1, Ordering::Relaxed);
                if unit >= units || !gate.admit(unit) {
                    break;
                }
                let result = std::panic::catch_unwind(AssertUnwindSafe(|| work(unit)))
                    .unwrap_or_else(|panic| {
                        Err(anyhow::anyhow!("panicked: {}", message(panic.as_ref())))
                    })
                    .with_context(|| describe(unit));
                // A failed send means the writer is gone; it reports why.
                if sender.send((unit, result)).is_err() {
                    break;
                }
            }
        });
        // The writer sees the channel close if the workers stop short of the
        // last unit.
        drop(sender);
        writer
            .join()
            .map_err(|panic| anyhow::anyhow!("the writer panicked: {}", message(panic.as_ref())))?
    })
}

/// The window of units workers may compute.
struct Gate {
    state: Mutex<GateState>,
    changed: Condvar,
    window: usize,
}

struct GateState {
    /// How many units the writer has handed over.
    released: usize,
    /// Set once the writer is gone or a worker panicked outside its unit.
    stopped: bool,
}

impl Gate {
    /// Waits until `unit` is inside the window; false if the pipeline stopped.
    fn admit(&self, unit: usize) -> bool {
        let mut state = self
            .state
            .lock()
            .expect("no thread panics holding the gate");
        while !state.stopped && unit >= state.released + self.window {
            state = self
                .changed
                .wait(state)
                .expect("no thread panics holding the gate");
        }
        !state.stopped
    }

    fn release(&self, released: usize) {
        self.state
            .lock()
            .expect("no thread panics holding the gate")
            .released = released;
        self.changed.notify_all();
    }

    fn stop(&self) {
        self.state
            .lock()
            .expect("no thread panics holding the gate")
            .stopped = true;
        self.changed.notify_all();
    }
}

/// Stops the gate if its thread unwinds, so no worker waits for a writer
/// that will never move the window.
struct StopOnPanic<'g>(&'g Gate);

impl Drop for StopOnPanic<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.0.stop();
        }
    }
}

/// The workers' results, in unit order, on the writer thread.
pub(super) struct Ordered<'g, T> {
    receiver: mpsc::Receiver<(usize, Result<T>)>,
    gate: &'g Gate,
    /// Results that arrived before an earlier unit's.
    pending: BTreeMap<usize, T>,
    /// The next unit to hand over.
    next: usize,
    units: usize,
}

impl<T> Ordered<'_, T> {
    /// The next unit and its result; `None` once every unit is handed over.
    pub(super) fn next(&mut self) -> Result<Option<(usize, T)>> {
        if self.next == self.units {
            return Ok(None);
        }
        let result = loop {
            if let Some(result) = self.pending.remove(&self.next) {
                break result;
            }
            let (unit, result) = self.receiver.recv().map_err(|_| {
                anyhow::anyhow!("the workers stopped before computing unit {}", self.next)
            })?;
            self.pending.insert(unit, result?);
        };
        let unit = self.next;
        self.next += 1;
        self.gate.release(self.next);
        Ok(Some((unit, result)))
    }
}

/// Whether the writer finished, returned early or panicked, workers stop
/// claiming units once it is gone.
impl<T> Drop for Ordered<'_, T> {
    fn drop(&mut self) {
        self.gate.stop();
    }
}

/// A panic's message.
fn message(panic: &(dyn Any + Send)) -> String {
    if let Some(s) = panic.downcast_ref::<&str>() {
        (*s).to_owned()
    } else if let Some(s) = panic.downcast_ref::<String>() {
        s.clone()
    } else {
        "a panic with no message".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// With a window of one unit and work that finishes in reverse order
    /// of claiming, every unit arrives, in order.
    #[test]
    fn units_arrive_in_order_through_the_smallest_window() {
        let units = 200;
        let written = run(
            units,
            1,
            |unit| format!("unit {unit}"),
            |unit| {
                std::thread::sleep(std::time::Duration::from_micros(
                    ((units - unit) % 7) as u64 * 50,
                ));
                Ok(unit * 2)
            },
            |ordered| {
                let mut written = Vec::new();
                while let Some((unit, value)) = ordered.next()? {
                    assert_eq!(value, unit * 2);
                    written.push(unit);
                }
                Ok(written)
            },
        )
        .unwrap();
        assert_eq!(written, (0..units).collect::<Vec<_>>());
    }

    #[test]
    fn a_worker_s_error_is_the_result() {
        let error = run(
            1000,
            4,
            |unit| format!("unit {unit}"),
            |unit| {
                anyhow::ensure!(unit != 300, "no unit 300");
                Ok(unit)
            },
            |ordered| {
                while ordered.next()?.is_some() {}
                Ok(())
            },
        )
        .unwrap_err();
        assert_eq!(format!("{error:#}"), "unit 300: no unit 300");
    }

    #[test]
    fn a_worker_s_panic_is_the_result() {
        let error = run(
            1000,
            4,
            |unit| format!("unit {unit}"),
            |unit| {
                assert!(unit != 300, "no unit 300");
                Ok(unit)
            },
            |ordered| {
                while ordered.next()?.is_some() {}
                Ok(())
            },
        )
        .unwrap_err();
        assert_eq!(format!("{error:#}"), "unit 300: panicked: no unit 300");
    }

    /// The writer stops after unit 10 while the workers wait on the window:
    /// they stop too, and its error is the result.
    #[test]
    fn the_writer_s_error_and_panic_are_the_result() {
        for panics in [false, true] {
            let error = run(
                100_000,
                2,
                |unit| format!("unit {unit}"),
                Ok,
                |ordered| {
                    while let Some((unit, _)) = ordered.next()? {
                        assert!(!panics || unit != 10, "panicked at unit 10");
                        anyhow::ensure!(unit != 10, "stopped at unit 10");
                    }
                    Ok(())
                },
            )
            .unwrap_err();
            let expected = if panics {
                "the writer panicked: panicked at unit 10"
            } else {
                "stopped at unit 10"
            };
            assert_eq!(format!("{error:#}"), expected);
        }
    }
}
