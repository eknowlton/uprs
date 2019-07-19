use config::{Config, ConfigError, File};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq)]
struct SlackNotification {
    enabled: bool,
    api_key: String,
    channel: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Notifications {
    slack: SlackNotification,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Site {
    uri: String,
    interval: usize,
}

type Sites = HashMap<String, Site>;

#[derive(Debug)]
pub struct Settings {
    notifications: Notifications,
    sites: Sites,
}

impl Settings {
    pub fn new(config_dir: PathBuf) -> Result<Self, ConfigError> {
        let notifications = Settings::notifications(&config_dir)?;
        let sites = Settings::sites(&config_dir)?;

        Ok(Settings {
            notifications,
            sites,
        })
    }

    fn sites(config_dir: &PathBuf) -> Result<Sites, ConfigError> {
        let mut sites = Config::new();
        let mut config_dir = config_dir.clone();
        config_dir.push("sites");

        match sites.merge(File::with_name(config_dir.as_path().to_str().unwrap())) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        match sites.try_into::<Sites>() {
            Ok(sites) => Ok(sites),
            Err(e) => Err(e),
        }
    }

    fn notifications(config_dir: &PathBuf) -> Result<Notifications, ConfigError> {
        let mut notifications = Config::new();
        let mut config_dir = config_dir.clone();
        config_dir.push("notifications");

        match notifications.merge(File::with_name(config_dir.as_path().to_str().unwrap())) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        match notifications.try_into::<Notifications>() {
            Ok(notifications) => Ok(notifications),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_settings_sites() {
        let config_dir = PathBuf::from_str("config").unwrap();
        let sites = Settings::sites(&config_dir);

        let mut expected = HashMap::new();
        expected.insert(
            String::from("google"),
            Site {
                uri: String::from("https://google.com"),
                interval: 10,
            },
        );
        expected.insert(
            String::from("digg"),
            Site {
                uri: String::from("https://digg.com"),
                interval: 10,
            },
        );

        assert_eq!(expected, sites.unwrap());
    }

    #[test]
    fn test_settings_notifications() {
        let config_dir = PathBuf::from_str("config").unwrap();
        let notifications = Settings::notifications(&config_dir);

        let expected = Notifications {
            slack: SlackNotification {
                enabled: true,
                api_key: String::from("sdfk3oasdfsadfkj239dsdff"),
                channel: String::from("#devops"),
            },
        };

        assert_eq!(expected, notifications.unwrap());
    }
}
