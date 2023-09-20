use bitfield::bitfield;
use bitfield_struct::bitfield as bitfield_struct;

#[macro_use]
extern crate bitflags;
extern crate bitfield;
extern crate bitfield_struct;

bitfield! {
    #[derive (Clone, Copy, Debug)]
    struct MyRegField(u32);
    lsb, set_lsb: 1, 0;
    p0,  set_p0: 10, 0;
/*     p1,  set_p1: 10, 11; */
    p2,  set_p2: 10, 0;
    /* msb, set_msb: 1, 31; */
}

union  MyRegFieldUnion{
    field: MyRegField,
    raw : u32,
}

bitflags! {
    struct MyField: u32 {
        const LSB     = 0b00000000_00000000_00000000_00000001;
        const P0      = 0b00000000_00000000_00000111_11111110;
        const P1      = 0b00000000_00011111_11111000_00000000;
        const P2      = 0b01111111_11100000_00000000_00000000;
        const MSB     = 0b10000000_00000000_00000000_00000000;
    }
}

#[bitfield_struct(u32)]
struct MyRegStructRegField {
    #[bits(1)]
    lsb: u32,
    #[bits(10)]
    p0: u32,
    #[bits(10)]
    p1: u32,
    #[bits(10)]
    p2: u32,
    #[bits(1)]
    msb: u32,
}

union MyRegStruct {
    field: MyRegStructRegField,
    raw: u32,
}

fn main() {
    unsafe {
        let mut my_reg = MyRegFieldUnion { raw: 0 };

        println!("my_reg: 0x{:08x}", my_reg.raw);
        my_reg.field.set_lsb(1);
        println!("my_reg: 0x{:08x}", my_reg.raw);
        my_reg.raw = 0;
        my_reg.field.set_p2(1023);
        println!("my_reg: 0x{:08x}", my_reg.raw);
        println!("Hello, world!");

    let mut my_reg_struct: MyRegStruct = MyRegStruct { raw: 0 };
        println!("my_reg_struct: {}", my_reg_struct.raw);
        my_reg_struct.field.set_lsb(1);
        println!("my_reg_struct: {}", my_reg_struct.raw);
        my_reg_struct.raw = 0;
        my_reg_struct.field.set_p2(1023);
        println!("my_reg_struct: 0x{:08x}", my_reg_struct.raw);
    }
}
