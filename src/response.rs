use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(flatten)]
    pub data_or_error: DataOrError<T>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataOrError<T> {
    Data { data: T },
    Error { error: String },
}

pub type SuccessResponse<T> = ApiResponse<T>;

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data_or_error: DataOrError::Data { data },
        }
    }

    pub fn error(error_msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data_or_error: DataOrError::Error {
                error: error_msg.into(),
            },
        }
    }
}
