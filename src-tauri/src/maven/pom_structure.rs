// https://maven.apache.org/xsd/maven-4.0.0.xsd

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "project")]
pub struct Project {
    pub xmlns: String,
    #[serde(rename = "xml:xsi")]
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
pub struct Build {}

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::se::to_string;

    #[test]
    fn it_works() {
        let dep1 = Dependency {
            group_id: "test".to_string(),
            artifact_id: "test".to_string(),
            version: "1.0.0".to_string(),
        };
        let dep2 = Dependency {
            group_id: "test".to_string(),
            artifact_id: "test".to_string(),
            version: "1.0.0".to_string(),
        };
        let dependencies = Dependencies {
            dependencies: vec![dep1, dep2],
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
            build: Build {},
        };

        let str = to_string(&project).unwrap();
        println!("{}", &str);
    }
}
