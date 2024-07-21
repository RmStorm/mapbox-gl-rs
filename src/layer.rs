use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint: Option<Paint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_mode: Option<String>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    pub r#type: String,
}

impl Layer {
    pub fn new(
        id: impl Into<String>,
        r#type: impl Into<String>,
        source: impl Into<String>,
    ) -> Layer {
        Layer {
            id: id.into(),
            layout: None,
            maxzoom: None,
            minzoom: None,
            paint: None,
            rendering_mode: None,
            source: source.into(),
            source_layer: None,
            r#type: r#type.into(),
        }
    }
}

/// Layout property can be either value in String or Number e.g. 0.25
/// or tuple of 2 elements, ("get", "icon").
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayoutProperty {
    String(String),
    Number(f64),
    FeatureProperty((String, String)),
}

impl From<&str> for LayoutProperty {
    fn from(value: &str) -> Self {
        LayoutProperty::String(value.into())
    }
}

impl From<String> for LayoutProperty {
    fn from(value: String) -> Self {
        LayoutProperty::String(value)
    }
}

impl From<f64> for LayoutProperty {
    fn from(value: f64) -> Self {
        LayoutProperty::Number(value)
    }
}

impl From<(String, String)> for LayoutProperty {
    fn from((verb, name): (String, String)) -> Self {
        LayoutProperty::FeatureProperty((verb, name))
    }
}

impl From<(&str, &str)> for LayoutProperty {
    fn from((verb, name): (&str, &str)) -> Self {
        LayoutProperty::FeatureProperty((verb.into(), name.into()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Layout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_join: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_cap: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_image: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<LayoutProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Paint {
    pub line_color: String,
    pub line_width: u32,
}
