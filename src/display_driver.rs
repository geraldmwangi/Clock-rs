use core::{array, iter::Map};

use cortex_m::delay;
use ds323x::{NaiveTime, Timelike};
use rp_pico::hal::{clocks::ClocksManager, gpio::Error, pac};
use embedded_hal::{digital::{OutputPin, StatefulOutputPin}};
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
use crate::{flower::FlowerPattern, led_matrix::LedPattern};

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
    delay: cortex_m::delay::Delay,
   

    image_r: [[bool;Matrix_COLS];Matrix_ROWS],
    image_g: [[bool;Matrix_COLS];Matrix_ROWS],
    image_b:[[bool;Matrix_COLS];Matrix_ROWS]
}

impl DisplayDriver {
    pub fn new(delay: cortex_m::delay::Delay ) -> Result<Self,Error> {

        
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
            image_r: [[false;Matrix_COLS];Matrix_ROWS],
            image_g: [[false;Matrix_COLS];Matrix_ROWS],
            image_b: [[false;Matrix_COLS];Matrix_ROWS],
            delay,
          
        };





        Ok(driver)
    }

    pub fn fill_frame(&mut self, time: NaiveTime) {
        let c_y = Matrix_COLS / 2;
        let c_x = Matrix_ROWS / 2;
        let width=4;
        let sec=time.second();
        let sec_pattern=LedPattern::from_bidigit_number(sec);
        let min_pattern=LedPattern::from_bidigit_number(time.minute());
        let hour_pattern=LedPattern::from_bidigit_number(time.hour());
        let colon=LedPattern::colon();

        // self.display_pattern(hour_pattern.0, 0, 0);
        // self.display_pattern(hour_pattern.1, width, 0);
        // self.display_pattern(colon, 2*width, 0);
        // self.display_pattern(min_pattern.0, 3*width, 0);
        // self.display_pattern(min_pattern.1, 4*width, 0); 
        // self.display_pattern(LedPattern::colon(), 5*width, 0);   
        // self.display_pattern(sec_pattern.0, 6*width, 0);
        // self.display_pattern(sec_pattern.1, 7*width, 0);   
        self.display_flower(FlowerPattern::new());

        
        // let box_width = 30;

        // let tl_x = c_x - box_width / 2;
        // let tl_y = c_y - box_width / 2;

        // for x in tl_x..=tl_x + box_width {
        //     self.image_b[x][tl_y] = true;
        //     self.image_b[x][tl_y + box_width] = true;
        // }

        // for y in tl_y..=tl_y + box_width {
        //     self.image_r[tl_x][y] = true;
        //     self.image_r[tl_x + box_width][y] = true;
        // }

        // for x in (tl_x + 1)..tl_x + box_width {
        //     for y in (tl_y + 1)..tl_y + box_width {
        //         self.image_g[x][y] = true;
        //     }
        // }
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

    fn set_pins(&self, high_mask: u32, low_mask: u32) {
        unsafe {
            (*rp2040_pac::SIO::ptr()).gpio_out_set().write(|w| w.bits(high_mask));
            (*rp2040_pac::SIO::ptr()).gpio_out_clr().write(|w| w.bits(low_mask));
        }
    }

    fn set_line(&mut self, line: usize) {
        let mut row_high = 0;
        let mut row_low = 0;
        if line & 1 == 1 { row_high |= 1 << self.a; } else { row_low |= 1 << self.a; }
        if line & 2 == 2 { row_high |= 1 << self.b; } else { row_low |= 1 << self.b; }
        if line & 4 == 4 { row_high |= 1 << self.c; } else { row_low |= 1 << self.c; }
        if line & 8 == 8 { row_high |= 1 << self.d; } else { row_low |= 1 << self.d; }
        if line & 16 == 16 { row_high |= 1 << self.e; } else { row_low |= 1 << self.e; }

        self.set_pins(row_high, row_low);
    }

    fn shift(&mut self) {
        self.set_high(self.clk);
        self.delay.delay_us(1);
        self.set_low(self.clk);
    }

    fn latch(&mut self) {
        self.set_high(self.stb);
        self.delay.delay_us(1);
        
        self.set_low(self.stb);
    }
    fn output_enable(&mut self){
        
        
        self.set_low(self.oe);
        // self.delay.delay_us(1000);
      
    }

    pub fn display_image(&mut self) {
        for l in 0..Matrix_ROWS / 2 {
           //from https://benjemmett.com/archives/114
            //Shift out the data
            for c in 0..Matrix_COLS {
                let red = self.image_r[l][c];
                let green = self.image_g[l][c];
                let blue = self.image_b[l][c];
                let mut high = 0;
                let mut low = 0;
                high |= (red as u32) << self.r1; //if red > 0 { high |= 1 << self.r1; } else { low |= 1 << self.r1; }
                low|= (!red as u32) << self.r1;
                high |= (green as u32) << self.g1;//if green > 0 { high |= 1 << self.g1; } else { low |= 1 << self.g1; }
                low|= (!green as u32) <<self.g1;
                high |= (blue as u32) << self.b1;//if blue > 0 { high |= 1 << self.b1; } else { low |= 1 << self.b1; }
                low|= (!blue as u32) << self.b1;
                //high |= 1 << self.b1;//if blue > 0 { high |= 1 << self.b1; } else { low |= 1 << self.b1; }
                self.set_pins(high, low);
                 self.shift();
                //self.set_pins(1 << self.clk, 1 << self.clk);
            }

            //Disable the display by setting OR to High
            self.set_high(self.oe);

            //Set the row adress
            self.set_line(l);

            //latch the row
            self.latch();
            //self.set_pins(1 << self.stb, 1 << self.stb);

            //Enable the output and wait
             self.output_enable();
            //self.set_pins(1 << self.oe, 1 << self.oe);

            
            // self.set_pins(1 << self.oe, 1 << self.oe);

             //Shift out the data
            for c in 0..Matrix_COLS {
                let red = self.image_r[l + Matrix_ROWS / 2][c];
                let green = self.image_g[l + Matrix_ROWS / 2][c];
                let blue = self.image_b[l + Matrix_ROWS / 2][c];
                let mut high = 0;
                let mut low = 0;
                // if red > 0 { high |= 1 << self.r2; } else { low |= 1 << self.r2; }
                // if green > 0 { high |= 1 << self.g2; } else { low |= 1 << self.g2; }
                // if blue > 0 { high |= 1 << self.b2; } else { low |= 1 << self.b2; }
                high |= (red as u32) << self.r2; //if red > 0 { high |= 1 << self.r1; } else { low |= 1 << self.r1; }
                low|= (!red as u32) << self.r2;
                high |= (green as u32) << self.g2;//if green > 0 { high |= 1 << self.g1; } else { low |= 1 << self.g1; }
                low|= (!green as u32) <<self.g2;
                high |= (blue as u32) << self.b2;//if blue > 0 { high |= 1 << self.b1; } else { low |= 1 << self.b1; }
                low|= (!blue as u32) << self.b2;
                self.set_pins(high, low);
                 self.shift();
                //self.set_pins(1 << self.clk, 1 << self.clk);
            }
             //Disable the display by setting OR to High
            self.set_high(self.oe);

            //Set the row adress
            self.set_line(l + Matrix_ROWS / 2);

            //latch the row
            self.latch();
            //self.set_pins(1 << self.stb, 1 << self.stb);

            //Enable the output and wait
            self.output_enable();
            //self.set_pins(1 << self.oe, 1 << self.oe);
        }
    }

    pub fn display_flower(&mut self, pattern: FlowerPattern){
        for y in 0..pattern.height {
            for x in 0..pattern.width {
                if x < Matrix_COLS && y  < Matrix_ROWS {
                    self.image_r[y ][x ] = pattern.head[y][x];
                    self.image_g[y ][x ] = pattern.stem[y][x];
                }
            }
        }   
    }
    pub fn display_pattern(&mut self, pattern: LedPattern, x_offset: usize, y_offset: usize) {
        

        for y in 0..pattern.height {
            for x in 0..pattern.width {
                if x + x_offset < Matrix_COLS && y + y_offset < Matrix_ROWS {
                    self.image_r[y + y_offset][x + x_offset] = pattern.data[y][x];
                }
            }
        }
    }

    
}


