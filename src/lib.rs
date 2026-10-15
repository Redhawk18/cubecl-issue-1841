//! Reproduction: a `#[cube] fn` also defines a module with the same name, so it collides with
//! modules in places where a plain Rust function doesn't. See README.md.

/// Control: plain Rust allows a function and a module with the same name, because functions
/// live in the value namespace and modules live in the type namespace.
pub mod plain {
    /// A function next to a module with the same name.
    pub mod scale {}

    pub fn scale(x: f32) -> f32 {
        x * 2.0
    }

    /// A function with the same name as the module it is in, imported next to that module.
    mod interpolation {
        pub fn interpolation(x: f32) -> f32 {
            x * 2.0
        }
    }

    use interpolation::interpolation;

    pub fn call(x: f32) -> f32 {
        interpolation(x)
    }
}

/// Bug: the same as `plain::scale`, but with `#[cube]`.
/// error[E0428]: the name `scale` is defined multiple times
#[cfg(feature = "definition_clash")]
pub mod definition_clash {
    use cubecl::prelude::*;

    pub mod scale {}

    #[cube]
    pub fn scale<F: Float>(x: F) -> F {
        x * F::new(2.0)
    }
}

/// Bug: the same as `plain::call`, but with `#[cube]`.
/// error[E0255]: the name `interpolation` is defined multiple times
/// error[E0425]: cannot find function `expand` in module `interpolation`
#[cfg(feature = "import_clash")]
pub mod import_clash {
    use cubecl::prelude::*;

    mod interpolation {
        use cubecl::prelude::*;

        #[cube]
        pub fn interpolation<F: Float>(x: F) -> F {
            x * F::new(2.0)
        }
    }

    use interpolation::interpolation;

    #[cube(launch)]
    fn kernel<F: Float>(input: &[F], output: &mut [F]) {
        if ABSOLUTE_POS < input.len() {
            output[ABSOLUTE_POS] = interpolation::<F>(input[ABSOLUTE_POS]);
        }
    }
}

/// Workaround: skip the `use` and call the function by its full path.
pub mod workaround {
    use cubecl::prelude::*;

    mod interpolation {
        use cubecl::prelude::*;

        #[cube]
        pub fn interpolation<F: Float>(x: F) -> F {
            x * F::new(2.0)
        }
    }

    #[cube(launch)]
    fn kernel<F: Float>(input: &[F], output: &mut [F]) {
        if ABSOLUTE_POS < input.len() {
            output[ABSOLUTE_POS] = interpolation::interpolation::<F>(input[ABSOLUTE_POS]);
        }
    }
}
