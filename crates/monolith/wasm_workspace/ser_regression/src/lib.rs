use derive_more::*;
use crate::hdk3::prelude::*;

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
struct CreateMessageInput {
    channel_hash: EntryHash,
    content: String,
}
#[derive(Debug, From, Into, Serialize, Deserialize, SerializedBytes)]
pub struct ChannelName(String);

#[hdk_entry(id = "channel")]
#[derive(Constructor)]
pub struct Channel {
    name: String,
}

#[hdk_entry(id = "channel_message")]
#[derive(Constructor)]
pub struct ChannelMessage {
    message: String,
}

entry_defs![
    Path::entry_def(),
    Channel::entry_def(),
    ChannelMessage::entry_def()
];

fn channels_path() -> Path {
    let path = Path::from("channels");
    path.ensure().expect("Couldn't ensure path");
    path
}

#[hdk_extern]
fn create_channel(name: ChannelName) -> ExternResult<EntryHash> {
    debug!(format!("channel name {:?}", name));
    let path = channels_path();
    let channel = Channel::new(name.into());
    let channel_hash = hash_entry(&channel)?;
    let sb: SerializedBytes = channel_hash.clone().try_into().unwrap();
    create_entry(&channel)?;
    debug!(format!("sb in channel {:?}", sb));
    create_link(hash_entry(&path)?, channel_hash.clone(), ())?;
    Ok(channel_hash)
}

#[hdk_extern]
fn create_message(input: CreateMessageInput) -> ExternResult<EntryHash> {
    debug!(format!("{:?}", input));
    let CreateMessageInput {
        channel_hash,
        content,
    } = input;
    let message = ChannelMessage::new(content);
    let message_hash = hash_entry(&message)?;
    create_entry(&message)?;
    create_link(channel_hash, message_hash.clone(), ())?;
    Ok(message_hash)
}