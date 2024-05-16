macro_rules! impl_add_stats {
    ($type:tt, $($field:ident);+) => {
        impl ::std::ops::Add<&$type> for &$type {
            type Output = $type;
            fn add(self, rhs: &$type) -> Self::Output {
                $type {
                    $(
                        $field: Some(
                            self.$field.unwrap_or_default()
                                + rhs.$field.unwrap_or_default()
                        ),
                    )+
                }
            }
        }
    };
    ($type:tt, $($field:ident);+, $($ref:ident);+) => {
        impl ::std::ops::Add<&$type> for &$type {
            type Output = $type;
            fn add(self, rhs: &$type) -> Self::Output {
                $type {
                    $(
                        $field: Some(
                            self.$field.unwrap_or_default()
                                + rhs.$field.unwrap_or_default()
                        ),
                    )+

                    $(
                        $ref: &self.$ref + &rhs.$ref
                    )+
                }
            }
        }
    };
}
pub(crate) use impl_add_stats;
