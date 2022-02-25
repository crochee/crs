use http::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub trait ErrorCode<'de, T>: Deserialize<'de> + Serialize + Error {
    fn status_code(&self) -> StatusCode;
    fn code(&self) -> &str;
    fn message(&self) -> &str;
    fn result(&self) -> Option<T>;
    fn with_status_code(&mut self, status_code: usize) -> Self;
    fn with_code(&mut self, code: &str) -> Self;
    fn with_message(&mut self, message: &str) -> Self;
    fn with_result(&mut self, value: T) -> Self;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrCode<T> {
    pub code: String,
    pub message: String,
    pub result: Option<T>,
}

impl<T> Display for ErrCode<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(result) = &self.result {
            return write!(f, "({}, {}, {})", self.code, self.message, result);
        }
        write!(f, "({}, {})", self.code, self.message)
    }
}

impl<T> Error for ErrCode<T> where T: Display + Debug {}

impl<'de, T> ErrorCode<'de, T> for ErrCode<T>
where
    T: Display + Debug + Serialize + Deserialize<'de> + Copy,
{
    fn status_code(&self) -> StatusCode {
        match self.parse_status_code() {
            Ok(status_code) => status_code,
            Err(err) => {
                println!("{}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn code(&self) -> &str {
        self.code.as_str()
    }
    fn message(&self) -> &str {
        self.message.as_str()
    }
    fn result(&self) -> Option<T> {
        self.result
    }

    fn with_status_code(&mut self, status_code: usize) -> Self {
        let mut code = self.code.clone();
        let (left, _) = code.split_at_mut(3);
        let s = status_code.to_string();
        unsafe {
            let bytes = left.as_bytes_mut();
            bytes.copy_from_slice(s.as_bytes());
        }
        Self {
            code: code,
            message: self.message.clone(),
            result: self.result.clone(),
        }
    }

    fn with_code(&mut self, code: &str) -> Self {
        let mut temp_code = self.code.clone();
        let (_, right) = temp_code.split_at_mut(3);
        unsafe {
            let bytes = right.as_bytes_mut();
            bytes.copy_from_slice(code.as_bytes());
        }
        Self {
            code: temp_code,
            message: self.message.clone(),
            result: self.result.clone(),
        }
    }

    fn with_message(&mut self, message: &str) -> Self {
        Self {
            code: self.code.clone(),
            message: message.to_string(),
            result: self.result.clone(),
        }
    }
    fn with_result(&mut self, value: T) -> Self {
        Self {
            code: self.code.clone(),
            message: self.message.clone(),
            result: Some(value),
        }
    }
}

impl<T> ErrCode<T> {
    fn parse_status_code(&self) -> anyhow::Result<StatusCode> {
        let (left, _) = self.code.split_at(3);
        let status_code_int = left.parse::<usize>().map_err(anyhow::Error::new)?;
        if status_code_int < 100 {
            return Err(anyhow::anyhow!("{} < 100", status_code_int));
        }
        if status_code_int >= 600 {
            return Err(anyhow::anyhow!("{} >= 600", status_code_int));
        }
        StatusCode::from_u16(status_code_int as u16).map_err(anyhow::Error::new)
    }
}

impl<T, B> From<Response<B>> for ErrCode<T> {
    fn from(req: Response<B>) -> Self {
        todo!()
    }
}
