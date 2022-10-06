/// Usage:
/// ```
/// use cosmwasm_std::Uint256;
/// use ethnum::U256;
/// use better_secret_math::make_btr;
/// make_btr!{
///     Rebase {
///        elastic: Uint256, U256, "";
///        base: Uint256, U256, ""
///     }
/// }
///
/// pub struct BtrRebase {
///  pub elastic: U256,
///  pub base: U256,
/// }
///
/// pub struct Rebase {
///  pub elastic: Uint256,
///  pub base: Uint256,
/// }
/// ```
/// The two structs above also derive from using the [derive-from-ext] crate.
///
///
///
///
///
#[macro_export]
macro_rules! make_btr {
    ($(#[$meta:meta])* $struct:ident {$($element: ident: $ty: ty, $btr_ty: ty, $doc:expr); *}) => {
        $crate::make_better! {
            $(#[$meta])* "btr" $struct {$($element: $ty, $btr_ty, $doc); *}
        }
    }
}

/// Same as [make_btr] but also derives Borsh serializer
#[macro_export]
macro_rules! make_borsh {
    ($(#[$meta:meta])* $struct:ident {$($element: ident: $ty: ty, $btr_ty: ty, $doc:expr); *}) => {
        $crate::make_better! {
            $(#[$meta])* "borsh" $struct {$($element: $ty, $btr_ty, $doc); *}
        }
    }
}

#[macro_export]
macro_rules! make_better {
    ($(#[$meta:meta])* $type:literal $struct:ident {$($element: ident: $ty: ty, $btr_ty: ty, $doc:expr); *}) => {
        paste::paste! {
            $(#[$meta])*
            #[btr_macros::[<$type _derive>]([<Btr $struct>])]
            pub struct $struct {
                $(
                    #[doc=$doc]
                    pub $element: $ty
                ),*
            }
            $crate::impl_new! {
                $struct {
                    $($element, $ty); *
                }
            }
            $(#[$meta])*
            #[doc = "[" $struct "] optimized for math and storage (via support for either Borsh or Bincode2 serialization)."]
            #[btr_macros::[<$type _derive>]($struct)]
            pub struct [<Btr $struct>] {
                $(
                    #[doc=$doc]
                    pub $element: $btr_ty
                ),*
            }
            $crate::impl_new! {
                [<Btr $struct>] {
                    $($element, $btr_ty); *
                }
            }
        }
    }
}

/// Creates a set of vector structs
/// that have the item (2nd arg) in them that can be converted between.
///
/// **Usage:** impl_btr_vec!(Liquidations, Liquidation);
///
/// Creates two vectors (Liquidations, BtrLiquidations) each with field `items` of type Vec<Liquidation> and Vec<BtrLiquidation>.
#[macro_export]
macro_rules! impl_btr_vec {
    ($struct:ident, $item:ident) => {
        paste::paste! {
            #[cosmwasm_schema::cw_serde]
            #[derive(Default)]
            pub struct $struct { pub items: Vec<$item> }

            impl $struct {
                pub fn new(items: Vec<$item>) -> Self {
                    $struct { items }
                }
            }

            impl From<[<Btr $struct>]> for $struct {
                fn from(s: [<Btr $struct>]) -> Self {
                    $struct { items: s.items.into_iter().map(|x| x.into()).collect() }
                }
            }

            #[derive(
                serde::Serialize,
                serde::Deserialize,
                Clone,
                Debug,
                PartialEq,
                Default,
            )]
            pub struct [<Btr $struct>] { pub items: Vec<[<Btr $item>]> }

            impl [<Btr $struct>] {
                pub fn new(items: Vec<[<Btr $item>]>) -> Self {
                    [<Btr $struct>] { items }
                }
            }

            impl From<$struct> for [<Btr $struct>] {
                fn from(s: $struct) -> Self {
                    [<Btr $struct>] { items: s.items.into_iter().map(|x| x.into()).collect() }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_new {
    ($struct:ident {$($element: ident, $ty: ty); *}) => {
        impl $struct {
            pub fn new($($element: $ty),*) -> Self {
                $struct {
                    $($element),*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_btr_default {
    ($struct:ty, [$($addr: ident), *], {$($element: ident), *}) => {
        paste::paste! {
        impl Default for $struct {
            fn default() -> Self {
                Self {
                    $($addr: cosmwasm_std::Addr::unchecked(String::default()),)*
                    $($element: Default::default(),)*
                }
            }
        }
        impl Default for [<Btr $struct>] {
            fn default() -> Self {
                Self {
                    $($addr: cosmwasm_std::Addr::unchecked(String::default()),)*
                    $($element: Default::default(),)*
                }
            }
        }
    }
    }
}

#[macro_export]
macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}
#[macro_export]
macro_rules! as_item {
    ($i:item) => {
        $i
    };
}
#[macro_export]
macro_rules! as_pat {
    ($p:pat) => {
        $p
    };
}
#[macro_export]
macro_rules! as_stmt {
    ($s:stmt) => {
        $s
    };
}
#[macro_export]
macro_rules! as_ty {
    ($t:ty) => {
        $t
    };
}

/// Creates a set of vector structs
/// that have the item (2nd arg) in them that can be converted between.
///
/// **Usage:** impl_borsh_vec!(Liquidations, Liquidation);
///
/// Creates two vectors (Liquidations, BtrLiquidations) each with field `items` of type Vec<Liquidation> and Vec<BtrLiquidation>.
#[macro_export]
macro_rules! impl_borsh_vec {
    ($struct:ident, $item:ident) => {
        paste::paste! {
            #[cosmwasm_schema::cw_serde]
            #[derive(Default)]
            pub struct $struct { pub items: Vec<$item> }

            impl $struct {
                pub fn new(items: Vec<$item>) -> Self {
                    $struct { items }
                }
            }

            impl From<[<Btr $struct>]> for $struct {
                fn from(s: [<Btr $struct>]) -> Self {
                    $struct { items: s.items.into_iter().map(|x| x.into()).collect() }
                }
            }

            #[derive(
                serde::Serialize,
                serde::Deserialize,
                borsh::BorshSerialize,
                borsh::BorshDeserialize,
                Clone,
                Debug,
                PartialEq,
                Default,
            )]
            pub struct [<Btr $struct>] { pub items: Vec<[<Btr $item>]> }

            impl [<Btr $struct>] {
                pub fn new(items: Vec<[<Btr $item>]>) -> Self {
                    [<Btr $struct>] { items }
                }
            }

            impl From<$struct> for [<Btr $struct>] {
                fn from(s: $struct) -> Self {
                    [<Btr $struct>] { items: s.items.into_iter().map(|x| x.into()).collect() }
                }
            }
        }
    };
}
