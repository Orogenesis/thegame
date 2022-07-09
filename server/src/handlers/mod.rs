pub mod connect;
pub mod create_room;
pub mod discard;
pub mod disconnect;
pub mod end_turn;
pub mod join;
pub mod start;

pub mod prelude {
    //! ```
    //! # #![allow(unused_imports)]
    //! use crate::handlers::prelude::*;
    //! ```
    pub use crate::handlers::connect::*;
    pub use crate::handlers::create_room::*;
    pub use crate::handlers::discard::*;
    pub use crate::handlers::disconnect::*;
    pub use crate::handlers::join::*;
    pub use crate::handlers::start::*;
    pub use crate::handlers::end_turn::*;
}
