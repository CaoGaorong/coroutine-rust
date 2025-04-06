use std::arch::asm;

use super::{context::Context, coroutine::Coroutine};



pub fn switch<const N: usize>(from: &mut Coroutine<N>, to: &Coroutine<N>) {
    switch_context(&mut from.context, &to.context);
}

#[inline(never)]
#[no_mangle]
fn switch_context(from: &mut Context, to: &Context) {
    asm!(
        // 保存rsp的值
        "mov rax, rsp",

        "mov {from_ret}, rax",
        "mov {from_rbx}, rbx",
        "mov {from_rcx}, rcx",
        "mov {from_rdx}, rdx",
        "mov {from_rbp}, rbp",
        "mov {from_rsp}, rsp",
        "mov {from_rsi}, rsi",
        "mov {from_rdi}, rdi",
        
        "mov {from_r8}, r8",
        "mov {from_r9}, r9",
        "mov {from_r10}, r10",
        "mov {from_r11}, r11",
        "mov {from_r12}, r12",
        "mov {from_r13}, r13",
        "mov {from_r14}, r14",
        "mov {from_r15}, r15",

        "mov rax, {to_ret}",
        "mov rbx, {to_rbx}",
        "mov rcx, {to_rcx}",
        "mov rdx, {to_rdx}",
        "mov rbp, {to_rbp}",
        "mov rsp, {to_rsp}",
        "mov rsi, {to_rsi}",
        "mov rdi, {to_rdi}",
        "mov r8, {to_r8},",
        "mov r9, {to_r9},",
        "mov r10, {to_r10}",
        "mov r11, {to_r11}",
        "mov r12, {to_r12}",
        "mov r13, {to_r13}",
        "mov r14, {to_r14}",
        "mov r15, {to_r15}",

        // 恢复rsp的值
        "mov rsp, rax",
        "ret",

        from_ret = in(reg) &mut from.ret,
        from_rbx = in(reg) &mut from.rbx,
        from_rcx = in(reg) &mut from.rcx,
        from_rdx = in(reg) &mut from.rdx,
        from_rbp = in(reg) &mut from.rbp,
        from_rsp = in(reg) &mut from.rsp,
        from_rsi = in(reg) &mut from.rsi,
        from_rdi = in(reg) &mut from.rdi,
        from_r8 = in(reg) &mut from.r8,
        from_r9 = in(reg) &mut from.r9,
        from_r10 = in(reg) &mut from.r10,
        from_r11 = in(reg) &mut from.r11,
        from_r12 = in(reg) &mut from.r12,
        from_r13 = in(reg) &mut from.r13,
        from_r14 = in(reg) &mut from.r14,
        from_r15 = in(reg) &mut from.r15,

        to_ret = in(reg) &to.ret,
        to_rbx = in(reg) &to.rbx,
        to_rcx = in(reg) &to.rcx,
        to_rdx = in(reg) &to.rdx,
        to_rsp = in(reg) &to.rsp,
        to_rbp = in(reg) &to.rbp,
        to_rsi = in(reg) &to.rsi,
        to_rdi = in(reg) &to.rdi,
        to_r8 = in(reg) &to.r8,
        to_r9 = in(reg) &to.r9,
        to_r10 = in(reg) &to.r10,
        to_r11 = in(reg) &to.r11,
        to_r12 = in(reg) &to.r12,
        to_r13 = in(reg) &to.r13,
        to_r14 = in(reg) &to.r14,
        to_r15 = in(reg) &to.r15,
    );

}
