use colored::Colorize;
use matrix_sdk::{
    config::SyncSettings,
    ruma::{OwnedRoomId, RoomAliasId, RoomId, RoomOrAliasId},
    Client,
};

/// Parse a Room ID/Room Alias and return the Room ID.
pub(crate) async fn parse_room<S: AsRef<str>>(client: &Client, id: S) -> Result<OwnedRoomId, i32> {
    let id_ref = id.as_ref();

    let room_alias_id = match RoomOrAliasId::parse(id_ref) {
        Ok(room_id) => room_id,
        Err(err) => {
            hw_msg::errorln!("Failed to parse room id '{}'. [{}]", id_ref.green(), err);
            return Err(exitcode::USAGE);
        }
    };

    if room_alias_id.is_room_id() {
        return Ok(RoomId::parse(room_alias_id.as_str()).unwrap());
    } else {
        let alias_id = RoomAliasId::parse(id_ref).unwrap();
        match client.resolve_room_alias(&alias_id).await {
            Ok(resp) => Ok(RoomId::parse(resp.room_id).unwrap()),
            Err(err) => {
                hw_msg::errorln!(
                    "Failed to get room ID for room alias '{}'. [{}]",
                    id_ref.green(),
                    err
                );
                Err(exitcode::UNAVAILABLE)
            }
        }
    }
}

/// Sync the client.
pub(crate) async fn sync_client(client: &Client) -> Result<(), i32> {
    let mut sync_settings = SyncSettings::new();

    if let Some(token) = client.sync_token().await {
        sync_settings = sync_settings.token(token);
    }

    if let Err(err) = client.sync_once(sync_settings).await {
        hw_msg::errorln!("Failed to sync client. [{}]", err);
        Err(exitcode::UNAVAILABLE)
    } else {
        Ok(())
    }
}
