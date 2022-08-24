#![no_std]
#![cfg_attr(target_os = "none", no_main)]
#![feature(type_alias_impl_trait)]

#[cfg(target_os = "none")]
use defmt::*;
use embassy_stm32::adc::{Adc, Resolution};
use embassy_stm32::pac;
use embassy_time::Delay;
#[cfg(not(target_os = "none"))]
use log::*;
use {defmt_rtt as _, panic_probe as _};

#[cfg_attr(target_os = "none", cortex_m_rt::entry)]
fn main() -> ! {
    info!("Hello World!");

    unsafe {
        pac::RCC.ccipr().modify(|w| {
            w.set_adcsel(0b11);
        });
        pac::RCC.ahb2enr().modify(|w| w.set_adcen(true));
    }

    let p = embassy_stm32::init(Default::default());

    let mut adc = Adc::new(p.ADC1, &mut Delay);
    //adc.enable_vref();
    adc.set_resolution(Resolution::EightBit);
    let mut channel = p.PC0;

    loop {
        let v = adc.read(&mut channel);
        info!("--> {}", v);
    }
}
