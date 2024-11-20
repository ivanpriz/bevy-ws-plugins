use bevy::prelude::*;
use bevy_iced::{
    iced::widget::{column, container, scrollable, text, text_input, Column},
    IcedContext,
};

use super::super::events::UiMessage;
use super::super::resources::UIState;

pub fn render_ui(mut ctx: IcedContext<UiMessage>, ui_state: Res<UIState>) {
    let mut col = Column::new();
    for msg in ui_state.chat_messages.iter() {
        col = col.push(text(msg));
    }
    let msgs_history = scrollable(container(col)).height(250.0).width(300);

    // input for msgs
    let input_form = text_input("Your msg", &ui_state.chat_input_value)
        .on_input(|value| UiMessage::ChatInputUpdated(value))
        .on_submit(UiMessage::ChatInputSubmitted);
    ctx.display(
        container(column![msgs_history, input_form].spacing(10))
            .padding(10)
            .center_x()
            .center_y(),
    );
}
