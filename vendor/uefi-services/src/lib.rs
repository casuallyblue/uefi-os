//! This crate simplifies the writing of higher-level code for UEFI.
//!
//! It initializes the memory allocation and logging crates,
//! allowing code to use Rust's data structures and to log errors.
//!
//! Logging and allocation are only allowed while boot services are
//! active. Once runtime services are activated by calling
//! [`exit_boot_services`], the logger will be disabled and the
//! allocator will always return null.
//!
//! It also stores a global reference to the UEFI system table,
//! in order to reduce the redundant passing of references to it.
//!
//! Library code can simply use global UEFI functions
//! through the reference provided by `system_table`.
//!
//! [`exit_boot_services`]: uefi::table::SystemTable::exit_boot_services

#![no_std]
#![feature(lang_items)]
#![feature(panic_info_message)]

// Core types.
extern crate uefi;

use core::ffi::c_void;
use core::ptr::NonNull;

use uefi::prelude::*;
use uefi::table::boot::{EventType, Tpl};
use uefi::table::{Boot, SystemTable};
use uefi::{Event, Result};

/// Reference to the system table.
///
/// This table is only fully safe to use until UEFI boot services have been exited.
/// After that, some fields and methods are unsafe to use, see the documentation of
/// UEFI's ExitBootServices entry point for more details.
static mut SYSTEM_TABLE: Option<SystemTable<Boot>> = None;

/// Global logger object
static mut LOGGER: Option<uefi::logger::Logger> = None;

/// Obtains a pointer to the system table.
///
/// This is meant to be used by higher-level libraries,
/// which want a convenient way to access the system table singleton.
///
/// `init` must have been called first by the UEFI app.
///
/// The returned pointer is only valid until boot services are exited.
pub fn system_table() -> NonNull<SystemTable<Boot>> {
    unsafe {
        let table_ref = SYSTEM_TABLE
            .as_ref()
            .expect("The system table handle is not available");
        NonNull::new(table_ref as *const _ as *mut _).unwrap()
    }
}

/// Initialize the UEFI utility library.
///
/// This must be called as early as possible,
/// before trying to use logging or memory allocation capabilities.
pub fn init(st: &mut SystemTable<Boot>) -> Result {
    unsafe {
        // Avoid double initialization.
        if SYSTEM_TABLE.is_some() {
            return Status::SUCCESS.into();
        }

        // Setup the system table singleton
        SYSTEM_TABLE = Some(st.unsafe_clone());

        // Setup logging and memory allocation
        init_logger(st);
        let boot_services = st.boot_services();

        // Schedule these tools to be disabled on exit from UEFI boot services
        boot_services
            .create_event(
                EventType::SIGNAL_EXIT_BOOT_SERVICES,
                Tpl::NOTIFY,
                Some(exit_boot_services),
                None,
            )
            .map(|_| ())
    }
}

/// Set up logging
///
/// This is unsafe because you must arrange for the logger to be reset with
/// disable() on exit from UEFI boot services.
unsafe fn init_logger(st: &mut SystemTable<Boot>) {
    let stdout = st.stdout();

    // Construct the logger.
    let logger = {
        LOGGER = Some(uefi::logger::Logger::new(stdout));
        LOGGER.as_ref().unwrap()
    };

    // Set the logger.
    log::set_logger(logger).unwrap(); // Can only fail if already initialized.

    // Log everything.
    log::set_max_level(log::LevelFilter::Info);
}

/// Notify the utility library that boot services are not safe to call anymore
/// As this is a callback, it must be `extern "efiapi"`.
unsafe extern "efiapi" fn exit_boot_services(_e: Event, _ctx: Option<NonNull<c_void>>) {
    // DEBUG: The UEFI spec does not guarantee that this printout will work, as
    //        the services used by logging might already have been shut down.
    //        But it works on current OVMF, and can be used as a handy way to
    //        check that the callback does get called.
    //
    // info!("Shutting down the UEFI utility library");
    SYSTEM_TABLE = None;
    if let Some(ref mut logger) = LOGGER {
        logger.disable();
    }
}
