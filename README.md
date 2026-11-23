# Splice Machines

> Switching the traffic doesn't mean the old passage is empty.

Splice lets you replace a live path in stages. The `state-machines` typestate
graph owns their order. Splice checks the evidence you supply; it neither moves
traffic nor removes the old path.

## The Flight Plan

1. **Expand.** Start `Splice::new(generation)` with a fresh generation. Install
   the new path alongside the old one, then report `expanded(generation)`.
2. **Backfill and switch.** Prepare the replacement and report
   `backfilled(generation)`. Switch traffic, then report `switched(generation)`.
   Evidence from a different generation cannot advance the replacement.
3. **Close and drain.** Fence admission to the retiring path and wait for its
   last reader to leave. Pass `ReaderEvidence` for the current generation to
   `authorize_contract`. `Unknown` and `Closed` with readers still present
   both fail the guard. A failed guard returns the original machine so you can
   report new evidence later.
4. **Remove, then confirm.** `ReadyToContract` is permission, not completion.
   Keep admission closed while removing the old path; report
   `contracted(generation)` only after removal.

## What's Yours

The adapter serializes replacements, allocates durable generations without
reuse, supplies authoritative evidence, and owns persistence and the effects.
Splice ships no compatibility implementation or fallback path. The Foundation
simulation uses Pennant to check for fenced retirement before starting Splice;
that is an adapter decision, not a Splice dependency.

## Allocator-free build

Splice uses `state-machines` with default features disabled. To verify that
the example links without a global allocator, run from this directory:

```sh
RUSTFLAGS='-C panic=abort -C link-arg=-esplice_probe' \
  cargo build --target thumbv7em-none-eabihf --example no_allocator
```
