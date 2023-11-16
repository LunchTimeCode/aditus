#[derive(Debug)]
pub struct PureInput {
    pub(crate) username: String,
    pub(crate) pw: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) domain: String,
    pub(crate) aud: String,
}
