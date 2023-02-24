use cairo_lang_sierra::program::LibfuncDeclaration;
use inkwell::types::BasicType;
use inkwell::IntPredicate;

use super::DEFAULT_PRIME;
use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;
use crate::sierra::process::corelib::PRINT_DOUBLE_FELT_FUNC;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a felt division.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of felt_div.
    ///
    /// # Error
    ///
    /// Returns an error if the felt type has not been declared previously.
    pub fn felt_div(&self, libfunc_declaration: &LibfuncDeclaration) {
        // We could hardcode the LLVM IR type for felt but this adds a check.
        let felt_type = self.get_type_from_name("felt").unwrap();
        // fn felt_div(a: felt, b: felt) -> felt
        let func = self.module.add_function(
            libfunc_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
            felt_type.fn_type(&[felt_type.as_basic_type_enum().into(), felt_type.as_basic_type_enum().into()], false),
            None,
        );

        // blocks needed
        let entry_block = self.context.append_basic_block(func, "entry");
        let while_loop = self.context.append_basic_block(func, "while");
        let body_loop = self.context.append_basic_block(func, "body");
        let exit_loop = self.context.append_basic_block(func, "exit");

        self.builder.position_at_end(entry_block);

        // The maximum value of a multiplication is (prime - 1)² which is 503 bits.
        let double_felt = self.context.custom_width_int_type(503);

        let prime_val = double_felt
            .const_int_from_string(DEFAULT_PRIME, inkwell::types::StringRadix::Decimal)
            .expect("Should have been able to parse the prime");

        // Compute felt division = a * b^-1
        // Calculate the multiplicative inverse of b

        // allocate needed stack variables
        let x = self.builder.build_alloca(double_felt, "x");
        let y = self.builder.build_alloca(double_felt, "y");
        let r = self.builder.build_alloca(double_felt, "r");
        let s = self.builder.build_alloca(double_felt, "s");
        // let quotient = self.builder.build_alloca(double_felt, "quotient");

        // store their initial values
        self.builder.build_store(x, double_felt.const_int(0, false));
        self.builder.build_store(y, double_felt.const_int(1, false));
        self.builder.build_store(r, prime_val);
        self.builder.build_store(
            s,
            self.builder.build_int_s_extend(func.get_last_param().unwrap().into_int_value(), double_felt, "s"),
        );

        self.builder.build_unconditional_branch(while_loop);
        self.builder.position_at_end(while_loop);

        // while (new_r != 0)
        let is_divisor_zero = self.builder.build_int_compare(
            IntPredicate::NE,
            self.builder.build_load(double_felt, s, "s_check").into_int_value(),
            double_felt.const_zero(),
            "while_compare",
        );
        self.builder.build_conditional_branch(is_divisor_zero, body_loop, exit_loop);
        self.builder.position_at_end(body_loop);
        self.call_printf("loop body:\n", &[]);
        let r_val = self.builder.build_load(double_felt, r, "r").into_int_value();
        let s_val = self.builder.build_load(double_felt, s, "s").into_int_value();
        let q = self.builder.build_int_signed_div(r_val, s_val, "q");
        self.call_printf("q = r / s\n", &[]);
        self.call_printf("q = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, q.into());
        self.call_printf("r = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, r_val.into());
        self.call_printf("s = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, s_val.into());
        self.call_printf("r, s = s, r - q * s\n", &[]);
        let q_mul_s = self.builder.build_int_signed_rem(
            self.builder.build_int_mul(q, s_val, "q_mul_s"),
            prime_val,
            "q_mul_s_mod",
        );
        self.call_printf("q * s = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, q_mul_s.into());
        let new_s = self.builder.build_int_signed_rem(
            self.builder.build_int_sub(r_val, q_mul_s, "new_s"),
            prime_val,
            "new_s_mod",
        );
        self.call_printf("s = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, new_s.into());
        let new_r = self.builder.build_load(double_felt, s, "new_r");
        self.call_printf("r = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, new_r.into());
        self.builder.build_store(s, new_s);
        self.builder.build_store(r, new_r);

        let x_val = self.builder.build_load(double_felt, x, "x").into_int_value();
        let y_val = self.builder.build_load(double_felt, y, "y").into_int_value();
        self.call_printf("x = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, x_val.into());
        self.call_printf("y = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, y_val.into());
        self.call_printf("x, y = y, x - q * y\n", &[]);
        let q_mul_y = self.builder.build_int_signed_rem(
            self.builder.build_int_mul(q, y_val, "q_mul_y"),
            prime_val,
            "q_mul_y_mod",
        );
        self.call_printf("q * y = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, q_mul_y.into());
        let new_y = self.builder.build_int_sub(x_val, q_mul_y, "new_y");
        let new_x = self.builder.build_load(double_felt, y, "new_x");
        self.call_printf("x = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, new_x.into());
        self.call_printf("y = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, new_y.into());
        self.builder.build_store(y, new_y);
        self.builder.build_store(x, new_x);

        self.builder.build_unconditional_branch(while_loop);

        // At this point, t holds the inverse of b.

        self.builder.position_at_end(exit_loop);

        self.call_printf("out of loop\n", &[]);

        // Extend left hand side.
        let lhs = self.builder.build_int_s_extend(
            func.get_first_param().expect("felt_div should have a first arg").into_int_value(),
            double_felt,
            "extended_a",
        );

        let rhs = self.builder.build_load(double_felt, x, "inverse").into_int_value();
        self.call_printf("x (inverse) = ", &[]);
        self.call_print_type(PRINT_DOUBLE_FELT_FUNC, rhs.into());

        let mul = self.builder.build_int_mul(lhs, rhs, "res");
        // Panics if the function doesn't have enough arguments but it shouldn't happen since we just
        // defined it above.
        // Also panics if the modulo function doesn't return a value but it shouldn't happen.
        // return a * b^-1 % prime
        let res = self
            .builder
            .build_call(
                self.module.get_function("modulo").expect("Modulo should have been defined before"),
                &[mul.into()],
                "res",
            )
            .try_as_basic_value()
            .left()
            .expect("Should have a left return value");
        self.builder.build_return(Some(&res));
    }
}
