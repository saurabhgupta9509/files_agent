use notify::{EventKind, event::ModifyKind};

use crate::rules::operation::Operation;

pub enum FsAction {
    Create,
    Modify,
    Delete,
    Rename,
}
pub fn map_event(event: &notify::Event) -> Option<FsAction> {
    match event.kind {
        EventKind::Create(_) => Some(FsAction::Create),
        EventKind::Modify(_) => Some(FsAction::Modify),
        EventKind::Remove(_) => Some(FsAction::Delete),
        EventKind::Modify(ModifyKind::Name(_)) => Some(FsAction::Rename),
        _ => None,
    }
}

impl From<FsAction> for Operation {
    fn from(action: FsAction) -> Self {
        match action {
            FsAction::Create => Operation::Create,
            FsAction::Modify => Operation::Modify,
            FsAction::Delete => Operation::Delete,
            FsAction::Rename => Operation::Rename,
        }
    }
}