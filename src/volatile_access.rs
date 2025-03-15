use core::marker::PhantomData;

 use rp_pico::pac::sio::{GPIO_OUT, GPIO_OUT_CLR, GPIO_OUT_SET, GPIO_OUT_XOR};
// // RP2040 GPIO register base addresses
// const GPIO_BASE: usize = 0x40014000;
// const GPIO_OUT: usize = GPIO_BASE + 0x010;
// const GPIO_OUT_SET: usize = GPIO_BASE + 0x014;
// const GPIO_OUT_CLR: usize = GPIO_BASE + 0x018;
// const GPIO_OUT_XOR: usize = GPIO_BASE + 0x01C;
// struct RegisterBlock {
//     out: VolatileRegister<u32>,
//     out_set: VolatileRegister<u32>,
//     out_clr: VolatileRegister<u32>,
//     out_xor: VolatileRegister<u32>,
// }

// struct VolatileRegister<T> {
//     ptr: *mut T,
//     _marker: PhantomData<T>,
// }

// impl<T> VolatileRegister<T> {
//     const fn new(address: usize) -> Self {
//         Self {
//             ptr: address as *mut T,
//             _marker: PhantomData,
//         }
//     }

//     fn read(&self) -> T where T: Copy {
//         unsafe { core::ptr::read_volatile(self.ptr) }
//     }

//     fn write(&self, value: T) {
//         unsafe { core::ptr::write_volatile(self.ptr, value) }
//     }
// }

// const GPIO_REGISTER_BLOCK: RegisterBlock = RegisterBlock {
//     out: VolatileRegister::new(GPIO_OUT),
//     out_set: VolatileRegister::new(GPIO_OUT_SET),
//     out_clr: VolatileRegister::new(GPIO_OUT_CLR),
//     out_xor: VolatileRegister::new(GPIO_OUT_XOR),
// };

pub fn set_pin_high(pin: u8) {
    // GPIO_REGISTER_BLOCK.out_set.write(1 << pin);
    // GPIO_OUT_SET{ register: todo!(), _marker: PhantomData }
}

pub fn set_pin_low(pin: u8) {
    // GPIO_REGISTER_BLOCK.out_clr.write(1 << pin);
}