// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2024.

use crate::chip_specific::ChipSpecs;
use crate::clocks::Clocks;
use crate::gpio::{GpioPort, GPIO_NUM_PORTS};
use crate::rcc::{APBPrescaler, Rcc, RtcClockSource};
use kernel::platform::chip::ClockInterface;

/// Extension to ClockInterface
pub trait PeripheralClockInterface: ClockInterface {
    fn get_frequency(&self) -> u32;
    fn configure(&self);
}

pub struct PeripheralClock<'a, C>
where
    C: ChipSpecs,
{
    pub clock: PeripheralClockType,
    clocks: &'a Clocks<'a, C>,
    rcc: &'a Rcc,
}

/// Bus + Clock name for the peripherals
pub enum PeripheralClockType {
    AHB1(HCLK1),
    AHB2(HCLK2),
    AHB3(HCLK3),
    APB1(PCLK1),
    APB2(PCLK2),
    RTC,
    PWR,
}

/// Peripherals clocked by HCLK1
pub enum HCLK1 {
    DMA1,
    DMA2,
    GPIOH,
    GPIOG,
    GPIOF,
    GPIOE,
    GPIOD,
    GPIOC,
    GPIOB,
    GPIOA,
}

/// Peripherals clocked by HCLK3
pub enum HCLK3 {
    FMC,
}

/// Peripherals clocked by HCLK2
pub enum HCLK2 {
    RNG,
    OTGFS,
}

/// Peripherals clocked by PCLK1
pub enum PCLK1 {
    TIM2,
    USART2,
    USART3,
    SPI3,
    I2C1,
    CAN1,
    DAC,
}

/// Peripherals clocked by PCLK2
pub enum PCLK2 {
    USART1,
    ADC1,
    SYSCFG,
}

impl<'a, C> PeripheralClock<'a, C>
where
    C: ChipSpecs,
{
    pub const fn new(clock: PeripheralClockType, rcc: &'a Rcc, clocks: &'a Clocks<'a, C>) -> Self {
        Self { clock, rcc, clocks }
    }
}

