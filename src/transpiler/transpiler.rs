use crate::parser::types::CompilationTarget;

pub fn transpile_all_targets (mut uncompiled: Vec<CompilationTarget>) -> Vec<CompilationTarget> {
    for target in &uncompiled {
        compile_target(target);
    }
    uncompiled
}

fn compile_target (mut target: &CompilationTarget) {
    
}
