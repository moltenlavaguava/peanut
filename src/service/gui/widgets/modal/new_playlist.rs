use std::time::Duration;

use iced::{
    Element, Length, Padding, Task,
    task::Handle,
    widget::{container, row, space},
};
use url::Url;

use crate::service::gui::{
    enums::Message,
    widgets::{
        button::{default_text_button, secondary_text_button},
        modal::{
            AbstractModal,
            AbstractModalMessage::{self, Global, Local},
            Modal, column,
        },
        text::{error_text, title_text},
        text_input::default_text_input,
    },
};

#[derive(Debug, Clone)]
pub enum NewPlaylistModalMsg {
    UrlTextUpdate(String),
    PlaylistURLError(String),
    RemoveError,
    // Make sure the url is valid, and if it is, close the modal
    CheckSubmitURL,
}

#[derive(Debug, Clone)]
pub struct NewPlaylistModal {
    url_text: String,
    url_error: Option<String>,
    error_timer_handle: Option<Handle>,
}
impl AbstractModal<Message> for NewPlaylistModal {
    type ModalMsg = NewPlaylistModalMsg;

    fn view(
        &self,
        theme: &iced::Theme,
    ) -> Element<'_, AbstractModalMessage<Self::ModalMsg, Message>> {
        // build new playlist modal
        let title = title_text("New Playlist", theme, true, true);
        let playlist_url_box = default_text_input("Youtube Playlist URL", &self.url_text, theme)
            .on_input(|s| Local(NewPlaylistModalMsg::UrlTextUpdate(s)))
            .on_paste(|s| Local(NewPlaylistModalMsg::UrlTextUpdate(s)))
            .on_submit(Local(NewPlaylistModalMsg::CheckSubmitURL));
        let mut playlist_data = column![playlist_url_box];
        if let Some(et) = &self.url_error {
            playlist_data =
                playlist_data.push(error_text(format!("Error: {}", et), theme, true, true))
        } else {
            playlist_data = playlist_data.push(error_text("", theme, true, true))
        }

        let playlist_submit = default_text_button("Submit", theme)
            .on_press(Local(NewPlaylistModalMsg::CheckSubmitURL));
        let cancel = secondary_text_button("Cancel", theme).on_press(Global(Message::HideModal));
        let buttons_row = row![space().width(Length::Fill), cancel, playlist_submit].spacing(10);
        container(column![title, playlist_data, buttons_row].spacing(10.0))
            .width(Length::Fixed(400.0))
            .padding(Padding::new(20.0))
            .into()
    }

    fn update(
        &mut self,
        message: Self::ModalMsg,
    ) -> iced::Task<AbstractModalMessage<Self::ModalMsg, Message>> {
        match message {
            NewPlaylistModalMsg::UrlTextUpdate(s) => {
                self.url_text = s;
                Task::none()
            }
            NewPlaylistModalMsg::CheckSubmitURL => {
                // check 1: make sure this is a valid URL
                match Url::parse(&self.url_text) {
                    Ok(u) => {
                        // check 2: make sure url is valid youtube url
                        if !matches!(u.domain(), Some(d) if d == "www.youtube.com" || d == "www.youtu.be")
                        {
                            return Task::done(Local(NewPlaylistModalMsg::PlaylistURLError(
                                String::from("Input (probably) isn't a valid YT URL"),
                            )));
                        }
                        // check 3: make sure we're on 'playlist' and not 'watch'
                        if u.path() != "/playlist" {
                            return Task::done(Local(NewPlaylistModalMsg::PlaylistURLError(
                                String::from("Only playlist URLs are accepted currently :("),
                            )));
                        }
                    }
                    Err(_) => {
                        return Task::done(Local(NewPlaylistModalMsg::PlaylistURLError(
                            String::from("Failed to parse URL"),
                        )));
                    }
                };
                Task::batch(vec![
                    Task::done(Global(Message::PlaylistURLSubmit(self.url_text.clone()))),
                    Task::done(Global(Message::HideModal)),
                ])
            }
            NewPlaylistModalMsg::PlaylistURLError(e) => {
                self.url_error = Some(e);
                // if there was previously a timer remove it
                if let Some(h) = &mut self.error_timer_handle {
                    h.abort();
                }
                // start delay to remove error
                let (t, h) = Task::perform(
                    async {
                        tokio::time::sleep(Duration::from_secs(3)).await;
                        NewPlaylistModalMsg::RemoveError
                    },
                    AbstractModalMessage::Local,
                )
                .abortable();
                self.error_timer_handle = Some(h);
                t
            }
            NewPlaylistModalMsg::RemoveError => {
                self.url_error = None;
                self.error_timer_handle = None;
                Task::none()
            }
        }
    }
}
impl Into<Modal> for NewPlaylistModal {
    fn into(self) -> Modal {
        Modal::NewPlaylist(self)
    }
}
impl NewPlaylistModal {
    pub fn new() -> Self {
        Self {
            url_text: String::new(),
            url_error: None,
            error_timer_handle: None,
        }
    }
}
