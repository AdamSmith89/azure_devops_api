custom_error! { pub ApiError
    Request{source: reqwest::Error} = "Request Error",
    JsonParse{source: serde_json::Error} = "Json Parsing Error",
    UrlParse{source: url::ParseError} = "Url Parsing Error",
}
