
///
/// outb(port: u16, value: u8)
/// 
/// Writes a byte to the specified port
/// 
/// Unsafe because the caller has to verify that the right port is being used
pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(preserves_flags, nomem, nostack),
    );
}

///
/// inb(port: u16)
/// 
/// Reads a byte from the specified port
/// 
/// Unsafe because the caller must verify that the right port is being used
pub unsafe fn inb(port: u16) -> u8 {
    let mut val: u8 = 0;
    asm!(
        "in al, dx",
        in("dx") port,
        out("al") val,
        options(preserves_flags, nomem, nostack),
    );
    val
}

///
/// outw(port: u16, value: u16)
/// 
/// Writes a word (16 bits) to the specified port
/// 
/// Unsafe because the caller must verify the right port is being used
pub unsafe fn outw(port: u16, value: u16) {
    asm!(
        "out dx, ax",
        in("dx") port,
        in("ax") value,
        options(preserves_flags, nomem, nostack)
    );
}

///
/// inw(port: u16)
/// 
/// Reads a word (16 bits) from the specified port
/// Unsafe because the caller must verify the right port is being used
pub unsafe fn inw(port: u16) -> u16 {
    let mut val: u16 = 0;
    asm!(
        "in ax, dx",
        in("dx") port,
        out("ax") val,
        options(preserves_flags, nomem, nostack),
    );
    val
}

///
/// outl(port: u16, value: u32)
/// 
/// Writes a 32-bit value to the specified port
/// 
/// Unsafe because the caller must verify the right port is being used
pub unsafe fn outl(port: u16, value: u16) {
    asm!(
        "out dx, eax",
        in("dx") port,
        in("eax") value,
        options(preserves_flags, nomem, nostack)
    );
}

///
/// inl(port: u16)
/// 
/// Reads a 32-bit value from the specified port
/// Unsafe because the caller must verify the right port is being used
pub unsafe fn inl(port: u16) -> u16 {
    let mut val: u16 = 0;
    asm!(
        "in eax, dx",
        in("dx") port,
        out("eax") val,
        options(preserves_flags, nomem, nostack),
    );
    val
}