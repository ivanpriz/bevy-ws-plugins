use bevy::prelude::*;
use bevy_iced::{
    iced::{
        widget::{button, column, container, text, text_input},
        Background, Border, Color as IcedColor, Element, Style, Theme,
    },
    IcedContext,
};

use crate::{
    backend_communicator::account_login::AccountToken,
    state_specific::character_selection::state_resources::Characters,
};

use super::super::events::UiMessage;

pub fn render_ui(
    mut ctx: IcedContext<UiMessage>,
    characters: Res<Characters>,
    account_token: Res<AccountToken>,
) {
    let characters_display = characters.characters_data.iter().map(|(char_id, char)| {
        let mut b = container(
            button(
                container(column![text(char_id), text(char.name.clone())].spacing(10))
                    .padding(10)
                    .center_x()
                    .center_y(),
            )
            .padding(20)
            .on_press(UiMessage::CharacterSelected(char_id.clone())),
        );
        if let Some(selected_char_id) = &characters.selected_character_id {
            if selected_char_id == char_id {
                b = b.style(container::Appearance {
                    background: Some(Background::Color(IcedColor::from_rgb(0.0, 0.9, 0.1))),
                    text_color: Some(IcedColor::WHITE),
                    border: Border::default(),
                    shadow: default(),
                });
            } else {
                // println!(
                //     "Selected id not equal: {:?}, {:?}",
                //     selected_char_id, char_id
                // )
            }
        }
        b.into()
    });
    let token = text(account_token.account_token.clone().unwrap());

    // println!(
    //     "Rendering character selection ui, selected char: {:?}",
    //     characters.selected_character_id
    // );
    ctx.display(
        container(column!(token, column(characters_display)))
            .padding(10)
            .center_x()
            .center_y(),
    );
}
