use embedded_hal::digital::OutputPin;
use rp2040_hal::gpio::{bank0::{Gpio10, Gpio12, Gpio13, Gpio16, Gpio18, Gpio20, Gpio22}, FunctionSioOutput, Pin, PullDown};
pub type A = Gpio10;
pub(crate) type B = Gpio16;
pub(crate) type C = Gpio18;
pub(crate) type D = Gpio20;
pub(crate) type E = Gpio22;
pub(crate) type STB = Gpio12;
pub(crate) type OE = Gpio13;
pub(crate) struct LineDriver{
    pub a: Pin<A, FunctionSioOutput, PullDown>,
    pub b: Pin<B, FunctionSioOutput, PullDown>,
    pub c: Pin<C, FunctionSioOutput, PullDown>,
    pub d: Pin<D, FunctionSioOutput, PullDown>,
    pub e: Pin<E, FunctionSioOutput, PullDown>,
    pub oe: Pin<OE, FunctionSioOutput, PullDown>,
}

impl LineDriver{
    pub fn new(a: Pin<A, FunctionSioOutput, PullDown>, b: Pin<B, FunctionSioOutput, PullDown>, c: Pin<C, FunctionSioOutput, PullDown>, d: Pin<D, FunctionSioOutput, PullDown>, e: Pin<E, FunctionSioOutput, PullDown>,  oe: Pin<OE, FunctionSioOutput, PullDown>) -> Self{
        Self{
            a,
            b,
            c,
            d,
            e,
            oe,
        }
    }
    pub fn set_line(&mut self, line: usize){
     
        if line&1==1{
            self.a.set_high();
        }
        else{
            self.a.set_low();
        }
        if line&2==2{
            self.b.set_high();
        }
        else{
            self.b.set_low();
        }
        if line&4==4{
            self.c.set_high();
        }else{
            self.c.set_low();
        }
        if line&8==8{
            self.d.set_high();
        }else{
            self.d.set_low();
        }
        if line&16==16{
            self.e.set_high();
        }else{
            self.e.set_low();
        }
    }
}