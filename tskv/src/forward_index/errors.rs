use snafu::Snafu;

use super::record_file;

#[derive(Snafu, Debug)]
pub enum ForwardIndexError {
    #[snafu(display("Error with write record file: {}", source))]
    WriteFile {
        source: record_file::RecordFileError,
    },

    #[snafu(display("Error with read record file: {}", source))]
    ReadFile {
        source: record_file::RecordFileError,
    },

    #[snafu(display("Error with write close file: {}", source))]
    CloseFile {
        source: record_file::RecordFileError,
    },

    Action,

    Version,

    FieldType,
}

pub type ForwardIndexResult<T> = std::result::Result<T, ForwardIndexError>;