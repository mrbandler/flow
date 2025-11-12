use anyhow::Result;
use chrono::Local;
use loro::{ExportMode, LoroDoc, UpdateOptions};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const FLOW_DIR: &str = ".flow";
const METADATA_FILE: &str = "graph.toml";
const DOCUMENT_FILE: &str = "graph.loro";
const JOURNAL_DIR: &str = "journal"; // TODO: Implement ability to customize for the user

/// Graph metadata.
///
/// # Fields
///
/// - `name` (`String`) - Name of the graph.
/// - `version` (`String`) - Version the graph was created with.
#[derive(serde::Serialize, serde::Deserialize)]
struct Metadata {
    name: String,
    version: String,
}

/// Node graph.
///
/// # Fields
///
/// - `path` (`PathBuf`) - Path of the graph.
/// - `metadata` (`Metadata`) - Metadata of the graph.
pub struct Graph {
    path: PathBuf,
    metadata: Metadata,
    document: LoroDoc,
    dirty: HashSet<String>,
}

impl Graph {
    /// Initializes a new graph given a path and a optional name.
    ///
    /// # Arguments
    ///
    /// - `path` (`&Path`) - Path to create the graph in.
    /// - `name` (`Option<&String>`) - Optional name of the graph (if none is provided it will fallback to the path's basename).
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Initialized graph.
    ///
    /// # Errors
    ///
    /// IO errors when creating directories or writing files.
    pub fn init(path: &Path, name: Option<&String>) -> Result<Self> {
        let flow_dir = path.join(FLOW_DIR);
        fs::create_dir_all(&flow_dir)?;

        let graph_name = name.map(|s| s.to_string()).unwrap_or_else(|| {
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("flow-graph") // TODO: Find a better default name or generate one
                .to_string()
        });
        let metadata = Metadata {
            name: graph_name,
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let metadata_path = flow_dir.join(METADATA_FILE);
        let metadata_json = toml::to_string_pretty(&metadata)?;
        fs::write(metadata_path, metadata_json)?;

        let doc = LoroDoc::new();
        let doc_path = flow_dir.join(DOCUMENT_FILE);
        let snapshot = doc.export(ExportMode::Snapshot)?;
        fs::write(doc_path, snapshot)?;

        Ok(Graph {
            path: path.to_path_buf(),
            metadata: metadata,
            document: doc,
            dirty: HashSet::new(),
        })
    }

    /// Loads a graph given a path.
    ///
    /// # Arguments
    ///
    /// - `path` (`&Path`) - Path of the graph to load.
    ///
    /// # Returns
    ///
    /// - `Result<Self>` - Loaded graph.
    ///
    /// # Errors
    ///
    /// IO errors when creating directories or writing files.
    pub fn load(path: &Path) -> Result<Self> {
        let flow_dir = path.join(FLOW_DIR);
        let metadata_path = flow_dir.join(METADATA_FILE);

        let metadata_json = std::fs::read_to_string(metadata_path)?;
        let metadata: Metadata = toml::from_str(&metadata_json)?;

        let doc = LoroDoc::new();
        let doc_path = flow_dir.join(DOCUMENT_FILE);
        if doc_path.exists() {
            let doc_content = std::fs::read(doc_path)?;
            doc.import(&doc_content)?;
        }

        // TODO: Load and index all markdown files in the graph directory.

        Ok(Graph {
            path: path.to_path_buf(),
            metadata: metadata,
            document: doc,
            dirty: HashSet::new(),
        })
    }

    /// Adds a node to the todays page.
    ///
    /// # Arguments
    ///
    /// - `&mut self` (`Graph`) - Graph to add the node to todays page to.
    /// - `content` (`&str`) - Content to add.
    ///
    /// # Errors
    ///
    /// IO errors when creating directories or writing files.
    pub fn add(&mut self, content: &str) -> Result<()> {
        let journal_dir = self.path.join(JOURNAL_DIR);
        fs::create_dir_all(&journal_dir)?;

        let today = Local::now().format("%Y-%m-%d").to_string();
        let daily_path = journal_dir.join(format!("{}.md", today));
        let id = format!("{}/{}.md", JOURNAL_DIR, today);
        let text = self.document.get_text(id.clone());

        if daily_path.exists() {
            let existing = fs::read_to_string(&daily_path)?;
            text.update(&existing, UpdateOptions::default())?;
        }

        // TODO: Check content to add for multi lines. Currently we assume that it's a single line.
        let prefix = if text.is_empty() { "" } else { "\n" };
        text.push_str(&format!("{}- {}", prefix, content))?;

        self.dirty.insert(id);
        self.save()?;

        Ok(())
    }

    /// Saves the graph to disk.
    ///
    /// # Arguments
    ///
    /// - `&self` (`Graph`) - Graph to save.
    ///
    /// # Errors
    ///
    /// IO errors when writing files.
    fn save(&mut self) -> Result<()> {
        let flow_dir = self.path.join(FLOW_DIR);

        let metadata_path = flow_dir.join(METADATA_FILE);
        let metadata_json = toml::to_string_pretty(&self.metadata)?;
        fs::write(metadata_path, metadata_json)?;

        let doc_path = flow_dir.join(DOCUMENT_FILE);
        let snapshot = self.document.export(ExportMode::Snapshot)?;
        fs::write(doc_path, snapshot)?;

        for id in &self.dirty {
            let file_path = self.path.join(id);
            let text = self.document.get_text(id.to_string());
            fs::write(file_path, text.to_string())?;
        }
        self.dirty.clear();

        Ok(())
    }
}
