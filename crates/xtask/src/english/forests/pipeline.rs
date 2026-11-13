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
/// An error or panic in `work` (with `describe` of its unit as context) is
/// sent to the writer like a result, and `next` returns it when it arrives;
/// workers go on claiming units until `write` has returned and dropped its
/// [`Ordered`], which stops them. An error or panic in `write`, or a panic
/// in a worker outside `work`, is the returned error and stops the workers
/// too. `write` returning `Ok` before taking every unit is an error.
///
/// `work` must not use Rayon: the no-deadlock argument in the worker loop
/// rests on it.
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
            let written = write(&mut ordered)?;
            anyhow::ensure!(
                ordered.next == units,
                "the writer returned after taking {} of {units} units",
                ordered.next
            );
            Ok(written)
        });
        let workers = std::panic::catch_unwind(AssertUnwindSafe(|| {
            rayon::broadcast(|_| {
                let _stop = StopOnPanic(gate);
                // Why this cannot deadlock: let `c` be the lowest unit the
                // writer has yet to receive in order (`Ordered::next`). Units
                // are claimed in increasing order, and every unit below `c` was
                // claimed, so `c` is claimed or is the next claim. Its worker
                // passes `admit` at once (`c < c + window`), and computing and
                // sending never wait on anything, so `c` reaches the writer,
                // which hands it over and moves the window. Only units at `c +
                // window` or above ever wait, and they wait for the writer
                // alone, never for one another.
                //
                // This relies on `work` never using Rayon, so that computing
                // `c` waits on nothing: a worker thread waiting in a nested
                // `join` for a stolen part of `c` could run its own broadcast
                // job there and block in `admit` above `c`'s unfinished part,
                // and the writer would wait forever. `unit_rows` calls only
                // crates that do not depend on Rayon.
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
        }));
        // The writer sees the channel close if the workers stop short of the
        // last unit.
        drop(sender);
        let written = writer
            .join()
            .map_err(|panic| anyhow::anyhow!("the writer panicked: {}", message(panic.as_ref())))?;
        // A worker's panic outside `work` is why the writer stopped short.
        workers.map_err(|panic| {
            anyhow::anyhow!(
                "a worker panicked outside its unit: {}",
                message(panic.as_ref())
            )
        })?;
        written
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
pub(super) fn message(panic: &(dyn Any + Send)) -> String {
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
    use std::time::Duration;

    use super::*;

    /// Runs `f` on a thread of its own and fails if it has not finished
    /// within a minute, so a pipeline that stops neither side fails the test
    /// instead of hanging it.
    fn bounded<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            // A panic drops `sender`, which `recv_timeout` reports.
            sender.send(f()).unwrap();
        });
        match receiver.recv_timeout(Duration::from_secs(60)) {
            Ok(result) => result,
            Err(mpsc::RecvTimeoutError::Timeout) => panic!("the pipeline ran for a minute"),
            Err(mpsc::RecvTimeoutError::Disconnected) => panic!("the pipeline panicked"),
        }
    }

    /// Runs units `0..units` through a writer that takes them all and
    /// returns them in the order it took them.
    fn take_all(
        units: usize,
        window: usize,
        work: impl Fn(usize) -> Result<usize> + Sync,
    ) -> Result<Vec<usize>> {
        run(
            units,
            window,
            |unit| format!("unit {unit}"),
            work,
            |ordered| {
                let mut taken = Vec::new();
                while let Some((unit, value)) = ordered.next()? {
                    assert_eq!(value, unit * 2);
                    taken.push(unit);
                }
                Ok(taken)
            },
        )
    }

    /// Every fourth unit is slow, so later units finish first, and the
    /// writer still takes every unit in order.
    #[test]
    fn units_finishing_out_of_order_are_taken_in_order() {
        let (taken, out_of_order) = bounded(|| {
            let finished = AtomicUsize::new(0);
            let out_of_order = AtomicUsize::new(0);
            let taken = take_all(300, 16, |unit| {
                if unit % 4 == 0 {
                    std::thread::sleep(Duration::from_millis(2));
                }
                if finished.fetch_max(unit, Ordering::Relaxed) > unit {
                    out_of_order.fetch_add(1, Ordering::Relaxed);
                }
                Ok(unit * 2)
            })
            .unwrap();
            (taken, out_of_order.into_inner())
        });
        assert!(out_of_order > 0, "no unit finished out of order");
        assert_eq!(taken, (0..300).collect::<Vec<_>>());
    }

    /// The writer is slower than the work, so workers run ahead of it as
    /// far as the window lets them and no further: a unit starts once fewer
    /// than `window` units past the next one the writer is to take. The
    /// writer's own count can lag the gate's by the unit it has just taken.
    #[test]
    fn workers_run_ahead_of_the_writer_by_at_most_the_window() {
        const WINDOW: usize = 8;
        let ahead = bounded(|| {
            let taken = AtomicUsize::new(0);
            let ahead = AtomicUsize::new(0);
            run(
                500,
                WINDOW,
                |unit| format!("unit {unit}"),
                |unit| {
                    let at = taken.load(Ordering::SeqCst);
                    ahead.fetch_max(unit.saturating_sub(at), Ordering::SeqCst);
                    Ok(())
                },
                |ordered| {
                    while let Some((unit, ())) = ordered.next()? {
                        std::thread::sleep(Duration::from_micros(200));
                        taken.store(unit + 1, Ordering::SeqCst);
                    }
                    Ok(())
                },
            )
            .unwrap();
            ahead.into_inner()
        });
        assert!(ahead >= 2, "workers never ran ahead: {ahead}");
        assert!(
            ahead <= WINDOW,
            "a unit started {ahead} ahead of the writer"
        );
    }

    #[test]
    fn a_worker_s_error_is_the_result() {
        let error = bounded(|| {
            take_all(1000, 4, |unit| {
                anyhow::ensure!(unit != 300, "no unit 300");
                Ok(unit * 2)
            })
            .unwrap_err()
        });
        assert_eq!(format!("{error:#}"), "unit 300: no unit 300");
    }

    #[test]
    fn a_worker_s_panic_is_the_result() {
        let error = bounded(|| {
            take_all(1000, 4, |unit| {
                assert!(unit != 300, "no unit 300");
                Ok(unit * 2)
            })
            .unwrap_err()
        });
        assert_eq!(format!("{error:#}"), "unit 300: panicked: no unit 300");
    }

    /// A worker panics outside `work`, describing unit 5's error: the other
    /// workers, waiting on a window that never moves past unit 5, stop, and
    /// the panic is the result.
    #[test]
    fn a_worker_s_panic_outside_its_unit_is_the_result() {
        let error = bounded(|| {
            run(
                1000,
                2,
                |unit| panic!("no description of unit {unit}"),
                |unit| {
                    anyhow::ensure!(unit != 5, "no unit 5");
                    Ok(unit)
                },
                |ordered| {
                    while ordered.next()?.is_some() {}
                    Ok(())
                },
            )
            .unwrap_err()
        });
        assert_eq!(
            format!("{error:#}"),
            "a worker panicked outside its unit: no description of unit 5"
        );
    }

    /// The writer stops after unit 10 while the workers wait on the window:
    /// they stop too, and its error is the result.
    #[test]
    fn the_writer_s_error_and_panic_are_the_result() {
        for panics in [false, true] {
            let error = bounded(move || {
                run(
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
                .unwrap_err()
            });
            let expected = if panics {
                "the writer panicked: panicked at unit 10"
            } else {
                "stopped at unit 10"
            };
            assert_eq!(format!("{error:#}"), expected);
        }
    }

    /// A writer that returns `Ok` after unit 10 drops units it never took.
    #[test]
    fn a_writer_that_stops_short_is_an_error() {
        let error = bounded(|| {
            run(
                100_000,
                2,
                |unit| format!("unit {unit}"),
                Ok,
                |ordered| {
                    while let Some((unit, _)) = ordered.next()? {
                        if unit == 10 {
                            return Ok(());
                        }
                    }
                    Ok(())
                },
            )
            .unwrap_err()
        });
        assert_eq!(
            format!("{error:#}"),
            "the writer returned after taking 11 of 100000 units"
        );
    }
}
