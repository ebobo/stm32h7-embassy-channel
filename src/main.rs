#![no_std] // Don't link the Rust standard library, it's not available for the ARM Cortex-M architecture
#![no_main] // Don't use the Rust standard entry point, we will provide our own

use defmt::*; // defmt is a logging framework for embedded systems.
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};

use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _}; //

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;

// define a channel for communication
pub static CHANNEL: Channel<ThreadModeRawMutex, u32, 1> = Channel::new(); // Channel now uses Message instead of a byte array

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize, otherwise the timer will not work, i don't understand why
    embassy_stm32::init(Default::default());
    //say hello
    info!("Hello User!");
    // start the print logger
    spawner.spawn(log_task()).unwrap();

    loop {
        let channel_future = CHANNEL.receive();
        let timer_future = Timer::after_millis(500);

        match select(channel_future, timer_future).await {
            Either::First(channel_result) => {
                if channel_result > 0 {
                    info!("------ Received {}", channel_result);
                    let calculation = channel_result * 100;
                    info!("Calculation result {}", calculation);
                } else {
                    break;
                }
            }
            Either::Second(_) => {
                info!("Timer expired 500ms , no message received");
            }
        }
    }
}

// make a task
#[embassy_executor::task]
async fn log_task() -> () {
    let mut counter = 10;
    loop {
        counter -= 1;
        info!("Task 1 {}", counter);
        CHANNEL.send(counter).await;
        // wait for 1 second
        Timer::after(Duration::from_millis(1000)).await;
        if counter < 1 {
            info!("Counter is 0, exiting task 1");
            break;
        }
    }
}
