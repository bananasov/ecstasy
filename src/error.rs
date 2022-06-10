#[derive(Debug)]
pub enum EcstasyError {
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
    ReqwestUrlError(reqwest::UrlError),
    SerdeJSONError(serde_json::Error),
    SetLoggerError(log::SetLoggerError),
    SerdeXMLError(serde_xml_rs::Error),
    DecompressError(flate2::DecompressError),
    SimpleString(String),
    FromUTF8Error(std::string::FromUtf8Error),
}

impl From<std::io::Error> for EcstasyError {
    fn from(err: std::io::Error) -> Self {
        EcstasyError::IOError(err)
    }
}

impl From<reqwest::Error> for EcstasyError {
    fn from(err: reqwest::Error) -> Self {
        EcstasyError::ReqwestError(err)
    }
}

impl From<reqwest::UrlError> for EcstasyError {
    fn from(err: reqwest::UrlError) -> Self {
        EcstasyError::ReqwestUrlError(err)
    }
}

impl From<serde_json::Error> for EcstasyError {
    fn from(err: serde_json::Error) -> Self {
        EcstasyError::SerdeJSONError(err)
    }
}

impl From<log::SetLoggerError> for EcstasyError {
    fn from(err: log::SetLoggerError) -> Self {
        EcstasyError::SetLoggerError(err)
    }
}

impl From<serde_xml_rs::Error> for EcstasyError {
    fn from(err: serde_xml_rs::Error) -> Self {
        EcstasyError::SerdeXMLError(err)
    }
}

impl From<flate2::DecompressError> for EcstasyError {
    fn from(err: flate2::DecompressError) -> Self {
        EcstasyError::DecompressError(err)
    }
}

impl From<String> for EcstasyError {
    fn from(err: String) -> Self {
        EcstasyError::SimpleString(err)
    }
}

impl From<std::string::FromUtf8Error> for EcstasyError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        EcstasyError::FromUTF8Error(err)
    }
}
