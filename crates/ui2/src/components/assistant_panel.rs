use std::marker::PhantomData;

use gpui3::{rems, AbsoluteLength};

use crate::{h_stack, prelude::*, Button, LabelColor};
use crate::{Icon, IconButton, Label, Panel, PanelSide};

#[derive(Element)]
pub struct AssistantPanel<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
    current_side: PanelSide,
    token_count: usize,
}

impl<S: 'static + Send + Sync + Clone> AssistantPanel<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
            scroll_state: ScrollState::default(),
            current_side: PanelSide::default(),
            token_count: 10,
        }
    }

    pub fn side(mut self, side: PanelSide) -> Self {
        self.current_side = side;
        self
    }

    pub fn update_token_count(&mut self, token_count: usize) {
        self.token_count = token_count;
    }

    pub fn reset_token_count(&mut self) {
        self.token_count = 8185;
    }

    fn render(&mut self, view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);

        let token_count = self.token_count;
        let count_color = if token_count > 4000 {
            LabelColor::Positive
        } else if token_count <= 1000 {
            LabelColor::Negative
        } else {
            LabelColor::Warning
        };

        Panel::new(self.scroll_state.clone())
            .children(vec![div()
                .flex()
                .flex_col()
                .h_full()
                .px_2()
                .gap_2()
                // Header
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .child(IconButton::new(Icon::Menu))
                                .child(Label::new("New Conversation")),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_px()
                                .child(IconButton::new(Icon::SplitMessage))
                                .child(IconButton::new(Icon::Quote))
                                .child(IconButton::new(Icon::MagicWand))
                                .child(IconButton::new(Icon::Plus))
                                .child(IconButton::new(Icon::Maximize)),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .relative()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .overflow_y_scroll(self.scroll_state.clone())
                        .child(
                            h_stack()
                                .absolute()
                                .right_2()
                                .top_2()
                                .gap_1()
                                .child(
                                    Button::new("gpt-4")
                                        .variant(ButtonVariant::Ghost)
                                        .icon(Icon::Hash),
                                )
                                .child(Label::new(self.token_count.to_string()).color(count_color)),
                        )
                        .child(h_stack().justify_between().child("left").child("right")),
                )
                .into_any()])
            .side(self.current_side)
            .width(AbsoluteLength::Rems(rems(32.)))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct AssistantPanelStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> AssistantPanelStory<S> {
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
                .child(Story::title_for::<_, AssistantPanel<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(AssistantPanel::new())
        }
    }
}
