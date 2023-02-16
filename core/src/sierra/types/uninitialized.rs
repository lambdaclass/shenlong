use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Parses the Uninitialized type declaration.
    pub fn uninitialized(&mut self, type_declaration: &TypeDeclaration) {
        match &type_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => self.types.insert(
                type_declaration.id.id.to_string(),
                Box::from(
                    self.types
                        .get(&id.to_string())
                        .expect("store_temp type should have been declared")
                        .as_basic_type_enum(),
                ),
            ),
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("store_temp only takes type or user type")
            }
        };
    }
}
