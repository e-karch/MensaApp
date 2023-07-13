use std::fmt::Display;

use crate::util::ReportReason;

#[derive(Copy, Clone)]
pub enum ImageCommandType {
    ReportImage(ReportReason),
    AddUpvote,
    AddDownvote,
    RemoveUpvote,
    RemoveDownvote,
}

impl Display for ImageCommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ReportImage(_) => "reportImage",
            Self::AddUpvote => "addUpvote",
            Self::AddDownvote => "addDownvote",
            Self::RemoveUpvote => "removeUpvote",
            Self::RemoveDownvote => "removeDownvote",
        };
        write!(f, "{msg}")
    }
}
