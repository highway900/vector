use http::{uri::InvalidUri, Uri};
use rusoto_core::{region::ParseRegionError, Region};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RegionOrEndpoint {
    region: Option<String>,
    endpoint: Option<String>,
}

impl RegionOrEndpoint {
    pub fn with_region(region: String) -> Self {
        Self {
            region: Some(region),
            endpoint: None,
        }
    }

    pub fn with_endpoint(endpoint: String) -> Self {
        Self {
            region: None,
            endpoint: Some(endpoint),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum ParseError {
    #[snafu(display("Failed to parse custom endpoint as URI: {}", source))]
    EndpointParseError { source: InvalidUri },
    #[snafu(display("{}", source))]
    RegionParseError { source: ParseRegionError },
    #[snafu(display("Only one of 'region' or 'endpoint' can be specified"))]
    BothRegionAndEndpoint,
    #[snafu(display("Must set either 'region' or 'endpoint'"))]
    MissingRegionAndEndpoint,
}

impl TryFrom<&RegionOrEndpoint> for Region {
    type Error = ParseError;

    fn try_from(r: &RegionOrEndpoint) -> Result<Self, Self::Error> {
        match (&r.region, &r.endpoint) {
            (Some(region), None) => region.parse().context(RegionParseError),
            (None, Some(endpoint)) => endpoint
                .parse::<Uri>()
                .map(|_| Region::Custom {
                    name: "custom".into(),
                    endpoint: endpoint.into(),
                })
                .context(EndpointParseError),
            (Some(_), Some(_)) => Err(ParseError::BothRegionAndEndpoint),
            (None, None) => Err(ParseError::MissingRegionAndEndpoint),
        }
    }
}

impl TryFrom<RegionOrEndpoint> for Region {
    type Error = ParseError;
    fn try_from(r: RegionOrEndpoint) -> Result<Self, Self::Error> {
        Region::try_from(&r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusoto_core::Region;
    use serde::Deserialize;
    use std::convert::TryInto;

    #[derive(Deserialize)]
    struct Config {
        inner: Inner,
    }

    #[derive(Deserialize)]
    struct Inner {
        #[serde(flatten)]
        region: RegionOrEndpoint,
    }

    #[test]
    fn region_es_east_1() {
        let config: Config = toml::from_str(
            r#"
        [inner]
        region = "us-east-1"
        "#,
        )
        .unwrap();

        let region: Region = config.inner.region.try_into().unwrap();
        assert_eq!(region, Region::UsEast1);
    }

    #[test]
    fn region_custom_name_endpoint() {
        let config: Config = toml::from_str(
            r#"
        [inner]
        endpoint = "http://localhost:9000"
        "#,
        )
        .unwrap();

        let expected_region = Region::Custom {
            name: "custom".into(),
            endpoint: "http://localhost:9000".into(),
        };

        let region: Region = config.inner.region.try_into().unwrap();
        assert_eq!(region, expected_region);
    }

    #[test]
    fn region_not_provided() {
        let config: Config = toml::from_str(
            r#"
        [inner]
        endpoint_is_spelled_wrong = "http://localhost:9000"
        "#,
        )
        .unwrap();

        let region: Result<Region, ParseError> = config.inner.region.try_into();
        match region {
            Err(ParseError::MissingRegionAndEndpoint) => {}
            other => panic!("assertion failed, wrong result {:?}", other),
        }
    }
}
