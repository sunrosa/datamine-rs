macro_rules! impl_add_stat {
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
pub(crate) use impl_add_stat;

macro_rules! stat {
    ($visibility:tt $name:ident, $($field:ident);+) => {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(
            rename_all(serialize = "camelCase", deserialize = "camelCase"),
            deny_unknown_fields
        )]
        $visibility struct $name {
            $(
                #[serde(skip_serializing_if = "Option::is_none")]
                pub $field: Option<i32>,
            )*
        }

        $crate::model::macros::impl_add_stat!($name, $($field);+);
    };
}
pub(crate) use stat;
