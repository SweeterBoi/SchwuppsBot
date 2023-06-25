use frankenstein::GetUpdatesParams;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};

static TOKEN: &str = "";
//static AwarenessGroup : frankenstein::objects::Chat = -919978662;

fn main() {
    let api = Api::new(TOKEN);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    loop {
        let result = api.get_updates(&update_params);

        //println!("result: {result:?}");

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        if message.chat.username.ne(&None) && message.text.ne(&None) {
                            if !message.text.clone().unwrap().starts_with("/") {

                                let userName: String = message.chat.username.unwrap();
                                let botText : String = "@".to_owned() + &userName + " schreibt: \n" + &message.text.unwrap();

                                let send_message_params = SendMessageParams::builder()
                                    .chat_id(-919978662)
                                    .text(botText)
                                    .build();

                                if let Err(err) = api.send_message(&send_message_params) {
                                    println!("Failed to send message: {err:?}");
                                }
                            }
                        }
                    }
                    update_params = update_params_builder
                        .clone()
                        .offset(update.update_id + 1)
                        .build();
                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }
}
