use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct WebIcon {
    pub src: String,
    pub sizes: String,
    #[serde(rename = "type")]
    pub typ: String
}

#[derive(Serialize, Debug)]
pub struct WebManifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    pub start_url: String,
    pub display: String,
    pub theme_color: String,
    pub name: String,
    pub icons: Vec<WebIcon>
}