use core::{array, iter::Map};
use rp_pico::hal::{clocks::ClocksManager, gpio::Error, pac};
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use rp_pico::{
    hal::{
        gpio::{
            bank0::{
                Gpio10, Gpio11, Gpio12, Gpio13, Gpio16, Gpio18, Gpio2, Gpio20, Gpio22, Gpio3,
                Gpio4, Gpio5, Gpio8, Gpio9,
            },
            FunctionSioOutput, Pin, PinId, PullDown,
        }, timer, Sio
    },
    pac::Peripherals,
    Pins,
};

use rp_pico::hal::prelude::*;

const Matrix_COLS: usize=64;
const Matrix_ROWS: usize=32;
const Matrix_ROWS_SHOW: usize=Matrix_ROWS/2;
const Matrix_COLS_BYTE: usize=Matrix_COLS/8;

type R1 = Gpio2;
type G1 = Gpio3;
type B1 = Gpio4;
type R2 = Gpio5;
type G2 = Gpio8;
type B2 = Gpio9;
type A = Gpio10;
type B = Gpio16;
type C = Gpio18;
type D = Gpio20;
type E = Gpio22;
type CLK = Gpio11;
type STB = Gpio12;
type OE = Gpio13;

// enum DisplayPins{
//     R1(Gpio2),
//     G1,
//     B1,
//     R2,
//     G2,
//     B2,
//     A,
//     B,
//     C,
//     D,
//     E,
//     CLK,
//     STB,
//     OE
// }

// impl From<DisplayPins> for u32{
//     fn from(value: DisplayPins) -> Self {
//         match value{
//             DisplayPins::R1 => 2,
//             DisplayPins::G1 => 3,
//             DisplayPins::B1 => 4,
//             DisplayPins::R2 => 5,
//             DisplayPins::G2 => 8,
//             DisplayPins::B2 => 9,
//             DisplayPins::A => 10,
//             DisplayPins::B => 16,
//             DisplayPins::C => 18,
//             DisplayPins::D => 20,
//             DisplayPins::E => 22,
//             DisplayPins::CLK => 11,
//             DisplayPins::STB => 12,
//             DisplayPins::OE => 13,
//         }
//     }
// }

pub struct DisplayDriver {
    delay: cortex_m::delay::Delay,
    pub r1: Pin<R1, FunctionSioOutput, PullDown>,
    pub g1: Pin<G1, FunctionSioOutput, PullDown>,
    pub b1: Pin<B1, FunctionSioOutput, PullDown>,
    pub r2: Pin<R2, FunctionSioOutput, PullDown>,
    pub g2: Pin<G2, FunctionSioOutput, PullDown>,
    pub b2: Pin<B2, FunctionSioOutput, PullDown>,
    pub a: Pin<A, FunctionSioOutput, PullDown>,
    pub b: Pin<B, FunctionSioOutput, PullDown>,
    pub c: Pin<C, FunctionSioOutput, PullDown>,
    pub d: Pin<D, FunctionSioOutput, PullDown>,
    pub e: Pin<E, FunctionSioOutput, PullDown>,
    pub clk: Pin<CLK, FunctionSioOutput, PullDown>,
    pub stb: Pin<STB, FunctionSioOutput, PullDown>,
    pub oe: Pin<OE, FunctionSioOutput, PullDown>,

    image_r: [[u8;Matrix_COLS];Matrix_ROWS],
    image_g: [[u8;Matrix_COLS];Matrix_ROWS],
    image_b:[[u8;Matrix_COLS];Matrix_ROWS]
}

