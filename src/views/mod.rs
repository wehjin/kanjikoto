//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.

mod navbar;
pub use navbar::Navbar;

mod start;
pub use start::Start;

mod home;
pub use home::Home;

pub mod practice;

mod lesson;
pub use lesson::Lesson;
