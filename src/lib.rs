use axum::{extract::FromRequestParts, http::StatusCode};
use std::{boxed::Box, future::Future, pin::Pin};
use futures::FutureExt;
use std::convert::Infallible;
pub struct Token(pub String);

pub struct Theme(pub theme::Theme);

impl<S> FromRequestParts<S> for Token 
    where 
        S: Send
{
    type Rejection = error::Error;
    fn from_request_parts<'a,'b,'c>(parts: &'a mut axum::http::request::Parts, _state: &'b S) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'c>> 
    where 
        'a: 'c,
        'b: 'c,
    {
        async {            
            let cookies = parts.headers.get(axum::http::header::COOKIE);
            match cookies.map(|e| e.to_str()) {
                Some(Ok(e)) => {
                    let tmp: Vec<_> = e.split(';').collect();
                    for i in tmp {
                        let cookie: Vec<_> = i.split("=").collect();
                        if let (Some(Ok(cookie::Cookie::TOKEN)), Some(value)) = (cookie.get(0).map(|x| cookie::Cookie::try_from(*x)), cookie.get(1)) {
                            return Ok(Self(value.to_string()));
                        }
                    }
                    Err(error::Error::KeyNotFound)
                },
                _ => Err(error::Error::KeyNotFound)
            }
        }.boxed()
    }
}

impl<S> FromRequestParts<S> for Theme 
    where 
        S: Send,
{
    type Rejection = Infallible;

    fn from_request_parts<'a,'b,'c>(parts: &'a mut axum::http::request::Parts, _state: &'b S) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'c>>
    where 
        'a: 'c,
        'b: 'c,
    {
        async {
            if let Some(e) = parts.headers.get(axum::http::header::COOKIE) {
                if let Ok(key_value) = e.to_str().map(|x| x.split(';').collect::<Vec<_>>()) {
                    for i in key_value {
                        let tmp: Vec<_> = i.split('=').collect();
                        if let (Some(Ok(self::cookie::Cookie::THEME)), Some(value)) = (tmp.get(0).map(|x| self::cookie::Cookie::try_from(*x)), tmp.get(1)) {
                            return Ok(Self( match self::theme::Theme::try_from(*value) {
                                Ok(e) => e,
                                _ => theme::Theme::Light,
                            }));
                        }
                    }
                }
            }
            Ok(Theme(theme::Theme::Light))
        }.boxed()
    }
}

pub mod cookie {
    #[derive(Debug, PartialEq)]
    pub enum Cookie {
        TOKEN,
        THEME,
    }

    impl std::fmt::Display for Cookie {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::TOKEN => write!(f, "token"),
                Self::THEME => write!(f, "theme"),
            }
        }
    }

    impl TryFrom<&str> for Cookie {
        type Error = super::error::ParseError;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "token" => Ok(Self::TOKEN),
                "theme" => Ok(Self::THEME),
                _ => Err(super::error::ParseError),
            }
        }
    }


}

pub mod theme {


    #[derive(Debug, PartialEq)]
    pub enum Theme {
        Dark,
        Light,
    }

    impl std::fmt::Display for Theme {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Theme::*;

            match self {
                Dark => write!(f, "dark"),
                Light => write!(f, "light"),
            }
        }
    }

    
    impl TryFrom<&str> for Theme {
        type Error = super::error::ParseError;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "dark" => Ok(Self::Dark),
                "light" => Ok(Self::Light),
                _ => Err(super::error::ParseError),
            }
        }
    }

}

pub mod error {
    use axum::response::IntoResponse;
    use super::StatusCode;

    #[derive(Debug)]
    pub enum Error {
        KeyNotFound,
    }

    impl IntoResponse for Error {
        fn into_response(self) -> axum::response::Response {
            match self {
                Self::KeyNotFound => (StatusCode::BAD_REQUEST, "Key not found").into_response()
            }
        }
    }

    #[derive(Debug)]
    pub struct ParseError;
}