use rp_pico::hal::gpio::{
    bank0::{Gpio15, Gpio27},
    FunctionSioOutput, Pin, PullDown,
};

struct Buzzer {
    piv: Pin<Gpio27, FunctionSioOutput, PullDown>,
}
