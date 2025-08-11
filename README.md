# Resume StaticMCP Generator

A Rust tool that generates [StaticMCP](https://staticmcp.com) from resume data, enabling AI assistants to efficiently access and analyze professional information through pre-computed resources and tools.

## Features

- **Static Generation**: Pre-computes all MCP resources and tool results for instant access
- **Relationship Analysis**: Automatically builds indexes linking skills, projects, and experiences
- **Skill Clustering**: Identifies frequently co-occurring skills across projects
- **Cross-References**: Maps relationships between projects, experiences, and skills

_Note: You'll still need a [StaticMCP bridge](https://staticmcp.com/docs/bridge) to connect this to an AI._

## Installation

**Prerequisites**
- Rust 1.70+ with Cargo

### Install from Cargo

```bash
cargo install resume_smg
```

### Build from Source

```bash
git clone git@github.com:binhonglee/resume_smg
cd resume_smg
cargo build --release
```

## Usage

### 1. Create Configuration File

Create a `config.json` file with your resume data:

```json
{
  "resume": {
    "info": {
      "name": "Your Name",
      "location": "City, State",
      "phone_number": "+1-555-0123",
      "email": "you@example.com",
      "links": {
        "github": "https://github.com/yourusername",
        "linkedin": "https://linkedin.com/in/yourprofile"
      }
    },
    "experiences": [
      {
        "id": "exp1",
        "title": "Senior Software Engineer",
        "start_date": "2022-01-01T00:00:00Z",
        "end_date": null,
        "projects": ["proj1", "proj2"]
      }
    ],
    "projects": [
      {
        "id": "proj1",
        "title": "E-commerce Platform",
        "duration": "6 months",
        "description": "Built scalable e-commerce platform",
        "skills": ["rust", "postgresql", "docker"]
      }
    ],
    "skills": [
      {
        "id": "rust",
        "name": "Rust",
        "type": "programming_language",
        "category": "backend"
      }
    ]
  }
}
```

### 2. Generate Static Site

```bash
# Using default paths
./resume_smg

# Or specify custom paths
./resume_smg config.json ./output-directory
```

### 3. Host the Generated Site

The generated static files can be hosted on any web server or CDN (GitHub Pages, Netlify, etc.):

## Generated Structure

```
dist/
  ├── mcp.json                    # MCP manifest
  ├── resources/                  # Static resources
  │     ├── info.json
  │     ├── experiences.json
  │     ├── projects.json
  │     └── skills.json
  ├── tools/                      # Pre-computed tool results
  │     ├── get_skills_for_project/
  │     ├── get_projects_using_skill/
  │     ├── get_experiences_using_skill/
  │     ├── get_shared_skills/
  │     └── find_skill_clusters.json
  └── indexes/                    # Lookup indexes
        ├── skill_to_projects.json
        ├── skill_to_experiences.json
        └── project_to_experiences.json
```

## MCP Resources

The generated MCP server provides these resources:

- **`resume://info`** - Personal information and contact details
- **`resume://experiences`** - Complete list of work experiences
- **`resume://projects`** - Complete list of projects
- **`resume://skills`** - Complete list of skills

## MCP Tools

Pre-computed tools for complex queries:

- **`get_skills_for_project`** - Get all skills used in a specific project
- **`get_projects_using_skill`** - Get all projects that use a specific skill
- **`get_experiences_using_skill`** - Get all experiences involving a specific skill
- **`get_shared_skills`** - Get skills shared between two projects
- **`find_skill_clusters`** - Find clusters of skills that frequently appear together (includes both skill pairs and full skill sets)

## Data Model

### Configuration Structure

- **Config**: Top-level configuration with port, resume data, and optional password
- **Resume**: Contains personal info, experiences, projects, and skills
- **PersonalInfo**: Basic contact information and social links
- **Experience**: Work experience with date ranges and associated projects
- **Project**: Individual project with description, duration, and required skills
- **Skill**: Technical or soft skill with categorization

### Relationships

- Experiences contain multiple Projects (many-to-many)
- Projects require multiple Skills (many-to-many)
- Skills are categorized by type and category
- Automatic indexing creates reverse lookups for efficient querying

## Testing

Run the test suite:

```bash
cargo test
```
