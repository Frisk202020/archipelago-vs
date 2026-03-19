pub(crate) struct RequestData {}
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, RequestData, Error>;