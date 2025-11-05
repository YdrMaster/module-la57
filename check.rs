use core::arch::asm;

/// 检测 CPU 是否支持 LA57
/// 使用 CPUID 指令 (EAX=7, ECX=0)，检查 ECX bit 16
pub(super) fn cpu_la57() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        let ecx: u32;
        unsafe {
            asm!(
                "push rbx",           // 保存 rbx
                "mov eax, 0x7",       // CPUID 功能号
                "xor ecx, ecx",       // 子功能号 0
                "cpuid",
                "pop rbx",            // 恢复 rbx
                out("ecx") ecx,
                out("eax") _,
                out("edx") _,
            )
        }
        // LA57 = ECX bit 16
        (ecx & (1 << 16)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

/// 检测内核编译配置
pub(super) const fn kernel_config_la57() -> bool {
    // 检查编译时配置
    #[cfg(CONFIG_X86_5LEVEL)]
    {
        true
    }
    #[cfg(not(CONFIG_X86_5LEVEL))]
    {
        false
    }
}

/// 检测运行时是否启用 LA57（CR4 寄存器的 bit 12）
pub(super) fn runtime_la57() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        let cr4: u64;
        unsafe { asm!("mov {}, cr4",out(reg) cr4) }
        // LA57 = CR4 bit 12
        (cr4 & (1 << 12)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}
