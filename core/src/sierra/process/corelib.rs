use inkwell::types::BasicMetadataTypeEnum;
use log::debug;

use crate::sierra::errors::CompilerResult;
use crate::sierra::libfunc::math::add::LlvmMathAdd;
use crate::sierra::libfunc::math::sub::LlvmMathSub;
use crate::sierra::libfunc::processor::{Func, LibfuncProcessor};
use crate::sierra::llvm_compiler::{CompilationState, Compiler};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Prepare the libfunc core processors (those are functions from the core lib).
    pub fn prepare_libfunc_processors(&mut self) -> CompilerResult<()> {
        let felt_type = self.types.get("felt").unwrap();
        // Add two felts and return the result.
        let felt_add = "felt_add".to_owned();

        let felt_param: BasicMetadataTypeEnum = felt_type.as_basic_type_enum().into();
        let parameter_types = vec![felt_param, felt_param];
        self.libfunc_processors.insert(
            felt_add,
            Func::new(parameter_types, felt_type.as_basic_type_enum(), Box::from(LlvmMathAdd {})),
        );
        let felt_sub = "felt_sub".to_owned();
        let parameter_types = vec![felt_param, felt_param];
        self.libfunc_processors.insert(
            felt_sub,
            Func::new(parameter_types, felt_type.as_basic_type_enum(), Box::from(LlvmMathSub {})),
        );

        Ok(())
    }

    /// Process core library functions in the Sierra program.
    pub fn process_core_lib_functions(&mut self) -> CompilerResult<()> {
        debug!("processing core lib functions");
        // Check that the current state is valid.
        self.check_state(&CompilationState::TypesProcessed)?;
        // Iterate over the libfunc declarations in the Sierra program.
        for libfunc_declaration in self.program.libfunc_declarations.iter() {
            if let Some(libfunc) = &libfunc_declaration.long_id.generic_id.debug_name {
                let mut func_name = libfunc.to_string();
                if libfunc.ends_with("const") {
                    func_name = self.process_const(libfunc_declaration)?;
                }
                if let Some(processor) = self.libfunc_processors.get(&func_name) {
                    processor.to_llvm(
                        &self.module,
                        self.context,
                        self.builder,
                        libfunc_declaration.id.id.to_string().as_str(),
                    )?;
                }
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::CoreLibFunctionsProcessed)
    }
}
