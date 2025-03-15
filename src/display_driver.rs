use core::{array, iter::Map};
use cortex_m::asm::delay;
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

use crate::volatile_access;

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
    pub r1: u8,
    pub g1: u8,
    pub b1:u8,
    pub r2: u8,
    pub g2: u8,
    pub b2: u8,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub clk: u8,
    pub stb:u8,
    pub oe: u8,
   

    image_r: [[u8;Matrix_COLS];Matrix_ROWS],
    image_g: [[u8;Matrix_COLS];Matrix_ROWS],
    image_b:[[u8;Matrix_COLS];Matrix_ROWS]
}

impl DisplayDriver {
    pub fn new(pins: Pins, core: pac::CorePeripherals,clocks: ClocksManager) -> Result<Self,Error> {
        let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        
        let mut driver=Self {
            r1: 2,
            g1: 3,
            b1: 4,
            r2: 5,
            g2: 8,
            b2: 9,
            a: 10,
            b: 16,
            c: 18,
            d: 20,
            e: 22,
            clk: 11,
            stb:12,
            oe:13,
            delay,
            image_r: [[0;Matrix_COLS];Matrix_ROWS],
            image_g: [[0;Matrix_COLS];Matrix_ROWS],
            image_b: [[0;Matrix_COLS];Matrix_ROWS],
          
        };
        pins.gpio2.into_push_pull_output();
        pins.gpio3.into_push_pull_output();
        pins.gpio4.into_push_pull_output();
        pins.gpio5.into_push_pull_output();
        pins.gpio8.into_push_pull_output();
        pins.gpio9.into_push_pull_output();
        pins.gpio10.into_push_pull_output();
        pins.gpio16.into_push_pull_output();
        pins.gpio18.into_push_pull_output();
        pins.gpio20.into_push_pull_output();
        pins.gpio22.into_push_pull_output();
        pins.gpio11.into_push_pull_output();
        pins.gpio12.into_push_pull_output();
        pins.gpio13.into_push_pull_output();




        Ok(driver)
    }

    pub fn fill_frame(&mut self){
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

    fn set_high(&self,pin: u8){
        unsafe {
            // Set pin 25 high using GPIO_OUT_SET register
            (*rp2040_pac::SIO::ptr()).gpio_out_set().write(|w| w.bits(1 << pin));
        }
    }

    fn set_low(&self,pin: u8){
        unsafe {
            // Set pin 25 high using GPIO_OUT_SET register
            (*rp2040_pac::SIO::ptr()).gpio_out_clr().write(|w| w.bits(1 << pin));
        }
    }

    fn set_high_bits(&self,pin_mask: u32){
        unsafe {
            // Set pin 25 high using GPIO_OUT_SET register
            (*rp2040_pac::SIO::ptr()).gpio_out_set().write(|w| w.bits(pin_mask));
        }
    }

    fn set_low_bits(&self,pin_mask: u32){
        unsafe {
            // Set pin 25 high using GPIO_OUT_SET register
            (*rp2040_pac::SIO::ptr()).gpio_out_clr().write(|w| w.bits(pin_mask));
        }
    }
    fn set_line(&mut self, line: usize){
        let mut high=0;
        let mut low=0;
        if line&1==1{
            high=high|(1<<self.a);//self.set_high(self.a);
        }
        else{
            low=low|(1<<self.a);
        }
        if line&2==2{
            high=high|(1<<self.b);//self.set_high(self.a);
        }
        else{
            low=low|(1<<self.b);
        }
        if line&4==4{
            high=high|(1<<self.c);//self.set_high(self.a);
        }
        else{
            low=low|(1<<self.c);
        }
        if line&8==8{
            high=high|(1<<self.d);//self.set_high(self.a);
        }
        else{
            low=low|(1<<self.d);
        }
        if line&16==16{
            high=high|(1<<self.e);//self.set_high(self.a);
        }
        else{
            low=low|(1<<self.e);
        }

        self.set_high_bits(high);
        self.set_low_bits(low);
    }
    fn shift(&mut self){



        self.set_high(self.clk);     
        self.set_low(self.clk);
        // self.clk.set_high();
        // self.clk.set_low();

       
        // volatile_access::set_pin_high(11);
        // volatile_access::set_pin_low(11);
    }

    fn latch(&mut self){
        self.set_high(self.stb);
        self.set_low(self.stb);
    }
    pub fn display_image(&mut self){

        // self.fill_frame();

        // self.oe.set_low();

        for l in 0..Matrix_ROWS/2{
            self.set_line(l);
            // self.set_high(self.oe);
            
            for c in 0..Matrix_COLS{
                
                let red=self.image_r[l][c];
                let green=self.image_g[l][c];
                let blue=self.image_b[l][c];
                let mut high=0;
                let mut low=0;
                if red>0{
                    high=high|(1<<self.r1);
                }else{
                    low=low|(1<<self.r1);               
                }  
                if green>0{
                    high=high|(1<<self.g1);
                }else{
                    low=low|(1<<self.g1);                      
                } 
                if blue>0{
                    high=high|(1<<self.b1);
                }else{
                    low=low|(1<<self.b1);                      
                }
                self.set_high_bits(high);
                self.set_low_bits(low); 
                self.shift();
              
                
            }
            self.latch();
            self.set_high(self.oe);
            self.set_low(self.oe);
            self.set_line(l+Matrix_ROWS/2);
            self.set_high(self.oe);
            self.set_low(self.oe);          

            
            for c in 0..Matrix_COLS{
               
                let red=self.image_r[l+Matrix_ROWS/2][c];
                let green=self.image_g[l+Matrix_ROWS/2][c];
                let blue=self.image_b[l+Matrix_ROWS/2][c];
                let mut high=0;
                let mut low=0;
                if red>0{
                    high=high|(1<<self.r2);
                }else{
                    low=low|(1<<self.r2);               
                }  
                if green>0{
                    high=high|(1<<self.g2);
                }else{
                    low=low|(1<<self.g2);                      
                } 
                if blue>0{
                    high=high|(1<<self.b2);
                }else{
                    low=low|(1<<self.b2);                      
                }
                self.set_high_bits(high);
                self.set_low_bits(low); 
                self.shift();
               
                
            }
            self.latch();


            self.set_high(self.oe);
            self.set_low(self.oe);

        }
        // self.oe.set_high();
    
    
      



        
    }
}


