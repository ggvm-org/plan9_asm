use plan9_asm::{operand, register_with_offset, Directive::Nop, ADDQ, JLS, JMP};

fn main() {
    let x: u32 = 1;
    let jmp_to = "somewhere";
    println!("{}", JLS!(33));
    println!("{}", JLS!(@body));
    println!("{}", JLS!(x));
    println!("{}", JLS!(jmp_to));

    println!("{}", JMP!(33));

    println!("{}", register_with_offset!(AX));
    println!("{}", register_with_offset!(16=>AX));

    println!("{}", ADDQ!(AX, SP));
    println!("{}", operand!(AX));
    println!("{}", operand!(1));

    println!("{}", Nop);
}
