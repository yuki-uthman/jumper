use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{}", message)]
    GitError { message: String },

    #[error(
        "no other commit found in \"{}\". try another branch? (eg. jumper --branch main next)",
        branch
    )]
    GitLogEmpty { branch: String },

    #[error("no more changes found for the file: {}", file)]
    NoMoreChanges { file: String },

    #[error("already at the first commit")]
    FirstCommit,

    #[error("already at the last commit")]
    LastCommit,
}
