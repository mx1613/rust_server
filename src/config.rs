pub struct ServerConfig {
    pub base_url: &'static str,
}

pub const SERVER_CONFIG: ServerConfig = ServerConfig {
    base_url: "127.0.0.1:8000",
};
