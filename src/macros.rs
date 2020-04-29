macro_rules! macros_l_self_r_ref_inner {
    ($self_:ident, $fn_name: ident, $hs_type: ident, $rhs_o: ident, $body:expr, $output: ty) => {
        fn $fn_name($self_, $rhs_o: &'b $hs_type<T>) -> $output {
            $body
        }
    };
}

macro_rules! macros_l_self_r_val_inner {
    ($self_:ident, $fn_name: ident, $hs_type: ident, $rhs_o: ident, $body:expr, $output: ty) => {
        fn $fn_name($self_, $rhs_o: $hs_type<T>) -> $output {
            $body
        }
    };
}

macro_rules! define_l_val_r_ref {
    (
        $hs_type: ident,
        $hs_a: ident,
        $trait_name: ident,
        $fn_name: ident,
        $output: ty
    ) => {
        impl<'b, T: $hs_a> $trait_name<&'b $hs_type<T>> for $hs_type<T> {
            type Output = $output;

            macros_l_self_r_ref_inner! {self, $fn_name, $hs_type, rhs_o, { (&self).$fn_name(rhs_o) }, $output}
        }
    };
}

macro_rules! define_l_val_r_val {
    (
        $hs_type: ident,
        $hs_a: ident,
        $trait_name: ident,
        $fn_name: ident,
        $output: ty
    ) => {
        impl<T: $hs_a> $trait_name<$hs_type<T>> for $hs_type<T> {
            type Output = $output;

            macros_l_self_r_val_inner! {self, $fn_name, $hs_type, rhs_o, { (&self).$fn_name(&rhs_o) }, $output}
        }
    };
}

macro_rules! define_l_ref_r_val {
    (
        $hs_type: ident,
        $hs_a: ident,
        $trait_name: ident,
        $fn_name: ident,
        $output: ty
    ) => {
        impl<'a, T: $hs_a> $trait_name<$hs_type<T>> for &'a $hs_type<T> {
            type Output = $output;

            macros_l_self_r_val_inner! {self, $fn_name, $hs_type, rhs_o, { self.$fn_name(&rhs_o) }, $output}
        }
    };
}
