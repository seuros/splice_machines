//! Evidence-gated replacement, with no compatibility layer or I/O.
//!
//! The adapter serializes replacements, never reuses a generation, and
//! keeps admission to the retiring path closed through removal.
#![no_std]

use core::num::NonZeroU64;
use state_machines::state_machine;

/// Caller-owned generation of one replacement.
pub type Generation = NonZeroU64;

/// Evidence concerning readers of the retiring path.
#[derive(Debug, Clone, Copy)]
pub struct ReaderEvidence {
    pub generation: Generation,
    pub fence: ReaderFence,
}

#[derive(Debug, Clone, Copy)]
pub enum ReaderFence {
    Unknown,
    Closed { remaining: usize },
}

state_machine! {
    name: Splice,
    context: Generation,
    initial: Original,
    states: [Original, Expanded, Backfilled, Switched, ReadyToContract, Contracted],
    events {
        expanded {
            payload: Generation,
            guards: [current_generation],
            transition: { from: Original, to: Expanded }
        }
        backfilled {
            payload: Generation,
            guards: [current_generation],
            transition: { from: Expanded, to: Backfilled }
        }
        switched {
            payload: Generation,
            guards: [current_generation],
            transition: { from: Backfilled, to: Switched }
        }
        authorize_contract {
            payload: ReaderEvidence,
            guards: [retiring_readers_drained],
            transition: { from: Switched, to: ReadyToContract }
        }
        contracted {
            payload: Generation,
            guards: [current_generation],
            transition: { from: ReadyToContract, to: Contracted }
        }
    }
}

#[allow(
    clippy::unused_self,
    clippy::trivially_copy_pass_by_ref,
    reason = "state-machines guard signatures borrow self, context, and payload"
)]
impl<S> Splice<S> {
    fn current_generation(&self, generation: &Generation, reported: &Generation) -> bool {
        generation == reported
    }

    fn retiring_readers_drained(&self, generation: &Generation, evidence: &ReaderEvidence) -> bool {
        generation == &evidence.generation
            && matches!(evidence.fence, ReaderFence::Closed { remaining: 0 })
    }
}
