use thiserror::Error;

#[derive(Error, Debug)]
pub enum TreeError {
    #[error("This tree is not Binary.")]
    IsNotBinary,
    #[error("This tree is not rooted.")]
    IsNotRooted,
    #[error("This tree is empty.")]
    IsEmpty,
    #[error("All your leaf nodes must ne named.")]
    UnnamedLeaves,
    #[error("Your leaf names must be unique.")]
    DuplicateLeafNames,
    #[error("The leaf index of the tree is not initialized.")]
    LeafIndexNotInitialized,
    #[error("The tree must have all branch lengths")]
    MissingBranchLengths
}