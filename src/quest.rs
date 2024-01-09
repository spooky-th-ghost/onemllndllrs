use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct QuestLog {
    quests: HashMap<QuestId, Quest>,
}

impl QuestLog {
    pub fn quests(&self) -> impl Iterator<Item = &Quest> {
        self.quests.values()
    }
}

#[derive(Debug)]
pub struct Quest {
    pub description: String,
    pub status: QuestStatus,
    pub progress: f32,
}

pub struct ObjectiveId(pub u16);
pub struct QuestId(pub u16);

#[derive(Debug)]
pub enum QuestStatus {
    Undiscovered,
    Discovered,
    Completed,
}

pub struct QuestStatusEvent {
    pub id: QuestId,
    pub status: QuestStatus,
}

pub struct QuestProgressEvent {
    pub id: QuestId,
    pub progress: f32,
}
