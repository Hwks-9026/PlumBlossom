use crate::emulator::Emulator;
use crate::registers::Register;
pub(crate) fn nop(em: &mut Emulator, _execution_words: Vec<u32>) {}
pub(crate) fn jmp(em: &mut Emulator, _execution_words: Vec<u32>) {
    em.registers.write(Register::IP as u32, em.registers.read(Register::J as u32));
}
pub(crate) fn cal(em: &mut Emulator, _execution_words: Vec<u32>) {}
pub(crate) fn ret(em: &mut Emulator, _execution_words: Vec<u32>) {}
pub(crate) fn jnz(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn jiz(em: &mut Emulator, execution_words: Vec<u32>) {
    if em.registers.read(execution_words[0]) == 0 {
        jmp(em, execution_words);
    }
}
pub(crate) fn psh(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn pop(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn not(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn shl(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn shr(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn mov(em: &mut Emulator, execution_words: Vec<u32>) {
    let r1 = em.registers.read(execution_words[1]);
    em.registers.write(execution_words[0], r1);
}
pub(crate) fn mvi(em: &mut Emulator, execution_words: Vec<u32>) {
    em.registers.write(execution_words[0], execution_words[1]);
}
pub(crate) fn lda(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn sta(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn add(em: &mut Emulator, execution_words: Vec<u32>) {
    let r1 = em.registers.read(execution_words[1]);
    let r2 = em.registers.read(execution_words[2]);
    em.registers.write(execution_words[0], r1 + r2);
}
pub(crate) fn sub(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn mul(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn div(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn and(em: &mut Emulator, execution_words: Vec<u32>) {}
pub(crate) fn xor(em: &mut Emulator, execution_words: Vec<u32>) {
    let r1 = em.registers.read(execution_words[1]);
    let r2 = em.registers.read(execution_words[2]);
    em.registers.write(execution_words[0], r1 ^ r2);
}

