use bevy::prelude::*;
use bevy_iced::{
    iced::widget::{button, column, container, text_input},
    IcedContext,
};

use super::super::events::UiMessage;
use super::super::resources::UIState;

pub fn render_ui(mut ctx: IcedContext<UiMessage>, ui_state: Res<UIState>) {
    // input for msgs
    let input_form_login =
        text_input("Login", &ui_state.login).on_input(|value| UiMessage::LoginInputUpdated(value));
    let input_form_password = text_input("Password", &ui_state.password)
        .on_input(|value| UiMessage::PasswordInputUpdated(value));
    let submit_button = button("Enter").on_press(UiMessage::CredentialsInputSubmitted);
    ctx.display(
        container(column![input_form_login, input_form_password, submit_button].spacing(10))
            .padding(10)
            .center_x()
            .center_y(),
    );
}
