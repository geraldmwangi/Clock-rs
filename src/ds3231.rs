use embedded_hal::blocking::i2c::{Write, WriteRead};

const DS3231_ADDRESS: u8 = 0x68;

pub struct Ds3231<I2C> {
    i2c: I2C,
}

impl<I2C, E> Ds3231<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Ds3231 { i2c }
    }

    pub fn get_time(&mut self) -> Result<(u8, u8, u8), E> {
        let mut data = [0; 3];
        self.i2c.write_read(DS3231_ADDRESS, &[0x00], &mut data)?;
        Ok((bcd_to_dec(data[2]), bcd_to_dec(data[1]), bcd_to_dec(data[0])))
    }

    pub fn set_time(&mut self, hours: u8, minutes: u8, seconds: u8) -> Result<(), E> {
        let data = [
            0x00,
            dec_to_bcd(seconds),
            dec_to_bcd(minutes),
            dec_to_bcd(hours),
        ];
        self.i2c.write(DS3231_ADDRESS, &data)
    }

    pub fn get_date(&mut self) -> Result<(u8, u8, u8), E> {
        let mut data = [0; 3];
        self.i2c.write_read(DS3231_ADDRESS, &[0x04], &mut data)?;
        Ok((bcd_to_dec(data[2]), bcd_to_dec(data[1]), bcd_to_dec(data[0])))
    }

    pub fn set_date(&mut self, day: u8, month: u8, year: u8) -> Result<(), E> {
        let data = [
            0x04,
            dec_to_bcd(day),
            dec_to_bcd(month),
            dec_to_bcd(year),
        ];
        self.i2c.write(DS3231_ADDRESS, &data)
    }
}

fn bcd_to_dec(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0x0F)
}

fn dec_to_bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}
