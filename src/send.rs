use crate::util;
use colored::Colorize;
use matrix_sdk::{room::Room, ruma::events::room::message::RoomMessageEventContent, Client};

pub(crate) async fn send(
    client: &Client,
    msg: String,
    room: String,
    markdown: bool,
    no_join: bool,
) -> i32 {
    let room_id = match util::parse_room(client, &room).await {
        Ok(id) => id,
        Err(exit_code) => return exit_code,
    };

    let room_struct = match client.get_room(&room_id) {
        Some(unwrapped_room) => unwrapped_room,
        None => {
            hw_msg::errorln!("Unable to find room '{}'.", room.green());
            return exitcode::USAGE;
        }
    };

    // See if we need to join the room.
    match room_struct {
        Room::Joined(_) => (),
        Room::Left(_) | Room::Invited(_) => {
            if no_join {
                hw_msg::errorln!(
                    "User '{}' isn't in room '{}'.",
                    client.user_id().unwrap().as_str().green(),
                    room_struct.room_id().as_str().green()
                );
                return exitcode::USAGE;
            } else {
                if let Err(err) = client.join_room_by_id(&room_id).await {
                    hw_msg::errorln!(
                        "Failed to join room '{}'. [{}]",
                        room_struct.room_id().as_str().green(),
                        err
                    );
                }
                if let Err(exit_code) = util::sync_client(client).await {
                    return exit_code;
                }
            }
        }
    }

    let joined_room = client.get_joined_room(&room_id).unwrap();
    let message: RoomMessageEventContent = if markdown {
        RoomMessageEventContent::text_markdown(msg)
    } else {
        RoomMessageEventContent::text_plain(msg)
    };

    if let Err(err) = joined_room.send(message, None).await {
        hw_msg::errorln!("Failed to send message. [{}]", err);
        return exitcode::UNAVAILABLE;
    }

    exitcode::OK
}
