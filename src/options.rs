#[derive(Debug, Clone, Default, Builder, Getters)]
pub struct Options {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    api_key: String,
    #[builder(setter(into))]
    #[getset(get = "pub")]
    secret_key: String,
    #[builder(setter(into))]
    #[getset(get = "pub")]
    base_url: String,
}
