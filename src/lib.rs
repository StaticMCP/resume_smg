use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub resume: Resume,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub info: PersonalInfo,
    pub experiences: Vec<Experience>,
    pub projects: Vec<Project>,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub name: String,
    pub location: String,
    pub phone_number: String,
    pub email: String,
    pub links: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub id: String,
    pub title: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub duration: Option<String>,
    pub description: String,
    pub skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub skill_type: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResource {
    pub uri: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPToolSchema {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPManifest {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: MCPCapabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: MCPServerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPCapabilities {
    pub resources: Vec<MCPResource>,
    pub tools: Vec<MCPToolSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResourceContent {
    pub uri: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPToolResult {
    pub content: Vec<MCPToolContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ResumeIndex {
    pub skill_to_projects: HashMap<String, Vec<String>>,
    pub skill_to_experiences: HashMap<String, Vec<String>>,
    pub project_to_experiences: HashMap<String, Vec<String>>,
    pub experience_lookup: HashMap<String, Experience>,
    pub project_lookup: HashMap<String, Project>,
    pub skill_lookup: HashMap<String, Skill>,
}

pub struct StaticGenerator {
    pub resume: Resume,
    pub index: ResumeIndex,
    pub output_dir: String,
}

impl StaticGenerator {
    pub fn new(resume: Resume, output_dir: String) -> Self {
        let index = build_index(&resume);
        Self {
            resume,
            index,
            output_dir,
        }
    }

    pub fn generate(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.output_dir)?;
        fs::create_dir_all(format!("{}/resources", self.output_dir))?;
        fs::create_dir_all(format!("{}/tools", self.output_dir))?;
        fs::create_dir_all(format!("{}/indexes", self.output_dir))?;

        self.generate_manifest()?;
        self.generate_resources()?;
        self.generate_tool_results()?;
        self.generate_indexes()?;

        println!("Static MCP site generated in: {}", self.output_dir);
        Ok(())
    }

    pub fn generate_manifest(&self) -> Result<(), Box<dyn std::error::Error>> {
        let manifest = MCPManifest {
            protocol_version: "2025-06-18".to_string(),
            capabilities: MCPCapabilities {
                resources: vec![
                    MCPResource {
                        uri: "resume://info".to_string(),
                        name: "Personal Information".to_string(),
                        description: "Basic personal details and contact information".to_string(),
                        mime_type: "application/json".to_string(),
                    },
                    MCPResource {
                        uri: "resume://experiences".to_string(),
                        name: "All Experiences".to_string(),
                        description: "Complete list of work experiences".to_string(),
                        mime_type: "application/json".to_string(),
                    },
                    MCPResource {
                        uri: "resume://projects".to_string(),
                        name: "All Projects".to_string(),
                        description: "Complete list of projects".to_string(),
                        mime_type: "application/json".to_string(),
                    },
                    MCPResource {
                        uri: "resume://skills".to_string(),
                        name: "All Skills".to_string(),
                        description: "Complete list of skills".to_string(),
                        mime_type: "application/json".to_string(),
                    },
                ],
                tools: vec![
                    MCPToolSchema {
                        name: "get_skills_for_project".to_string(),
                        description: "Get all skills used in a specific project".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "project_id": {"type": "string", "description": "Project ID"}
                            },
                            "required": ["project_id"]
                        }),
                    },
                    MCPToolSchema {
                        name: "get_projects_using_skill".to_string(),
                        description: "Get all projects that use a specific skill".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "skill_id": {"type": "string", "description": "Skill ID"}
                            },
                            "required": ["skill_id"]
                        }),
                    },
                    MCPToolSchema {
                        name: "get_experiences_using_skill".to_string(),
                        description: "Get all experiences that involve a specific skill".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "skill_id": {"type": "string", "description": "Skill ID"}
                            },
                            "required": ["skill_id"]
                        }),
                    },
                    MCPToolSchema {
                        name: "get_shared_skills".to_string(),
                        description: "Get skills shared between two projects".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "project_a": {"type": "string", "description": "First project ID"},
                                "project_b": {"type": "string", "description": "Second project ID"}
                            },
                            "required": ["project_a", "project_b"]
                        }),
                    },
                    MCPToolSchema {
                        name: "find_skill_clusters".to_string(),
                        description: "Find clusters of skills that frequently appear together".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {},
                            "required": []
                        }),
                    },
                ],
            },
            server_info: MCPServerInfo {
                name: "static-resume-mcp".to_string(),
                version: "0.1.0".to_string(),
            },
        };

        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        fs::write(format!("{}/mcp.json", self.output_dir), manifest_json)?;
        Ok(())
    }

    pub fn generate_resources(&self) -> Result<(), Box<dyn std::error::Error>> {
        let info_content = MCPResourceContent {
            uri: "resume://info".to_string(),
            mime_type: "application/json".to_string(),
            text: serde_json::to_string_pretty(&self.resume.info)?,
        };
        fs::write(
            format!("{}/resources/info.json", self.output_dir),
            serde_json::to_string_pretty(&info_content)?,
        )?;

        let experiences_content = MCPResourceContent {
            uri: "resume://experiences".to_string(),
            mime_type: "application/json".to_string(),
            text: serde_json::to_string_pretty(&self.resume.experiences)?,
        };
        fs::write(
            format!("{}/resources/experiences.json", self.output_dir),
            serde_json::to_string_pretty(&experiences_content)?,
        )?;

        let projects_content = MCPResourceContent {
            uri: "resume://projects".to_string(),
            mime_type: "application/json".to_string(),
            text: serde_json::to_string_pretty(&self.resume.projects)?,
        };
        fs::write(
            format!("{}/resources/projects.json", self.output_dir),
            serde_json::to_string_pretty(&projects_content)?,
        )?;

        let skills_content = MCPResourceContent {
            uri: "resume://skills".to_string(),
            mime_type: "application/json".to_string(),
            text: serde_json::to_string_pretty(&self.resume.skills)?,
        };
        fs::write(
            format!("{}/resources/skills.json", self.output_dir),
            serde_json::to_string_pretty(&skills_content)?,
        )?;

        Ok(())
    }

    pub fn generate_tool_results(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(format!("{}/tools/get_skills_for_project", self.output_dir))?;
        fs::create_dir_all(format!("{}/tools/get_projects_using_skill", self.output_dir))?;
        fs::create_dir_all(format!("{}/tools/get_experiences_using_skill", self.output_dir))?;
        fs::create_dir_all(format!("{}/tools/get_shared_skills", self.output_dir))?;

        for project in &self.resume.projects {
            let skills: Vec<&Skill> = project
                .skills
                .iter()
                .filter_map(|id| self.index.skill_lookup.get(id))
                .collect();
            
            let result = MCPToolResult {
                content: vec![MCPToolContent {
                    content_type: "text".to_string(),
                    text: serde_json::to_string_pretty(&skills)?,
                }],
            };
            
            fs::write(
                format!("{}/tools/get_skills_for_project/{}.json", self.output_dir, project.id),
                serde_json::to_string_pretty(&result)?,
            )?;
        }

        for skill in &self.resume.skills {
            let project_ids = self
                .index
                .skill_to_projects
                .get(&skill.id)
                .cloned()
                .unwrap_or_default();
            
            let projects: Vec<&Project> = project_ids
                .iter()
                .filter_map(|id| self.index.project_lookup.get(id))
                .collect();
            
            let result = MCPToolResult {
                content: vec![MCPToolContent {
                    content_type: "text".to_string(),
                    text: serde_json::to_string_pretty(&projects)?,
                }],
            };
            
            fs::write(
                format!("{}/tools/get_projects_using_skill/{}.json", self.output_dir, skill.id),
                serde_json::to_string_pretty(&result)?,
            )?;
        }

        for skill in &self.resume.skills {
            let experience_ids = self
                .index
                .skill_to_experiences
                .get(&skill.id)
                .cloned()
                .unwrap_or_default();
            
            let experiences: Vec<&Experience> = experience_ids
                .iter()
                .filter_map(|id| self.index.experience_lookup.get(id))
                .collect();
            
            let result = MCPToolResult {
                content: vec![MCPToolContent {
                    content_type: "text".to_string(),
                    text: serde_json::to_string_pretty(&experiences)?,
                }],
            };
            
            fs::write(
                format!("{}/tools/get_experiences_using_skill/{}.json", self.output_dir, skill.id),
                serde_json::to_string_pretty(&result)?,
            )?;
        }

        for (i, project_a) in self.resume.projects.iter().enumerate() {
            fs::create_dir_all(format!("{}/tools/get_shared_skills/{}", self.output_dir, project_a.id))?;
            
            for project_b in self.resume.projects.iter().skip(i + 1) {
                let skills_a: HashSet<String> = project_a.skills.iter().cloned().collect();
                let skills_b: HashSet<String> = project_b.skills.iter().cloned().collect();
                
                let shared: Vec<&Skill> = skills_a
                    .intersection(&skills_b)
                    .filter_map(|id| self.index.skill_lookup.get(id))
                    .collect();
                
                let result = MCPToolResult {
                    content: vec![MCPToolContent {
                        content_type: "text".to_string(),
                        text: serde_json::to_string_pretty(&shared)?,
                    }],
                };
                
                fs::write(
                    format!("{}/tools/get_shared_skills/{}/{}.json", self.output_dir, project_a.id, project_b.id),
                    serde_json::to_string_pretty(&result)?,
                )?;
                
                fs::create_dir_all(format!("{}/tools/get_shared_skills/{}", self.output_dir, project_b.id))?;
                fs::write(
                    format!("{}/tools/get_shared_skills/{}/{}.json", self.output_dir, project_b.id, project_a.id),
                    serde_json::to_string_pretty(&result)?,
                )?;
            }
        }

        let mut skill_combinations = HashMap::new();
        
        for project in &self.resume.projects {
            if project.skills.len() > 1 {
                for i in 0..project.skills.len() {
                    for j in (i + 1)..project.skills.len() {
                        let mut pair = vec![project.skills[i].clone(), project.skills[j].clone()];
                        pair.sort();
                        let key = pair.join(",");
                        
                        skill_combinations
                            .entry(key)
                            .or_insert_with(Vec::new)
                            .push(project.id.clone());
                    }
                }
                
                if project.skills.len() >= 3 {
                    let mut sorted_skills = project.skills.clone();
                    sorted_skills.sort();
                    let key = sorted_skills.join(",");
                    skill_combinations
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .push(project.id.clone());
                }
            }
        }
        
        let significant_clusters: HashMap<String, Vec<String>> = skill_combinations
            .into_iter()
            .filter(|(_, projects)| projects.len() > 1)
            .collect();
        
        let result = MCPToolResult {
            content: vec![MCPToolContent {
                content_type: "text".to_string(),
                text: serde_json::to_string_pretty(&significant_clusters)?,
            }],
        };
        
        fs::write(
            format!("{}/tools/find_skill_clusters.json", self.output_dir),
            serde_json::to_string_pretty(&result)?,
        )?;

        Ok(())
    }

    pub fn generate_indexes(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(
            format!("{}/indexes/skill_to_projects.json", self.output_dir),
            serde_json::to_string_pretty(&self.index.skill_to_projects)?,
        )?;
        fs::write(
            format!("{}/indexes/skill_to_experiences.json", self.output_dir),
            serde_json::to_string_pretty(&self.index.skill_to_experiences)?,
        )?;
        fs::write(
            format!("{}/indexes/project_to_experiences.json", self.output_dir),
            serde_json::to_string_pretty(&self.index.project_to_experiences)?,
        )?;

        Ok(())
    }
}

