// dande | silicon-prion
// intentional memory corruption via unsafe raw pointers.
// don't run this on a prod machine unless you like kernel panics.

use std::{ptr, thread};
use std::time::Duration;
use rand::Rng;

// wrapper to force rust to let us send raw pointers across threads
struct UnsafeHeap(*mut u8);
unsafe impl Send for UnsafeHeap {}
unsafe impl Sync for UnsafeHeap {}

fn main() {
    let size: usize = 1024 * 1024 * 500; // 500MB of healthy "brain" tissue
    let mut brain_tissue = vec![0u8; size];
    let ptr = UnsafeHeap(brain_tissue.as_mut_ptr());

    println!("[*] allocated 500mb. introducing prion seed...");

    let threads: Vec<_> = (0..8).map(|_| {
        let heap_ptr = ptr.0;
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                // pick a random neuron
                let idx = rng.gen_range(0..size) as isize;
                unsafe {
                    let target = heap_ptr.offset(idx);
                    let current_val = ptr::read_volatile(target);
                    
                    // infection logic: if we hit a 0, flip it. rot spreads.
                    if current_val == 0 {
                        ptr::write_volatile(target, rng.gen::<u8>());
                    } else {
                        // cascading failure: corrupt neighboring bytes
                        if idx < (size as isize - 1) {
                            ptr::write_volatile(heap_ptr.offset(idx + 1), 0xFF);
                        }
                    }
                }
                thread::sleep(Duration::from_nanos(100)); // slow agonizing decay
            }
        })
    }).collect();

    // monitor the decay
    let heap_ptr = ptr.0;
    loop {
        let mut corrupted = 0;
        for i in (0..size).step_by(1000) {
            unsafe { if ptr::read_volatile(heap_ptr.offset(i as isize)) != 0 { corrupted += 1; } }
        }
        println!("[-] systemic decay: {} active corrupted clusters", corrupted);
        thread::sleep(Duration::from_millis(500));
    }
}

