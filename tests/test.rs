use resume_smg::*;
use chrono::{DateTime, Utc};
use serde_json;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

fn create_test_resume() -> Resume {
    let mut links = HashMap::new();
    links.insert("github".to_string(), "https://github.com/testuser".to_string());
    links.insert("linkedin".to_string(), "https://linkedin.com/in/testuser".to_string());

    Resume {
        info: PersonalInfo {
            name: "Test User".to_string(),
            location: "San Francisco, CA".to_string(),
            phone_number: "+1-555-0123".to_string(),
            email: "test@example.com".to_string(),
            links,
        },
        experiences: vec![
            Experience {
                id: "exp1".to_string(),
                title: "Senior Software Engineer".to_string(),
                start_date: DateTime::parse_from_rfc3339("2022-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                end_date: None,
                projects: vec!["proj1".to_string(), "proj2".to_string()],
            },
            Experience {
                id: "exp2".to_string(),
                title: "Software Engineer".to_string(),
                start_date: DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                end_date: Some(
                    DateTime::parse_from_rfc3339("2021-12-31T23:59:59Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                projects: vec!["proj3".to_string()],
            },
        ],
        projects: vec![
            Project {
                id: "proj1".to_string(),
                title: "E-commerce Platform".to_string(),
                duration: Some("8 months".to_string()),
                description: "Built scalable e-commerce platform with microservices".to_string(),
                skills: vec!["rust".to_string(), "postgresql".to_string(), "docker".to_string()],
            },
            Project {
                id: "proj2".to_string(),
                title: "Data Pipeline".to_string(),
                duration: Some("4 months".to_string()),
                description: "Real-time data processing pipeline".to_string(),
                skills: vec!["rust".to_string(), "kafka".to_string(), "redis".to_string()],
            },
            Project {
                id: "proj3".to_string(),
                title: "Mobile App Backend".to_string(),
                duration: Some("6 months".to_string()),
                description: "REST API for mobile application".to_string(),
                skills: vec!["python".to_string(), "postgresql".to_string(), "docker".to_string()],
            },
        ],
        skills: vec![
            Skill {
                id: "rust".to_string(),
                name: "Rust".to_string(),
                skill_type: "programming_language".to_string(),
                category: "backend".to_string(),
            },
            Skill {
                id: "python".to_string(),
                name: "Python".to_string(),
                skill_type: "programming_language".to_string(),
                category: "backend".to_string(),
            },
            Skill {
                id: "postgresql".to_string(),
                name: "PostgreSQL".to_string(),
                skill_type: "database".to_string(),
                category: "backend".to_string(),
            },
            Skill {
                id: "docker".to_string(),
                name: "Docker".to_string(),
                skill_type: "tool".to_string(),
                category: "devops".to_string(),
            },
            Skill {
                id: "kafka".to_string(),
                name: "Apache Kafka".to_string(),
                skill_type: "message_queue".to_string(),
                category: "backend".to_string(),
            },
            Skill {
                id: "redis".to_string(),
                name: "Redis".to_string(),
                skill_type: "database".to_string(),
                category: "backend".to_string(),
            },
        ],
    }
}

fn create_test_config() -> Config {
    Config {
        resume: create_test_resume(),
    }
}

#[test]
fn test_resume_serialization() {
    let resume = create_test_resume();
    let json = serde_json::to_string(&resume).expect("Failed to serialize resume");
    let deserialized: Resume = serde_json::from_str(&json).expect("Failed to deserialize resume");
    
    assert_eq!(resume.info.name, deserialized.info.name);
    assert_eq!(resume.experiences.len(), deserialized.experiences.len());
    assert_eq!(resume.projects.len(), deserialized.projects.len());
    assert_eq!(resume.skills.len(), deserialized.skills.len());
}

#[test]
fn test_config_serialization() {
    let config = create_test_config();
    let json = serde_json::to_string(&config).expect("Failed to serialize config");
    let deserialized: Config = serde_json::from_str(&json).expect("Failed to deserialize config");
    
    assert_eq!(config.resume.info.name, deserialized.resume.info.name);
}

#[test]
fn test_build_index() {
    let resume = create_test_resume();
    let index = build_index(&resume);

    assert!(index.skill_to_projects.contains_key("rust"));
    assert!(index.skill_to_projects.contains_key("postgresql"));
    assert!(index.skill_to_projects.contains_key("docker"));
    
    let rust_projects = &index.skill_to_projects["rust"];
    assert_eq!(rust_projects.len(), 2);
    assert!(rust_projects.contains(&"proj1".to_string()));
    assert!(rust_projects.contains(&"proj2".to_string()));

    assert!(index.skill_to_experiences.contains_key("rust"));
    let rust_experiences = &index.skill_to_experiences["rust"];
    assert_eq!(rust_experiences.len(), 1);
    assert!(rust_experiences.contains(&"exp1".to_string()));

    assert!(index.project_to_experiences.contains_key("proj1"));
    assert!(index.project_to_experiences.contains_key("proj3"));
    
    let proj1_experiences = &index.project_to_experiences["proj1"];
    assert_eq!(proj1_experiences.len(), 1);
    assert!(proj1_experiences.contains(&"exp1".to_string()));

    assert_eq!(index.experience_lookup.len(), 2);
    assert_eq!(index.project_lookup.len(), 3);
    assert_eq!(index.skill_lookup.len(), 6);
}

#[test]
fn test_static_generator_creation() {
    let resume = create_test_resume();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    let generator = StaticGenerator::new(resume.clone(), output_path);
    
    assert_eq!(generator.resume.info.name, resume.info.name);
    assert_eq!(generator.resume.experiences.len(), resume.experiences.len());
    assert_eq!(generator.resume.projects.len(), resume.projects.len());
    assert_eq!(generator.resume.skills.len(), resume.skills.len());
}

#[test]
fn test_generate_static_site() {
    let resume = create_test_resume();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    let generator = StaticGenerator::new(resume, output_path.clone());
    generator.generate().expect("Failed to generate static site");

    assert!(fs::metadata(format!("{}/mcp.json", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/resources", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/tools", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/indexes", output_path)).is_ok());
    
    assert!(fs::metadata(format!("{}/resources/info.json", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/resources/experiences.json", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/resources/projects.json", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/resources/skills.json", output_path)).is_ok());
    
    assert!(fs::metadata(format!("{}/tools/get_skills_for_project", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/tools/get_projects_using_skill", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/tools/get_experiences_using_skill", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/tools/get_shared_skills", output_path)).is_ok());
    
    assert!(fs::metadata(format!("{}/indexes/skill_to_projects.json", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/indexes/skill_to_experiences.json", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/indexes/project_to_experiences.json", output_path)).is_ok());
}

#[test]
fn test_mcp_manifest_generation() {
    let resume = create_test_resume();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    let generator = StaticGenerator::new(resume, output_path.clone());
    generator.generate_manifest().expect("Failed to generate manifest");
    
    let manifest_content = fs::read_to_string(format!("{}/mcp.json", output_path))
        .expect("Failed to read manifest file");
    let manifest: MCPManifest = serde_json::from_str(&manifest_content)
        .expect("Failed to parse manifest JSON");
    
    assert_eq!(manifest.protocol_version, "2025-06-18");
    assert_eq!(manifest.server_info.name, "static-resume-mcp");
    assert_eq!(manifest.server_info.version, "0.1.0");
    assert_eq!(manifest.capabilities.resources.len(), 4);
    assert_eq!(manifest.capabilities.tools.len(), 5);
    
    let resource_uris: Vec<String> = manifest.capabilities.resources
        .iter()
        .map(|r| r.uri.clone())
        .collect();
    assert!(resource_uris.contains(&"resume://info".to_string()));
    assert!(resource_uris.contains(&"resume://experiences".to_string()));
    assert!(resource_uris.contains(&"resume://projects".to_string()));
    assert!(resource_uris.contains(&"resume://skills".to_string()));
    
    let tool_names: Vec<String> = manifest.capabilities.tools
        .iter()
        .map(|t| t.name.clone())
        .collect();
    assert!(tool_names.contains(&"get_skills_for_project".to_string()));
    assert!(tool_names.contains(&"get_projects_using_skill".to_string()));
    assert!(tool_names.contains(&"get_experiences_using_skill".to_string()));
    assert!(tool_names.contains(&"get_shared_skills".to_string()));
    assert!(tool_names.contains(&"find_skill_clusters".to_string()));
}

#[test]
fn test_resource_generation() {
    let resume = create_test_resume();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    let generator = StaticGenerator::new(resume.clone(), output_path.clone());
    
    fs::create_dir_all(format!("{}/resources", output_path))
        .expect("Failed to create resources directory");
        
    generator.generate_resources().expect("Failed to generate resources");
    
    let info_content = fs::read_to_string(format!("{}/resources/info.json", output_path))
        .expect("Failed to read info.json");
    let info_resource: MCPResourceContent = serde_json::from_str(&info_content)
        .expect("Failed to parse info.json");
    assert_eq!(info_resource.uri, "resume://info");
    assert_eq!(info_resource.mime_type, "application/json");
    
    let exp_content = fs::read_to_string(format!("{}/resources/experiences.json", output_path))
        .expect("Failed to read experiences.json");
    let exp_resource: MCPResourceContent = serde_json::from_str(&exp_content)
        .expect("Failed to parse experiences.json");
    assert_eq!(exp_resource.uri, "resume://experiences");
    
    let experiences: Vec<Experience> = serde_json::from_str(&exp_resource.text)
        .expect("Failed to parse experiences data");
    assert_eq!(experiences.len(), 2);
    assert_eq!(experiences[0].title, "Senior Software Engineer");
}

#[test]
fn test_tool_results_generation() {
    let resume = create_test_resume();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    let generator = StaticGenerator::new(resume, output_path.clone());
    
    fs::create_dir_all(format!("{}/tools", output_path))
        .expect("Failed to create tools directory");
    generator.generate_tool_results().expect("Failed to generate tool results");
    
    let proj1_skills_path = format!("{}/tools/get_skills_for_project/proj1.json", output_path);
    assert!(fs::metadata(&proj1_skills_path).is_ok());
    
    let proj1_content = fs::read_to_string(&proj1_skills_path)
        .expect("Failed to read proj1 skills");
    let proj1_result: MCPToolResult = serde_json::from_str(&proj1_content)
        .expect("Failed to parse proj1 skills result");
    assert_eq!(proj1_result.content.len(), 1);
    assert_eq!(proj1_result.content[0].content_type, "text");
    
    let skills: Vec<Skill> = serde_json::from_str(&proj1_result.content[0].text)
        .expect("Failed to parse skills data");
    assert_eq!(skills.len(), 3);
    
    let rust_projects_path = format!("{}/tools/get_projects_using_skill/rust.json", output_path);
    assert!(fs::metadata(&rust_projects_path).is_ok());
    
    let rust_content = fs::read_to_string(&rust_projects_path)
        .expect("Failed to read rust projects");
    let rust_result: MCPToolResult = serde_json::from_str(&rust_content)
        .expect("Failed to parse rust projects result");
    
    let projects: Vec<Project> = serde_json::from_str(&rust_result.content[0].text)
        .expect("Failed to parse projects data");
    assert_eq!(projects.len(), 2);
    
    let clusters_path = format!("{}/tools/find_skill_clusters.json", output_path);
    assert!(fs::metadata(&clusters_path).is_ok());
}

#[test]
fn test_index_generation() {
    let resume = create_test_resume();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    let generator = StaticGenerator::new(resume, output_path.clone());
    
    fs::create_dir_all(format!("{}/indexes", output_path))
        .expect("Failed to create indexes directory");
        
    generator.generate_indexes().expect("Failed to generate indexes");
    
    let skill_projects_path = format!("{}/indexes/skill_to_projects.json", output_path);
    assert!(fs::metadata(&skill_projects_path).is_ok());
    
    let content = fs::read_to_string(&skill_projects_path)
        .expect("Failed to read skill_to_projects index");
    let index: HashMap<String, Vec<String>> = serde_json::from_str(&content)
        .expect("Failed to parse skill_to_projects index");
    
    assert!(index.contains_key("rust"));
    assert!(index.contains_key("postgresql"));
    assert_eq!(index["rust"].len(), 2);
    assert!(index["rust"].contains(&"proj1".to_string()));
    assert!(index["rust"].contains(&"proj2".to_string()));
}

#[test]
fn test_shared_skills_computation() {
    let resume = create_test_resume();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    let generator = StaticGenerator::new(resume, output_path.clone());
    
    fs::create_dir_all(format!("{}/tools", output_path))
        .expect("Failed to create tools directory");
    generator.generate_tool_results().expect("Failed to generate tool results");
    
    let shared_path = format!("{}/tools/get_shared_skills/proj1/proj3.json", output_path);
    assert!(fs::metadata(&shared_path).is_ok());
    
    let content = fs::read_to_string(&shared_path)
        .expect("Failed to read shared skills");
    let result: MCPToolResult = serde_json::from_str(&content)
        .expect("Failed to parse shared skills result");
    
    let shared_skills: Vec<Skill> = serde_json::from_str(&result.content[0].text)
        .expect("Failed to parse shared skills data");
    assert_eq!(shared_skills.len(), 2);
    
    let skill_names: Vec<String> = shared_skills.iter().map(|s| s.name.clone()).collect();
    assert!(skill_names.contains(&"PostgreSQL".to_string()));
    assert!(skill_names.contains(&"Docker".to_string()));
}

#[test]
fn test_empty_resume_handling() {
    let empty_resume = Resume {
        info: PersonalInfo {
            name: "Empty User".to_string(),
            location: "".to_string(),
            phone_number: "".to_string(),
            email: "".to_string(),
            links: HashMap::new(),
        },
        experiences: vec![],
        projects: vec![],
        skills: vec![],
    };
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    let generator = StaticGenerator::new(empty_resume, output_path.clone());
    let result = generator.generate();
    
    assert!(result.is_ok());
    assert!(fs::metadata(format!("{}/mcp.json", output_path)).is_ok());
    assert!(fs::metadata(format!("{}/resources", output_path)).is_ok());
}

#[test]
fn test_skill_clustering_algorithm() {
    let mut test_resume = create_test_resume();
    
    test_resume.projects = vec![
        Project {
            id: "proj_a".to_string(),
            title: "Project A".to_string(),
            duration: Some("3 months".to_string()),
            description: "Test project".to_string(),
            skills: vec!["skill1".to_string(), "skill2".to_string(), "skill3".to_string()],
        },
        Project {
            id: "proj_b".to_string(),
            title: "Project B".to_string(),
            duration: Some("4 months".to_string()),
            description: "Another test project".to_string(),
            skills: vec!["skill1".to_string(), "skill2".to_string(), "skill4".to_string()],
        },
        Project {
            id: "proj_c".to_string(),
            title: "Project C".to_string(),
            duration: Some("2 months".to_string()),
            description: "Third test project".to_string(),
            skills: vec!["skill5".to_string()],
        },
    ];
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    let generator = StaticGenerator::new(test_resume, output_path.clone());
    
    fs::create_dir_all(format!("{}/tools", output_path))
        .expect("Failed to create tools directory");
    generator.generate_tool_results().expect("Failed to generate tool results");

    let clusters_path = format!("{}/tools/find_skill_clusters.json", output_path);
    let content = fs::read_to_string(&clusters_path)
        .expect("Failed to read skill clusters");
    let result: MCPToolResult = serde_json::from_str(&content)
        .expect("Failed to parse skill clusters result");
    
    let clusters: HashMap<String, Vec<String>> = serde_json::from_str(&result.content[0].text)
        .expect("Failed to parse clusters data");
    
    let pair_key = "skill1,skill2";
    assert!(clusters.contains_key(pair_key), 
            "Expected to find cluster with key '{}', found keys: {:?}", 
            pair_key, clusters.keys().collect::<Vec<_>>());
    
    let projects_in_cluster = &clusters[pair_key];
    assert_eq!(projects_in_cluster.len(), 2);
    assert!(projects_in_cluster.contains(&"proj_a".to_string()));
    assert!(projects_in_cluster.contains(&"proj_b".to_string()));
    
    let full_set_a = "skill1,skill2,skill3";
    let full_set_b = "skill1,skill2,skill4";
    
    assert!(!clusters.contains_key(full_set_a), 
            "Full skill set '{}' should not be clustered (appears only once)", full_set_a);
    assert!(!clusters.contains_key(full_set_b), 
            "Full skill set '{}' should not be clustered (appears only once)", full_set_b);
}

#[test]
fn test_skill_clusters_identification() {
    let resume = create_test_resume();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_path = temp_dir.path().to_str().unwrap().to_string();
    
    let generator = StaticGenerator::new(resume, output_path.clone());
    fs::create_dir_all(format!("{}/tools", output_path))
        .expect("Failed to create tools directory");
        
    generator.generate_tool_results().expect("Failed to generate tool results");
    
    let clusters_path = format!("{}/tools/find_skill_clusters.json", output_path);
    let content = fs::read_to_string(&clusters_path)
        .expect("Failed to read skill clusters");
    let result: MCPToolResult = serde_json::from_str(&content)
        .expect("Failed to parse skill clusters result");
    
    let clusters: HashMap<String, Vec<String>> = serde_json::from_str(&result.content[0].text)
        .expect("Failed to parse clusters data");
        
    assert!(!clusters.is_empty(), "Expected to find at least one skill cluster");
    
    let docker_postgresql_key = "docker,postgresql";
    if let Some(projects) = clusters.get(docker_postgresql_key) {
        assert!(projects.len() >= 2, 
                "Expected docker+postgresql to appear in at least 2 projects, found: {:?}", projects);
        assert!(projects.contains(&"proj1".to_string()), 
                "Expected proj1 to use docker+postgresql");
        assert!(projects.contains(&"proj3".to_string()), 
                "Expected proj3 to use docker+postgresql");
    } else {
        panic!("Expected to find docker,postgresql cluster. Available clusters: {:?}", clusters.keys().collect::<Vec<_>>());
    }
    
    for (skills, projects) in &clusters {
        assert!(projects.len() > 1, 
                "Cluster '{}' should have more than 1 project, found: {:?}", 
                skills, projects);
    }
}