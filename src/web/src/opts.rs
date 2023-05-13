


#[derive(Clone, Debug, PartialEq, clap::Parser)]
pub struct Opts {
    /// client_id
    #[clap(env)]
    pub client_id: String,

    /// issuer_url
    #[clap(env)]
    pub issuer_url: String,

    /// backend_url
    #[clap(env)]
    pub backend_url: String,
}