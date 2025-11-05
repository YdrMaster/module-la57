use core::arch::asm;
use kernel::prelude::*;

/// 读取 CR3 寄存器的值（根页表物理地址）
fn get_cr3() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let cr3: u64;
        unsafe { asm!("mov {}, cr3", out(reg) cr3) }
        cr3
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

/// 页表项标志位（简化版）
const PTE_PRESENT: u64 = 1 << 0; // Present bit
const PTE_PS: u64 = 1 << 7; // Page Size (大页)

/// 将物理地址转换为虚拟地址（简化）
fn phys_to_virt(phys: u64) -> *const u64 {
    let offset = unsafe { kernel::bindings::page_offset_base };
    (offset + phys) as *const u64
}

/// 递归打印页表树
/// current_level: 当前级别 (5=PML5, 4=PML4, etc.)
/// base_phys: 当前页表的物理基址
/// prefix: 树状打印前缀 (e.g., "")
fn print_page_table_recursive(current_level: u32, base_phys: u64, prefix: &mut Vec<u8>) {
    let level_name = match current_level {
        5 => "PML5",
        4 => "PML4",
        3 => "PDPT",
        2 => "PD",
        1 => "PT",
        0 => return,
        _ => unreachable!("Invalid pt level"),
    };
    let prefix_str = unsafe { core::str::from_utf8_unchecked(prefix.as_slice()) };
    pr_info!("{prefix_str}{level_name} 页表 (物理: {base_phys:#018x})\n");

    let base_virt = phys_to_virt(base_phys);

    for i in 0..512 {
        let pte = unsafe { base_virt.add(i).read() };
        let item_prefix = if i == 511 { "└── " } else { "├── " };
        let child_prefix = if i == 511 { "    " } else { "│   " };

        if (pte & PTE_PRESENT) != 0 {
            let next_phys = pte & !0xfff;
            let flags = if (pte & PTE_PS) != 0 { "大页" } else { "" };
            let prefix_str = unsafe { core::str::from_utf8_unchecked(prefix.as_slice()) };
            pr_info!("{prefix_str}{item_prefix}项 {i:03}: {pte:#018x}{flags}\n");

            if (pte & PTE_PS) == 0 && current_level > 1 {
                // 递归到下一级
                prefix
                    .try_extend_from_slice(child_prefix.as_bytes())
                    .unwrap();
                print_page_table_recursive(current_level - 1, next_phys, prefix);
                prefix.truncate(prefix.len() - child_prefix.len())
            }
        }
    }
}

/// 打印整个页表树（从根开始）
pub(super) fn print_page_table_tree(level: u32) {
    let cr3 = get_cr3();
    let root_phys = cr3 & !0xfff;
    pr_info!("页表树结构:\n");
    print_page_table_recursive(level, root_phys, &mut Vec::new())
}
