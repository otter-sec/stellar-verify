use soroban_env_common::Symbol;

use crate::Env;

trait SymbolExt {
    fn new(env: Env, sym: Symbol) -> Self;
}

impl SymbolExt for Symbol {
    fn new(env: Env, sym: Symbol) -> Self {
        let mut bytes = [0; 10];
        let sym_bytes = sym.as_bytes();
        let mut n = 0;
        while n < sym_bytes.len() {
            bytes[n] = sym_bytes[n];
            n += 1;
        }
        Symbol(bytes)
    }
}
