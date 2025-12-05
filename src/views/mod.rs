//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.

mod navbar;
pub use navbar::Navbar;

mod start;
pub use start::Start;

mod review;
pub use review::Review;

mod answers;
pub use answers::Answers;
