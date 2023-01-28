use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{}", message)]
    GitError { message: String },

    #[error("no other commit found in \"{0}\". try another branch? (eg. jumper --branch main next)")]
    GitLogEmpty(String),

    #[error("no more changes found for the file: {0}")]
    NoMoreChanges(String),

    #[error("already at the first commit")]
    FirstCommit,

    #[error("already at the last commit")]
    LastCommit,
}
