//! `stir` (or Small Threaded Intermediate Representation) is an intermediate
//! representation providing JIT Compilation and an interface to LLVM IR.
//! It aims to improve code speed by analyzing and multithreading code units.
//!
//! `stir` is organized in blocks. The smaller the block, the easier to
//! multithread !

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