impl DisplayDriver {
    pub fn new(pins: Pins, core: pac::CorePeripherals,clocks: ClocksManager) -> Result<Self,Error> {
        let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        
        let mut driver=Self {
            r1: pins.gpio2.into_push_pull_output(),
            g1: pins.gpio3.into_push_pull_output(),
            b1: pins.gpio4.into_push_pull_output(),
            r2: pins.gpio5.into_push_pull_output(),
            g2: pins.gpio8.into_push_pull_output(),
            b2: pins.gpio9.into_push_pull_output(),
            a: pins.gpio10.into_push_pull_output(),
            b: pins.gpio16.into_push_pull_output(),
            c: pins.gpio18.into_push_pull_output(),
            d: pins.gpio20.into_push_pull_output(),
            e: pins.gpio22.into_push_pull_output(),
            clk: pins.gpio11.into_push_pull_output(),
            stb: pins.gpio12.into_push_pull_output(),
            oe: pins.gpio13.into_push_pull_output(),
            delay,
            image_r: [[0;Matrix_COLS];Matrix_ROWS],
            image_g: [[0;Matrix_COLS];Matrix_ROWS],
            image_b: [[0;Matrix_COLS];Matrix_ROWS],
          
        };
        // driver.oe.set_high()?;
        // driver.stb.set_low()?;
        // driver.clk.set_low()?;

        // let MaxLed=64;
        // let C12=[0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

        // let C13=[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];

        // for l in 0..MaxLed{
        //     let y=l%16;
        //     driver.r1.set_low()?;
        //     driver.g1.set_low()?;
        //     driver.b1.set_low()?;
        //     driver.r2.set_low()?;
        //     driver.g2.set_low()?;
        //     driver.b2.set_low()?;
        //     if C12[y]==1{
        //         driver.r1.set_high()?;
        //         driver.g1.set_high()?;
        //         driver.b1.set_high()?;
        //         driver.r2.set_high()?;
        //         driver.g2.set_high()?;
        //         driver.b2.set_high()?;

        //     }
        //     if l>(MaxLed-12){
        //         driver.stb.set_high()?;
        //     }else{
        //         driver.stb.set_low()?;
        //     }
        //     driver.clk.set_high()?;
        //     driver.delay.delay_us(2);
        //     driver.clk.set_low()?;
        // }
        // driver.stb.set_low()?;
        // driver.clk.set_low()?;

        // // Send Data to control register 12
        // for l in 0..MaxLed{
        //     let y=l%16;
        //     driver.r1.set_low()?;
        //     driver.g1.set_low()?;
        //     driver.b1.set_low()?;
        //     driver.r2.set_low()?;
        //     driver.g2.set_low()?;
        //     driver.b2.set_low()?;
        //     if C13[y]==1{
        //         driver.r1.set_high()?;
        //         driver.g1.set_high()?;
        //         driver.b1.set_high()?;
        //         driver.r2.set_high()?;
        //         driver.g2.set_high()?;
        //         driver.b2.set_high()?;

        //     }
        //     if l>(MaxLed-13){
        //         driver.stb.set_high()?;
        //     }else{
        //         driver.stb.set_low()?;
        //     }
        //     driver.clk.set_high()?;
            
        //     driver.delay.delay_us(2);
        //     driver.clk.set_low()?;           

        // }
        // driver.fill_frame();




        Ok(driver)
    }

    fn fill_frame(&mut self){
        let c_y=Matrix_COLS/2;
        let c_x=Matrix_ROWS/2;
        let box_width=20;
        
        let tl_x=c_x-box_width/2;
        let tl_y=c_y-box_width/2;

        for x in tl_x..(tl_x+box_width){
            self.image_r[x][tl_y]=255;
            self.image_r[x][tl_y+box_width]=255;
        }


        for y in tl_y..(tl_y+box_width){
            self.image_r[tl_x][y]=255;
            self.image_r[tl_x+box_width][y]=255;
        }

        for x in (tl_x+1)..(tl_x+box_width){
            for y in (tl_y+1)..(tl_y+box_width){
                self.image_g[x][y]=255;
            }
        }


    }

    fn set_line(&mut self, line: usize){
     
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
    fn shift(&mut self){
        self.clk.set_high();
        // self.delay.delay_us(1);
        self.clk.set_low();
    }

    fn latch(&mut self){
        self.stb.set_high();
        self.stb.set_low();
    }
    pub fn display_image(&mut self){

        self.fill_frame();

        

        for l in 0..Matrix_ROWS/2{
            self.set_line(l);
            self.oe.set_high();
            
            for c in 0..Matrix_COLS{
                
                let red=self.image_r[l][c];
                let green=self.image_g[l][c];
                let blue=self.image_b[l][c];
                if red>0{
                    self.r1.set_high();                  
                }else{
                    self.r1.set_low();                
                }  
                if green>0{
                    self.g1.set_high();                 
                }else{
                    self.g1.set_low();                     
                } 
                if blue>0{
                    self.b1.set_high();                  
                }else{
                    self.b1.set_low();                     
                } 
                self.shift();
              
                
            }
            self.latch();
            self.oe.set_low();
            self.set_line(l+Matrix_ROWS/2);
            self.oe.set_high();

            
            for c in 0..Matrix_COLS{
               
                let red=self.image_r[l+Matrix_ROWS/2][c];
                let green=self.image_g[l+Matrix_ROWS/2][c];
                let blue=self.image_b[l+Matrix_ROWS/2][c];
                if red>0{
                    self.r2.set_high();                  
                }else{
                    self.r2.set_low();                     
                }  
                if green>0{
                    self.g2.set_high();                  
                }else{
                    self.g2.set_low();                     
                } 
                if blue>0{
                    self.b2.set_high();                  
                }else{
                    self.b2.set_low();                     
                } 
                self.shift();
               
                
            }
            self.latch();



            self.oe.set_low();

        }



        



        
    }
}


