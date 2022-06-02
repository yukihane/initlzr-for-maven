// https://maven.apache.org/xsd/maven-4.0.0.xsd

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "project")]
pub struct Project {
    pub xmlns: String,
    #[serde(rename = "xmlns:xsi")]
    pub xsi: String,
    #[serde(rename = "xsi:schemaLocation")]
    pub schema_location: String,
    #[serde(rename = "$unflatten=modelVersion")]
    pub model_version: String,
    #[serde(rename = "$unflatten=groupId")]
    pub group_id: String,
    #[serde(rename = "$unflatten=artifactId")]
    pub artifact_id: String,
    #[serde(rename = "$unflatten=version")]
    pub version: String,
    #[serde(rename = "$unflatten=packaging")]
    pub packaging: String,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
    pub properties: Properties,
    pub dependencies: Dependencies,
    pub build: Build,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub plugin_management: PluginManagement,
    pub plugins: Plugins,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Plugins {
    #[serde(rename = "plugin")]
    pub plugins: Vec<Plugin>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    #[serde(rename = "$unflatten=groupId")]
    pub group_id: String,
    #[serde(rename = "$unflatten=artifactId")]
    pub artifact_id: String,
    #[serde(rename = "$unflatten=version")]
    pub version: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginManagement {}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Properties {}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dependencies {
    #[serde(rename = "dependency")]
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    #[serde(rename = "$unflatten=groupId")]
    pub group_id: String,
    #[serde(rename = "$unflatten=artifactId")]
    pub artifact_id: String,
    #[serde(rename = "$unflatten=version")]
    pub version: String,
    #[serde(rename = "$unflatten=scope")]
    pub scope: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::se::to_string;

    #[test]
    fn it_works() {
        let jupiter_api = Dependency {
            group_id: "org.junit.jupiter".to_string(),
            artifact_id: "junit-jupiter-api".to_string(),
            version: "5.8.2".to_string(),
            scope: "test".to_string(),
        };
        let jupiter_engine = Dependency {
            group_id: "org.junit.jupiter".to_string(),
            artifact_id: "junit-jupiter-engine".to_string(),
            version: "5.8.2".to_string(),
            scope: "test".to_string(),
        };
        let assertj = Dependency {
            group_id: "org.assertj".to_string(),
            artifact_id: "assertj-core".to_string(),
            version: "3.23.1".to_string(),
            scope: "test".to_string(),
        };

        let dependencies = Dependencies {
            dependencies: vec![jupiter_api, jupiter_engine, assertj],
        };

        let surefire = Plugin {
            group_id: "org.apache.maven.plugins".to_string(),
            artifact_id: "maven-surefire-plugin".to_string(),
            version: "2.22.2".to_string(),
        };

        let failsafe = Plugin {
            group_id: "org.apache.maven.plugins".to_string(),
            artifact_id: "maven-failsafe-plugin".to_string(),
            version: "2.22.2".to_string(),
        };

        let plugins = Plugins {
            plugins: vec![surefire, failsafe],
        };

        let build = Build {
            plugin_management: PluginManagement {},
            plugins,
        };

        let project = Project {
            xmlns: "http://maven.apache.org/POM/4.0.0".to_string(),
            xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            schema_location:
                "http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd"
                    .to_string(),
            model_version: "4.0.0".to_string(),
            group_id: "com.github.yukihane.examples".to_string(),
            artifact_id: "hello-world".to_string(),
            version: "0.0.1-SNAPSHOT".to_string(),
            packaging: "jar".to_string(),
            name: "A project based on java17maven".to_string(),
            properties: Properties {},
            dependencies,
            build,
        };

        let str = to_string(&project).unwrap();
        println!("{}", &str);
    }
}
