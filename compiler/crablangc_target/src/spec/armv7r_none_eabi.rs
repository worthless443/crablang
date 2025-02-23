// Targets the Little-endian Cortex-R4/R5 processor (ARMv7-R)

use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "armv7r-unknown-none-eabi".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),

        options: TargetOptions {
            abi: "eabi".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("crablang-lld".into()),
            relocation_model: RelocModel::Static,
            panic_strategy: PanicStrategy::Abort,
            max_atomic_width: Some(32),
            emit_debug_gdb_scripts: false,
            // GCC and Clang default to 8 for arm-none here
            c_enum_min_bits: Some(8),
            ..Default::default()
        },
    }
}
