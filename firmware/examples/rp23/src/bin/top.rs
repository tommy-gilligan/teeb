#![no_std]
#![no_main]

use core::cell::Cell;
use core::cell::RefCell;
use core::fmt;
use core::iter::Peekable;
use core::slice::IterMut;
use core::slice::Windows;
use core::sync::atomic::{AtomicBool, Ordering};
use heapless::String;

use assign_resources::assign_resources;
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::{
    bind_interrupts,
    block::ImageDef,
    gpio::{Flex, Input, Level, Output, Pull},
    peripherals::UART1,
    peripherals::USB,
    uart::{Uart, BufferedInterruptHandler, BufferedUart, BufferedUartRx, Config as UartConfig, StopBits, Parity},
    usb::{Driver as UsbDriver, InterruptHandler},
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel;
use embassy_time::Timer;
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State as HidState};
use embassy_usb::control::OutResponse;
use embassy_usb::{Builder, Config, Handler};
use embedded_io_async::{Read, Write};
use static_cell::StaticCell;
use core::default::Default;
use usbd_hid::descriptor::KeyboardUsage;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    UART1_IRQ => BufferedInterruptHandler<UART1>;
});

static EVENT_CHANNEL: channel::Channel<CriticalSectionRawMutex, KeyboardUsage, 10> = channel::Channel::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let driver = UsbDriver::new(p.USB, Irqs);

    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("HID keyboard example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut request_handler = MyRequestHandler {};
    let mut device_handler = MyDeviceHandler::new();

    let mut state = HidState::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    builder.handler(&mut device_handler);

    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };
    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, config);

    let (tx_pin, rx_pin, uart) = (p.PIN_8, p.PIN_9, p.UART1);

    static TX_BUF: StaticCell<[u8; 100]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; 100])[..];
    static RX_BUF: StaticCell<[u8; 100]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; 100])[..];
    let mut uconfig = UartConfig::default();
    uconfig.parity = Parity::ParityEven;
    uconfig.stop_bits = StopBits::STOP2;

    let mut uart = Uart::new_blocking(uart, tx_pin, rx_pin, uconfig);

    // let (mut tx, mut rx) = uart.split();

    let (reader, mut writer) = hid.split();
    let mut usb = builder.build();
    let usb_fut = usb.run();
    let in_fut = async {
        loop {
            let mut read_buf: [u8; 11] = [0; 11];

            match uart.blocking_read(&mut read_buf) {
                Ok(_) => defmt::info!("{:?}", &read_buf),
                Err(e) => defmt::info!("{:?}", e)
            }
        }
    };

    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };

    join(usb_fut, join(in_fut, out_fut)).await;
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!("Device configured, it may now draw up to the configured current limit from Vbus.")
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
