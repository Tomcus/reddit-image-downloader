use std::{error::Error, fmt};

#[derive(Debug)]
enum RedditErrorSource {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Image(image::ImageError),
    IO(std::io::Error)
}

#[derive(Debug)]
pub struct RedditError {
    source: Option<RedditErrorSource>,
    msg: Option<String>,
}

pub type RedditResult<T> = std::result::Result<T, RedditError>;

impl fmt::Display for RedditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref msg) = self.msg {
            write!(f, "{}", msg)
        } else if let Some(ref source) = self.source {
            match source {
                RedditErrorSource::Reqwest(err) => {
                    write!(f, "{}", err.to_string())
                }
                RedditErrorSource::Serde(err) => {
                    write!(f, "{}", err.to_string())
                }
                RedditErrorSource::Image(err) => {
                    write!(f, "{}", err.to_string())
                }
                RedditErrorSource::IO(err) => {
                    write!(f, "{}", err.to_string())
                }
            }
        } else {
            write!(f, "FATAL: No message for given error")
        }
    }
}

impl From<reqwest::Error> for RedditError {
    fn from(err: reqwest::Error) -> RedditError {
        RedditError {
            source: Some(RedditErrorSource::Reqwest(err)),
            msg: None,
        }
    }
}

impl From<serde_json::Error> for RedditError {
    fn from(err: serde_json::Error) -> RedditError {
        RedditError {
            source: Some(RedditErrorSource::Serde(err)),
            msg: None,
        }
    }
}

impl From<image::ImageError> for RedditError {
    fn from(err: image::ImageError) -> RedditError {
        RedditError {
            source: Some(RedditErrorSource::Image(err)),
            msg: None,
        }
    }
}

impl From<std::io::Error> for RedditError {
    fn from(err: std::io::Error) -> RedditError {
        RedditError { source: Some(RedditErrorSource::IO(err)), msg: None }
    }
}

impl From<&str> for RedditError {
    fn from(message: &str) -> RedditError {
        RedditError {
            source: None,
            msg: Some(String::from(message)),
        }
    }
}

impl Error for RedditError {
    fn cause(&self) -> Option<&dyn Error> {
        match self.source {
            Some(RedditErrorSource::Reqwest(ref err)) => Some(err),
            Some(RedditErrorSource::Serde(ref err)) => Some(err),
            Some(RedditErrorSource::Image(ref err)) => Some(err),
            Some(RedditErrorSource::IO(ref err)) => Some(err),
            None => None,
        }
    }
}
