/// Return the remaining compute units the program may consume
#[inline]
pub fn trz_remaining_compute_units() -> u64 {
    #[cfg(target_os = "trezoa")]
    unsafe {
        crate::syscalls::trz_remaining_compute_units()
    }

    #[cfg(not(target_os = "trezoa"))]
    {
        crate::program_stubs::trz_remaining_compute_units()
    }
}
