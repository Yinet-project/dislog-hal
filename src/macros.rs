macro_rules! define_left_value_right_ref {
    (
        $lhs:ty, 
        $lhs_type:ty, 
        $rhs:ty, 
        $rhs_type:ty,
        $trait_name:ty,
        $fn_name:ident,
        $operator:expr,
        $output:ty
    ) => {
        impl<'b, L:$lhs_type, R: $rhs_type> $trait_name<&'b $rhs<R>> for $lhs<L> {
            type Output = $output;

            fn $fn_name(self, rhs: &'b $rhs<R>) -> $output {
                $operator
            }
        }
    }
}
