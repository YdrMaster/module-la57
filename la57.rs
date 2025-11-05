// SPDX-License-Identifier: GPL-2.0

//! La57

mod check;
mod tree;

use kernel::prelude::*;

module! {
    type: La57,
    name: "la57",
    author: "YuanPing <yangderui.ydr@antgroup.com>",
    description: "Check La57 supported",
    license: "GPL",
}

struct La57;

impl kernel::Module for La57 {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        // 1. 检测 CPU 是否支持 LA57
        let cpu_support = check::cpu_la57();
        pr_info!(
            "[1] CPU 支持五级页表: {}\n",
            if cpu_support { "✓ 是" } else { "✗ 否" }
        );

        // 2. 检测内核编译配置
        let kernel_config = check::kernel_config_la57();
        pr_info!(
            "[2] 内核编译支持五级页表 (CONFIG_X86_5LEVEL): {}\n",
            if kernel_config { "✓ 是" } else { "✗ 否" }
        );

        // 3. 检测当前运行状态
        let runtime_enabled = check::runtime_la57();
        pr_info!(
            "[3] 当前内核运行在五级页表模式: {}\n",
            if runtime_enabled {
                "✓ 是"
            } else {
                "✗ 否"
            }
        );

        if runtime_enabled {
            tree::print_page_table_tree(5)
        } else {
            tree::print_page_table_tree(4)
        }

        Ok(Self)
    }
}

impl Drop for La57 {
    fn drop(&mut self) {
        pr_info!("La57 exited\n");
    }
}