impl<'a, C> ClockInterface for PeripheralClock<'a, C>
where
    C: ChipSpecs,
{
    fn is_enabled(&self) -> bool {
        match self.clock {
            PeripheralClockType::AHB1(ref v) => match v {
                HCLK1::DMA1 => self.rcc.is_enabled_dma1_clock(),
                HCLK1::DMA2 => self.rcc.is_enabled_dma2_clock(),
                HCLK1::GPIOH => self.rcc.is_enabled_gpioh_clock(),
                HCLK1::GPIOG => self.rcc.is_enabled_gpiog_clock(),
                HCLK1::GPIOF => self.rcc.is_enabled_gpiof_clock(),
                HCLK1::GPIOE => self.rcc.is_enabled_gpioe_clock(),
                HCLK1::GPIOD => self.rcc.is_enabled_gpiod_clock(),
                HCLK1::GPIOC => self.rcc.is_enabled_gpioc_clock(),
                HCLK1::GPIOB => self.rcc.is_enabled_gpiob_clock(),
                HCLK1::GPIOA => self.rcc.is_enabled_gpioa_clock(),
            },
            PeripheralClockType::AHB2(ref v) => match v {
                HCLK2::RNG => self.rcc.is_enabled_rng_clock(),
                HCLK2::OTGFS => self.rcc.is_enabled_otgfs_clock(),
            },
            PeripheralClockType::AHB3(ref v) => match v {
                HCLK3::FMC => self.rcc.is_enabled_fmc_clock(),
            },
            PeripheralClockType::APB1(ref v) => match v {
                PCLK1::TIM2 => self.rcc.is_enabled_tim2_clock(),
                PCLK1::USART2 => self.rcc.is_enabled_usart2_clock(),
                PCLK1::USART3 => self.rcc.is_enabled_usart3_clock(),
                PCLK1::I2C1 => self.rcc.is_enabled_i2c1_clock(),
                PCLK1::SPI3 => self.rcc.is_enabled_spi3_clock(),
                PCLK1::CAN1 => self.rcc.is_enabled_can1_clock(),
                PCLK1::DAC => self.rcc.is_enabled_dac_clock(),
            },
            PeripheralClockType::APB2(ref v) => match v {
                PCLK2::USART1 => self.rcc.is_enabled_usart1_clock(),
                PCLK2::ADC1 => self.rcc.is_enabled_adc1_clock(),
                PCLK2::SYSCFG => self.rcc.is_enabled_syscfg_clock(),
            },
            PeripheralClockType::RTC => self.rcc.is_enabled_rtc_clock(),
            PeripheralClockType::PWR => self.rcc.is_enabled_pwr_clock(),
        }
    }

    fn enable(&self) {
        match self.clock {
            PeripheralClockType::AHB1(ref v) => match v {
                HCLK1::DMA1 => {
                    self.rcc.enable_dma1_clock();
                }
                HCLK1::DMA2 => {
                    self.rcc.enable_dma2_clock();
                }
                HCLK1::GPIOH => {
                    self.rcc.enable_gpioh_clock();
                }
                HCLK1::GPIOG => {
                    self.rcc.enable_gpiog_clock();
                }
                HCLK1::GPIOF => {
                    self.rcc.enable_gpiof_clock();
                }
                HCLK1::GPIOE => {
                    self.rcc.enable_gpioe_clock();
                }
                HCLK1::GPIOD => {
                    self.rcc.enable_gpiod_clock();
                }
                HCLK1::GPIOC => {
                    self.rcc.enable_gpioc_clock();
                }
                HCLK1::GPIOB => {
                    self.rcc.enable_gpiob_clock();
                }
                HCLK1::GPIOA => {
                    self.rcc.enable_gpioa_clock();
                }
            },
            PeripheralClockType::AHB2(ref v) => match v {
                HCLK2::RNG => {
                    self.rcc.enable_rng_clock();
                }
                HCLK2::OTGFS => {
                    self.rcc.enable_otgfs_clock();
                }
            },
            PeripheralClockType::AHB3(ref v) => match v {
                HCLK3::FMC => self.rcc.enable_fmc_clock(),
            },
            PeripheralClockType::APB1(ref v) => match v {
                PCLK1::TIM2 => {
                    self.rcc.enable_tim2_clock();
                }
                PCLK1::USART2 => {
                    self.rcc.enable_usart2_clock();
                }
                PCLK1::USART3 => {
                    self.rcc.enable_usart3_clock();
                }
                PCLK1::I2C1 => {
                    self.rcc.enable_i2c1_clock();
                }
                PCLK1::SPI3 => {
                    self.rcc.enable_spi3_clock();
                }
                PCLK1::CAN1 => {
                    self.rcc.enable_can1_clock();
                }
                PCLK1::DAC => {
                    self.rcc.enable_dac_clock();
                }
            },
            PeripheralClockType::APB2(ref v) => match v {
                PCLK2::USART1 => {
                    self.rcc.enable_usart1_clock();
                }
                PCLK2::ADC1 => {
                    self.rcc.enable_adc1_clock();
                }
                PCLK2::SYSCFG => {
                    self.rcc.enable_syscfg_clock();
                }
            },
            PeripheralClockType::RTC => self.rcc.enable_rtc_clock(RtcClockSource::LSI),
            PeripheralClockType::PWR => self.rcc.enable_pwr_clock(),
        }
    }

    fn disable(&self) {
        match self.clock {
            PeripheralClockType::AHB1(ref v) => match v {
                HCLK1::DMA1 => {
                    self.rcc.disable_dma1_clock();
                }
                HCLK1::DMA2 => {
                    self.rcc.disable_dma2_clock();
                }
                HCLK1::GPIOH => {
                    self.rcc.disable_gpioh_clock();
                }
                HCLK1::GPIOG => {
                    self.rcc.disable_gpiog_clock();
                }
                HCLK1::GPIOF => {
                    self.rcc.disable_gpiof_clock();
                }
                HCLK1::GPIOE => {
                    self.rcc.disable_gpioe_clock();
                }
                HCLK1::GPIOD => {
                    self.rcc.disable_gpiod_clock();
                }
                HCLK1::GPIOC => {
                    self.rcc.disable_gpioc_clock();
                }
                HCLK1::GPIOB => {
                    self.rcc.disable_gpiob_clock();
                }
                HCLK1::GPIOA => {
                    self.rcc.disable_gpioa_clock();
                }
            },
            PeripheralClockType::AHB2(ref v) => match v {
                HCLK2::RNG => {
                    self.rcc.disable_rng_clock();
                }
                HCLK2::OTGFS => {
                    self.rcc.disable_otgfs_clock();
                }
            },
            PeripheralClockType::AHB3(ref v) => match v {
                HCLK3::FMC => self.rcc.disable_fmc_clock(),
            },
            PeripheralClockType::APB1(ref v) => match v {
                PCLK1::TIM2 => {
                    self.rcc.disable_tim2_clock();
                }
                PCLK1::USART2 => {
                    self.rcc.disable_usart2_clock();
                }
                PCLK1::USART3 => {
                    self.rcc.disable_usart3_clock();
                }
                PCLK1::I2C1 => {
                    self.rcc.disable_i2c1_clock();
                }
                PCLK1::SPI3 => {
                    self.rcc.disable_spi3_clock();
                }
                PCLK1::CAN1 => {
                    self.rcc.disable_can1_clock();
                }
                PCLK1::DAC => {
                    self.rcc.disable_dac_clock();
                }
            },
            PeripheralClockType::APB2(ref v) => match v {
                PCLK2::USART1 => {
                    self.rcc.disable_usart1_clock();
                }
                PCLK2::ADC1 => {
                    self.rcc.disable_adc1_clock();
                }
                PCLK2::SYSCFG => {
                    self.rcc.disable_syscfg_clock();
                }
            },
            PeripheralClockType::RTC => self.rcc.disable_rtc_clock(),
            PeripheralClockType::PWR => self.rcc.disable_pwr_clock(),
        }
    }
}

