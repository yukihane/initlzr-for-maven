use maven::pom_structure::*;
use quick_xml::se::to_string;

pub mod maven;

pub fn generate() -> String {
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

    to_string(&project).unwrap()
}
