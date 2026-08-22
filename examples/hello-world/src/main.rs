// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Minimal "你好，世界" (Hello, World) example using libcosmic's application API.

use cosmic::app::Settings;
use cosmic::iced::{Alignment, Length, Size};
use cosmic::prelude::*;
use cosmic::widget;
use cosmic::{executor, Core};

/// Runs application with these settings
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let settings = Settings::default().size(Size::new(640., 480.));
    cosmic::app::run::<App>(settings, ()).unwrap();
    Ok(())
}

/// Messages that are used specifically by our [`App`].
#[derive(Clone, Debug)]
pub enum Message {
    Hi,
}

/// The [`App`] stores application-specific state.
pub struct App {
    core: Core,
    hidden: bool,
}

/// Implement [`cosmic::Application`] to integrate with COSMIC.
impl cosmic::Application for App {
    /// Default async executor to use with the app.
    type Executor = executor::Default;

    /// Argument received [`cosmic::Application::new`].
    type Flags = ();

    /// Message type specific to our [`App`].
    type Message = Message;

    /// The unique application ID to supply to the window manager.
    const APP_ID: &'static str = "org.cosmic.HelloWorld";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Creates the application, and optionally emits task on initialize.
    fn init(core: Core, _flags: Self::Flags) -> (Self, cosmic::app::Task<Self::Message>) {
        let mut app = App { core, hidden: true };

        let command = app.update_title();

        (app, command)
    }

    /// Handle application events here.
    fn update(&mut self, message: Self::Message) -> cosmic::app::Task<Self::Message> {
        match message {
            Message::Hi => {
                self.hidden = !self.hidden;
            }
        }
        Task::none()
    }

    /// Creates a view after each update.
    fn view(&self) -> Element<'_, Self::Message> {
        let greeting = widget::text::title1(if self.hidden {
            "你好，世界！"
        } else {
            "Hello, World!"
        });

        let centered = widget::column::with_capacity(2)
            .push(greeting)
            .push(
                widget::button::suggested(if self.hidden { "Say Hi" } else { "Hide" })
                    .on_press(Message::Hi),
            )
            .spacing(cosmic::theme::spacing().space_s)
            .align_x(Alignment::Center);

        Element::from(
            widget::container(centered)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
    }
}

impl App
where
    Self: cosmic::Application,
{
    fn update_title(&mut self) -> cosmic::app::Task<Message> {
        let header_title = "你好，世界".to_owned();
        let window_title = format!("{header_title} — COSMIC");
        self.set_header_title(header_title);
        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
}
