//! If any available, this provides handles for various forms of asynchronous
//! drivers that can be used in combination with audio interfaces.

mod atomic_waker;

cfg_events_driver! {
    pub(crate) mod events;
    pub use self::events::Events;
}

cfg_poll_driver! {
    pub(crate) mod poll;
    pub use self::poll::{Poll, PollHandle, PollEventsGuard};
}
