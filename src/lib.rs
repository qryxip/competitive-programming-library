// This file is automatically generated by `cargo-online-judge-verification-helper-helper`.
//! Re-exports the library crates for rustdoc.
//!
//! This crate itself is not intended to be used directly.
pub mod graph {
    pub mod dijkstra {
        #![doc = " Performs Dijkstra's algorithm.\n\n # Example\n\n ```\n // [0] --1-> [1] --999-> [4]\n //            |           ↑\n //            1           1\n //            ↓           |\n //           [2] ---1--> [3]\n\n let start = 0;\n let graph = vec![\n     vec![(1, 1)],\n     vec![(2, 1), (4, 999)],\n     vec![(3, 1)],\n     vec![(4, 1)],\n     vec![],\n ];\n\n let costs = dijkstra::costs(\n     start,\n     |i| graph[i].iter().copied(),\n     vec![u64::max_value(); graph.len()],\n     |graph, i| &mut graph[i],\n );\n\n assert_eq!([0, 1, 2, 3, 4], *costs);\n ```"]
        pub use ::dijkstra::*;
    }
}
pub mod integer {
    pub mod gcd {
        #![doc = " Calculates a Greatest Common Divisor (GCD).\n\n ```\n assert_eq!(2, gcd::gcd(10, 2));\n assert_eq!(1, gcd::gcd(10, 3));\n assert_eq!(14, gcd::gcd(56, 42));\n ```"]
        pub use ::gcd::*;
    }
    pub mod montmort_number {
        pub use ::montmort_number::*;
    }
    pub mod partition_point {
        pub use ::partition_point::*;
    }
    pub mod xorshift {
        pub use ::xorshift::*;
    }
}
pub mod io {
    pub mod buffered_print {
        pub use ::buffered_print::*;
    }
    pub mod input {
        pub use ::input::*;
    }
}
pub mod modulo {
    pub mod tonelli_shanks {
        pub use ::tonelli_shanks::*;
    }
}
pub mod prime {
    pub mod miller_rabin {
        pub use ::miller_rabin::*;
    }
    pub mod rho {
        pub use ::rho::*;
    }
    pub mod sieve {
        pub use ::sieve::*;
    }
}
