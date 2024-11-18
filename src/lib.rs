//! `serde` De/Serialization for `io::ErrorKind`.
#![feature(io_error_more)]

use serde::{Deserialize, Serialize};
use std::io;

/// Copy of [`io::ErrorKind`] for `serde` de/serialization.
#[non_exhaustive]
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(remote = "io::ErrorKind")]
pub enum IoErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    HostUnreachable,
    NetworkUnreachable,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    NetworkDown,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    NotADirectory,
    IsADirectory,
    DirectoryNotEmpty,
    ReadOnlyFilesystem,
    FilesystemLoop,
    StaleNetworkFileHandle,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    StorageFull,
    NotSeekable,
    FilesystemQuotaExceeded,
    FileTooLarge,
    ResourceBusy,
    ExecutableFileBusy,
    Deadlock,
    CrossesDevices,
    TooManyLinks,
    InvalidFilename,
    ArgumentListTooLong,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
}

impl serde_with::SerializeAs<io::ErrorKind> for IoErrorKind {
    fn serialize_as<S>(value: &io::ErrorKind, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        IoErrorKind::serialize(value, serializer)
    }
}

impl<'de> serde_with::DeserializeAs<'de, io::ErrorKind> for IoErrorKind {
    fn deserialize_as<D>(deserializer: D) -> Result<io::ErrorKind, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        IoErrorKind::deserialize(deserializer)
    }
}
