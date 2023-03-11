pub struct Santa {
    state: SantaState,
    keep_going: bool,
}

enum SantaState {
    Sleeping,
    ReadyForChristmas,
    WokenUpByElves,
    WokenUpByReindeer,
}
