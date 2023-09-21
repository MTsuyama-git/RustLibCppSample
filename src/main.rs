use bitfield_struct::bitfield;

extern crate bitfield_struct;

#[bitfield(u32)]
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
        let mut my_reg_struct: MyRegStruct = MyRegStruct { raw: 0 };
        println!("my_reg_struct: {}", my_reg_struct.raw);
        my_reg_struct.field.set_lsb(1);
        println!("my_reg_struct: {}", my_reg_struct.raw);
        my_reg_struct.raw = 0;
        my_reg_struct.field.set_p2(1023);
        println!("my_reg_struct: 0x{:08x}", my_reg_struct.raw);
    }
}
