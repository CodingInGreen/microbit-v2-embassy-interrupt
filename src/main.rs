#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Pull};
use embassy_nrf::gpiote::{InputChannel, InputChannelPolarity};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    info!("Starting!");

    // P0_14 and P0_23 are the pins for Button A and Button B
    let button_a_channel = InputChannel::new(
        p.GPIOTE_CH0, // Use first GPIOTE channel for Button A
        Input::new(p.P0_14, Pull::Up), // Configure Button A with pull-up
        InputChannelPolarity::HiToLo, // Trigger on high-to-low transition (press)
    );
    let button_b_channel = InputChannel::new(
        p.GPIOTE_CH1, // Use second GPIOTE channel for Button B
        Input::new(p.P0_23, Pull::Up), // Configure Button B with pull-up
        InputChannelPolarity::HiToLo, // Trigger on high-to-low transition (press)
    );

    // Asynchronously handle button press events
    let handle_button_a = async {
        loop {
            button_a_channel.wait().await;
            info!("Button A pressed");
        }
    };

    let handle_button_b = async {
        loop {
            button_b_channel.wait().await;
            info!("Button B pressed");
        }
    };

    // Join both button handling futures to run concurrently
    embassy_futures::join::join(handle_button_a, handle_button_b).await;
}
