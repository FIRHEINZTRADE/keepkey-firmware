//! USB OTG full-speed peripheral
//!
//! Requires the `usb_fs` feature.
//! Only one of the `usb_fs`/`usb_hs` features can be selected at the same time.

use stm32f2::stm32f215 as pac;

use crate::gpio::{
    gpioa::{PA11, PA12},
    Alternate, AF10,
};
use crate::time::Hertz;

pub use synopsys_usb_otg::UsbBus;
use synopsys_usb_otg::UsbPeripheral;

pub struct USB {
    pub usb_global: pac::OTG_FS_GLOBAL,
    pub usb_device: pac::OTG_FS_DEVICE,
    pub usb_pwrclk: pac::OTG_FS_PWRCLK,
}

unsafe impl Sync for USB {}

unsafe impl UsbPeripheral for USB {
    const REGISTERS: *const () = stm32::OTG_FS_GLOBAL::ptr() as *const ();
    const HIGH_SPEED: bool = false;
    const FIFO_DEPTH_WORDS: usize = 320;
    const ENDPOINT_COUNT: usize = 4;

    fn enable() {
        let rcc = unsafe { &*stm32::RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            rcc.ahb2enr.modify(|_, w| w.otgfsen().set_bit());

            // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
            cortex_m::asm::dsb();

            // Reset USB peripheral
            rcc.ahb2rstr.modify(|_, w| w.otgfsrst().set_bit());
            rcc.ahb2rstr.modify(|_, w| w.otgfsrst().clear_bit());
        });
    }

    fn ahb_frequency_hz(&self) -> u32 {
        120_000_000
    }
}

pub type UsbBusType = UsbBus<USB>;
