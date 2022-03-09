use plan9_asm::directives;
fn main() {
    let x: u32 = 1;
    let jmp_to = "somewhere";

    let directives = directives!(
        JLS 33;
        JLS @body;
        JLS x;
        JLS jmp_to;

        JMP 33;

        ADDQ [AX], [SP];
        SUBQ [16(AX)], [22(R14)];
        NOP;

        @body:

        CALL  runtime.morestack_noctxt;
    );
    println!("{:?}", directives);
}
