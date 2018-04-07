use std::collections::BTreeSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub applications: Vec<ApplicationConfig>,
    #[serde(default)]
    pub services: Vec<ServiceConfig>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationConfig {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub memory: AppMemoryConfig,
    #[serde(default)]
    pub permissions: BTreeSet<AppPermission>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppMemoryConfig {
    pub min: usize,
    pub max: usize
}

impl Default for AppMemoryConfig {
    fn default() -> AppMemoryConfig {
        AppMemoryConfig {
            min: 64 * 65536,
            max: 256 * 65536
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AppPermission {
    Timer,
    TcpListen(String /* address */)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceConfig {
    pub kind: ServiceKind
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServiceKind {
    Tcp,
    Http
}
