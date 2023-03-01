use std::process::{Command, Stdio};

use honggfuzz::fuzz;
use num_bigint::BigInt;
use num_traits::Num;
use serde::Serialize;
use shenlong_core::sierra::llvm_compiler::Compiler;

use crate::{get_prime, test_template_file};

#[derive(Serialize)]
pub struct BinaryContext {
    lhs: String,
    rhs: String,
    op: String,
}

#[inline(always)]
pub fn operation(case: &str) {
    fuzz!(|data: (&[u8], &[u8])| {
        let prime = get_prime();

        let lhs = BigInt::from_bytes_be(
            if data.0.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            data.0,
        ) % &prime;
        let rhs = BigInt::from_bytes_be(
            if data.1.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            data.1,
        ) % &prime;

        let mut expected = match case {
            "add" => (&lhs + &rhs) % prime,
            "sub" => (&lhs - &rhs) % prime,
            "mul" => (&lhs * &rhs) % prime,

            _ => panic!("invalid case: {case:}"),
        };
        let ctx = BinaryContext { lhs: lhs.to_string(), rhs: rhs.to_string(), op: case.to_owned() };
        let source = test_template_file!("operation.sierra", ctx);
        let tmp = tempdir::TempDir::new("test_simple_operation").unwrap();
        let file = tmp.into_path().join("output.ll");

        Compiler::compile_from_code(&source, &file, None).unwrap();
        let lli_path = std::env::var("LLI_PATH").expect("LLI_PATH must exist and point to the `lli` tool from llvm 16");

        let cmd = Command::new(lli_path).arg(file).stdout(Stdio::piped()).spawn().unwrap();

        let output = cmd.wait_with_output().unwrap();
        let output = std::str::from_utf8(&output.stdout).unwrap().trim();

        assert!(output.starts_with("Return value: "));
        let output = &output["Return value: ".len()..];
        let x = BigInt::from_str_radix(output, 16).unwrap();
        let two = BigInt::from(2).pow(x.bits() as u32);
        expected = expected.modpow(&BigInt::from(1), &two);
        assert_eq!(x, expected);
    });
}