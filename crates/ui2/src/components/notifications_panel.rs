use std::marker::PhantomData;

use crate::{prelude::*, static_notification_items, Icon, ListHeaderTool};
use crate::{theme, Input, List, ListHeader};

#[derive(Element)]
pub struct NotificationsPanel<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
}

impl<S: 'static + Send + Sync + Clone> NotificationsPanel<S> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            state_type: PhantomData,
            scroll_state,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);
        let color = ThemeColor::new(cx);

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .fill(color.surface)
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll(ScrollState::default())
                    .child(
                        List::new(static_notification_items())
                            .header(ListHeader::new("NOTIFICATIONS").set_tools(Some(vec![
                                ListHeaderTool::new(
                                    Icon::BellSlash,
                                    "Mute Notifications".into(),
                                    "mute notifications".into(),
                                ),
                                ListHeaderTool::new(
                                    Icon::MailOpen,
                                    "Mark all as read".into(),
                                    "mark all as read action".into(),
                                ),
                            ])))
                            .set_toggleable(Toggleable::NotToggleable)
                            .empty_message("No unread notifications."),
                    ),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::{Panel, Story};

    use super::*;

    #[derive(Element)]
    pub struct NotificationsPanelStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> NotificationsPanelStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, NotificationsPanel<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Panel::new(ScrollState::default())
                        .child(NotificationsPanel::new(ScrollState::default())),
                )
        }
    }
}