impl<'a, C> PeripheralClockInterface for PeripheralClock<'a, C>
where
    C: ChipSpecs,
{
    fn get_frequency(&self) -> u32 {
        #[inline(always)]
        fn tim_freq(rcc: &Rcc, hclk_freq: usize, prescaler: APBPrescaler) -> usize {
            // Reference Manual RM0090 section 6.2
            // When TIMPRE bit of the RCC_DCKCFGR register is reset, if APBx prescaler is 1, then
            // TIMxCLK = PCLKx, otherwise TIMxCLK = 2x PCLKx.
            // When TIMPRE bit in the RCC_DCKCFGR register is set, if APBx prescaler is 1,2 or 4,
            // then TIMxCLK = HCLK, otherwise TIMxCLK = 4x PCLKx.
            if !rcc.is_enabled_tim_pre() {
                match prescaler {
                    APBPrescaler::DivideBy1 | APBPrescaler::DivideBy2 => hclk_freq,
                    _ => hclk_freq / usize::from(prescaler) * 2,
                }
            } else {
                match prescaler {
                    APBPrescaler::DivideBy1 | APBPrescaler::DivideBy2 | APBPrescaler::DivideBy4 => {
                        hclk_freq
                    }
                    _ => hclk_freq / usize::from(prescaler) * 4,
                }
            }
        }
        let hclk_freq = self.rcc.get_sys_clock_frequency();
        match self.clock {
            PeripheralClockType::AHB1(_)
            | PeripheralClockType::AHB2(_)
            | PeripheralClockType::AHB3(_) => hclk_freq as u32,
            PeripheralClockType::APB1(ref v) => {
                let prescaler = self.rcc.get_apb1_prescaler();
                match v {
                    PCLK1::TIM2 => tim_freq(self.rcc, hclk_freq, prescaler) as u32,
                    _ => (hclk_freq / usize::from(prescaler)) as u32,
                }
            }
            PeripheralClockType::APB2(_) => {
                let prescaler = self.rcc.get_apb2_prescaler();
                (hclk_freq / usize::from(prescaler)) as u32
            }
            //TODO: implement clock frequency retrieval for RTC and PWR peripherals
            PeripheralClockType::RTC => todo!(),
            PeripheralClockType::PWR => todo!(),
        }
    }

    fn configure(&self) {
        match self.clock {
            PeripheralClockType::AHB2(HCLK2::RNG) => self.rcc.configure_rng_clock(),
            _ => (),
        }
    }
}

pub struct GpioClocks<'a, C: ChipSpecs>([PeripheralClock<'a, C>; GPIO_NUM_PORTS]);

