# Button Interrupt Handler for micro:bit v2

## Description
This Rust program demonstrates the use of Embassy for handling button interrupts on a micro:bit v2 board. It sets up asynchronous handling for two buttons using the GPIOTE (GPIO tasks and events) peripheral of the nRF microcontroller. Each button press is detected through a high-to-low transition and logs the event using `defmt`.

### Features
- Utilizes the `embassy-nrf` crate for embedded Rust development.
- Configures pull-up resistors and monitors button presses asynchronously.
- Logs button press events without blocking the main application flow.

This program is ideal for embedded applications requiring non-blocking user input handling in Rust.
