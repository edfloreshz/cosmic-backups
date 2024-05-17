// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::fl;
use cosmic::{
    app::{Command, Core},
    cosmic_theme,
    iced::{Alignment, Length},
};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced_core::keyboard::Key;
use cosmic::widget::menu::{
    action::{MenuAction},
    key_bind::{KeyBind, Modifier},
};
use cosmic::widget::segmented_button::Entity;
use cosmic::{widget, Application, Element};

pub mod menu;

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
#[derive(Clone, Default)]
pub struct CosmicBackups {
    /// This is the core of your application, it is used to communicate with the Cosmic runtime.
    /// It is used to send messages to your application, and to access the resources of the Cosmic runtime.
    core: Core,
    key_binds: HashMap<KeyBind, Action>,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    Cut(Option<Entity>),
    ToggleContextPage(ContextPage),
    LaunchUrl(String),
    WindowClose,
    WindowNew,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextPage {
    About,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => String::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    About,
    Cut,
    WindowClose,
    WindowNew,
}

impl MenuAction for Action {
    type Message = Message;
    fn message(&self, entity_opt: Option<Entity>) -> Self::Message {
        match self {
            Action::About => Message::ToggleContextPage(ContextPage::About),
            Action::Cut => Message::Cut(entity_opt),
            Action::WindowClose => Message::WindowClose,
            Action::WindowNew => Message::WindowNew,
        }
    }
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the executor that will be used to run your application.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for CosmicBackups {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.github.ahoneybun.CosmicBackups";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the header of your application, it can be used to display the title of your application.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![menu::menu_bar(&self.key_binds)]
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _input: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = CosmicBackups {
            core,
            key_binds: key_binds(),
        };

        (app, Command::none())
    }

    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {
        widget::container(widget::text::title1(fl!("welcome")))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}

    fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = cosmic::theme::active().cosmic().spacing;
        let repository = "https://github.com/ahoneybun/cosmic-backups";
        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");
        widget::column::with_children(vec![
                widget::svg(widget::svg::Handle::from_memory(
                    &include_bytes!(
                        "../res/icons/hicolor/128x128/apps/com.example.CosmicAppTemplate.svg"
                    )[..],
                ))
                .into(),
                widget::text::title3(fl!("cosmic-backups")).into(),
                widget::button::link(repository)
                    .on_press(Message::LaunchUrl(repository.to_string()))
                    .padding(0)
                    .into(),
                widget::button::link(fl!(
                    "git-description",
                    hash = short_hash.as_str(),
                    date = date
                ))
                    .on_press(Message::LaunchUrl(format!("{}/commits/{}", repository, hash)))
                    .padding(0)
                .into(),
            ])
        .align_items(Alignment::Center)
        .spacing(space_xxs)
        .into()
    }

pub fn key_binds() -> HashMap<KeyBind, Action> {
    let mut key_binds = HashMap::new();

    macro_rules! bind {
        ([$($modifier:ident),* $(,)?], $key:expr, $action:ident) => {{
            key_binds.insert(
                KeyBind {
                    modifiers: vec![$(Modifier::$modifier),*],
                    key: $key,
                },
                Action::$action,
            );
        }};
    }

    bind!([Ctrl], Key::Character("w".into()), WindowClose);
    bind!([Ctrl, Shift], Key::Character("n".into()), WindowNew);

    key_binds
}