pub fn build_index(resume: &Resume) -> ResumeIndex {
    let mut skill_to_projects = HashMap::new();
    let mut skill_to_experiences = HashMap::new();
    let mut project_to_experiences = HashMap::new();
    
    let experience_lookup: HashMap<String, Experience> = 
        resume.experiences.iter().map(|e| (e.id.clone(), e.clone())).collect();
    let project_lookup: HashMap<String, Project> = 
        resume.projects.iter().map(|p| (p.id.clone(), p.clone())).collect();
    let skill_lookup: HashMap<String, Skill> = 
        resume.skills.iter().map(|s| (s.id.clone(), s.clone())).collect();

    for project in &resume.projects {
        for skill_id in &project.skills {
            skill_to_projects
                .entry(skill_id.clone())
                .or_insert_with(Vec::new)
                .push(project.id.clone());
        }
    }

    for experience in &resume.experiences {
        for project_id in &experience.projects {
            project_to_experiences
                .entry(project_id.clone())
                .or_insert_with(Vec::new)
                .push(experience.id.clone());

            if let Some(project) = project_lookup.get(project_id) {
                for skill_id in &project.skills {
                    skill_to_experiences
                        .entry(skill_id.clone())
                        .or_insert_with(Vec::new)
                        .push(experience.id.clone());
                }
            }
        }
    }

    for vec in skill_to_experiences.values_mut() {
        vec.sort();
        vec.dedup();
    }

    ResumeIndex {
        skill_to_projects,
        skill_to_experiences,
        project_to_experiences,
        experience_lookup,
        project_lookup,
        skill_lookup,
    }
}