impl<'a, C> GpioClocks<'a, C>
where
    C: ChipSpecs,
{
    pub fn new(rcc: &'a Rcc, clocks: &'a Clocks<'a, C>) -> Self {
        Self ([
            PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::GPIOA), rcc, clocks),
            PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::GPIOB), rcc, clocks),
            PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::GPIOC), rcc, clocks),
            PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::GPIOD), rcc, clocks),
            PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::GPIOE), rcc, clocks),
            PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::GPIOF), rcc, clocks),
            PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::GPIOG), rcc, clocks),
            PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::GPIOH), rcc, clocks),
        ])
    }
    pub fn get_refs(&self) -> [&dyn PeripheralClockInterface; GPIO_NUM_PORTS] {
        [
            &self.0[0],
            &self.0[1],
            &self.0[2],
            &self.0[3],
            &self.0[4],
            &self.0[5],
            &self.0[6],
            &self.0[7],
        ]
    }
}

pub struct PeripheralClocks<'a, C>
where
    C: ChipSpecs,
{
    pub adc1: PeripheralClock<'a, C>,
    pub can1: PeripheralClock<'a, C>,
    pub dac: PeripheralClock<'a, C>,
    pub dma1: PeripheralClock<'a, C>,
    pub dma2: PeripheralClock<'a, C>,
    pub fsmc: PeripheralClock<'a, C>,
    pub i2c1: PeripheralClock<'a, C>,
    pub pwr: PeripheralClock<'a, C>,
    pub rtc: PeripheralClock<'a, C>,
    pub syscfg: PeripheralClock<'a, C>,
    pub spi3: PeripheralClock<'a, C>,
    pub tim2: PeripheralClock<'a, C>,
    pub trng: PeripheralClock<'a, C>,
    pub usart1: PeripheralClock<'a, C>,
    pub usart2: PeripheralClock<'a, C>,
    pub usart3: PeripheralClock<'a, C>,
    gpio_ports: GpioClocks<'a, C>,
}

impl<'a, C> PeripheralClocks<'a, C>
where
    C: ChipSpecs,
{
    pub fn new(rcc: &'a Rcc, clocks: &'a Clocks<'a, C>) -> Self {
        Self {
            adc1: PeripheralClock::new(PeripheralClockType::APB2(PCLK2::ADC1), rcc, clocks),
            can1: PeripheralClock::new(PeripheralClockType::APB1(PCLK1::CAN1), rcc, clocks),
            dac: PeripheralClock::new(PeripheralClockType::APB1(PCLK1::DAC), rcc, clocks),
            dma1: PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::DMA1), rcc, clocks),
            dma2: PeripheralClock::new(PeripheralClockType::AHB1(HCLK1::DMA2), rcc, clocks),
            fsmc: PeripheralClock::new(PeripheralClockType::AHB3(HCLK3::FMC), rcc, clocks),
            i2c1: PeripheralClock::new(PeripheralClockType::APB1(PCLK1::I2C1), rcc, clocks),
            pwr: PeripheralClock::new(PeripheralClockType::PWR, rcc, clocks),
            rtc: PeripheralClock::new(PeripheralClockType::RTC, rcc, clocks),
            syscfg: PeripheralClock::new(PeripheralClockType::APB2(PCLK2::SYSCFG), rcc, clocks),
            spi3: PeripheralClock::new(PeripheralClockType::APB1(PCLK1::SPI3), rcc, clocks),
            tim2: PeripheralClock::new(PeripheralClockType::APB1(PCLK1::TIM2), rcc, clocks),
            trng: PeripheralClock::new(PeripheralClockType::AHB2(HCLK2::RNG), rcc, clocks),
            usart1: PeripheralClock::new(PeripheralClockType::APB2(PCLK2::USART1), rcc, clocks),
            usart2: PeripheralClock::new(PeripheralClockType::APB1(PCLK1::USART2), rcc, clocks),
            usart3: PeripheralClock::new(PeripheralClockType::APB1(PCLK1::USART3), rcc, clocks),
            gpio_ports: GpioClocks::new(rcc, clocks),
        }
    }
    pub fn get_gpio_ports(&self) -> [&dyn PeripheralClockInterface; GPIO_NUM_PORTS] {
        self.gpio_ports.get_refs()
    }
}
