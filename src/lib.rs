//!

/// clone Arc values in closure
#[macro_export]
macro_rules! clone_input {
    ( ( $($x:ident),* )     $_:expr ) => {
        {
            $( let $x = $x.clone(); )*
            $_
        }
    };
}

pub mod catmd;
pub mod dir;
pub mod img;
pub mod utils {
    pub mod types;
}
