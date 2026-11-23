#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
mod bare {
    use core::{hint::black_box, num::NonZeroU64, panic::PanicInfo};
    use splice_machines::{ReaderEvidence, ReaderFence, Splice};

    #[panic_handler]
    fn panic(_: &PanicInfo<'_>) -> ! {
        loop {
            core::hint::spin_loop();
        }
    }

    // SAFETY: the bare-metal link probe sets this unique symbol as its entry.
    #[allow(unsafe_code)]
    #[unsafe(no_mangle)]
    extern "C" fn splice_probe() -> ! {
        let generation = black_box(NonZeroU64::MIN);
        let original = Splice::new(generation);
        if let Ok(expanded) = original.expanded(generation)
            && let Ok(backfilled) = expanded.backfilled(generation)
            && let Ok(switched) = backfilled.switched(generation)
            && let Ok(ready) = switched.authorize_contract(ReaderEvidence {
                generation,
                fence: ReaderFence::Closed { remaining: 0 },
            })
        {
            drop(black_box(ready.contracted(generation)));
        }
        loop {
            core::hint::spin_loop();
        }
    }
}

#[cfg(not(target_os = "none"))]
fn main() {}
