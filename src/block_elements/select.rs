use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Contents {
    User(UserSelect),
    Static(StaticSelect),
    External(ExternalSelect),
    Conversation(ConversationSelect),
    Channel(ChannelSelect),
}

#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct UserSelect {}

#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct StaticSelect {}

#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct ExternalSelect {}

#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct ConversationSelect {}

#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct ChannelSelect {}

