//! Integration tests for Flow
//!
//! These tests verify the behavior of Flow as a whole, testing the interaction
//! between different components.

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper struct for managing test directories
struct TestEnv {
    #[allow(dead_code)]
    temp_dir: TempDir,
    path: PathBuf,
}

impl TestEnv {
    /// Create a new test environment with a temporary directory
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let path = temp_dir.path().to_path_buf();
        Self { temp_dir, path }
    }

    /// Get the path to the test directory
    fn path(&self) -> &PathBuf {
        &self.path
    }
}

mod space_tests {
    use super::*;
    use flow_core::Space;

    #[test]
    fn test_space_init_creates_flow_directory() {
        let env = TestEnv::new();

        let space = Space::init(env.path(), None).expect("Failed to initialize space");

        // Verify .flow directory was created
        let flow_dir = env.path().join(".flow");
        assert!(flow_dir.exists(), ".flow directory should exist");
        assert!(flow_dir.is_dir(), ".flow should be a directory");

        // Verify the space name is derived from the directory name
        assert!(!space.name().is_empty(), "Space should have a name");
    }

    #[test]
    fn test_space_init_with_custom_name() {
        let env = TestEnv::new();
        let custom_name = "My Custom Space".to_string();

        let space =
            Space::init(env.path(), Some(&custom_name)).expect("Failed to initialize space");

        assert_eq!(space.name(), "My Custom Space");
    }

    #[test]
    fn test_space_init_creates_journal_directory() {
        let env = TestEnv::new();

        let _space = Space::init(env.path(), None).expect("Failed to initialize space");

        let journal_dir = env.path().join("journal");
        assert!(journal_dir.exists(), "journal directory should exist");
        assert!(journal_dir.is_dir(), "journal should be a directory");
    }

    #[test]
    fn test_space_exists_returns_false_for_empty_directory() {
        let env = TestEnv::new();

        assert!(
            !Space::exists(env.path()),
            "Empty directory should not be a space"
        );
    }

    #[test]
    fn test_space_exists_returns_true_after_init() {
        let env = TestEnv::new();

        Space::init(env.path(), None).expect("Failed to initialize space");

        assert!(
            Space::exists(env.path()),
            "Directory should be a space after init"
        );
    }

    #[test]
    fn test_space_load_after_init() {
        let env = TestEnv::new();
        let name = "Test Space".to_string();

        Space::init(env.path(), Some(&name)).expect("Failed to initialize space");

        let loaded_space = Space::load(env.path()).expect("Failed to load space");

        assert_eq!(loaded_space.name(), "Test Space");
        assert_eq!(loaded_space.path(), env.path());
    }

    #[test]
    fn test_space_add_creates_journal_entry() {
        let env = TestEnv::new();

        let mut space = Space::init(env.path(), None).expect("Failed to initialize space");

        space
            .add("Test note content")
            .expect("Failed to add content");

        // Verify a journal file was created
        let journal_dir = env.path().join("journal");
        let entries: Vec<_> = fs::read_dir(&journal_dir)
            .expect("Failed to read journal directory")
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
            .collect();

        assert!(
            !entries.is_empty(),
            "Should have at least one journal entry"
        );
    }
}

mod config_tests {
    use super::*;
    use flow_core::{Config, Space};

    // Note: Config tests that involve loading/saving are tricky because
    // confy uses system paths. These tests focus on the in-memory behavior.

    #[test]
    fn test_config_default_has_no_spaces() {
        let config = Config::default();

        assert_eq!(config.space_count(), 0);
        assert!(config.get_active_space().is_none());
    }

    #[test]
    fn test_space_count_alias_matches_graph_count() {
        let config = Config::default();

        assert_eq!(config.space_count(), config.graph_count());
    }

    #[test]
    fn test_all_spaces_alias_matches_all_graphs() {
        let config = Config::default();

        assert_eq!(config.all_spaces().len(), config.all_graphs().len());
    }
}

mod graph_alias_tests {
    use super::*;
    use flow_core::{graph::Graph, Space};

    #[test]
    fn test_graph_is_alias_for_space() {
        let env = TestEnv::new();

        // Initialize using Graph (which is Space)
        let graph = Graph::init(env.path(), None).expect("Failed to initialize graph");

        // Load using Space
        let space = Space::load(env.path()).expect("Failed to load space");

        // They should have the same name and path
        assert_eq!(graph.name(), space.name());
        assert_eq!(graph.path(), space.path());
    }

    #[test]
    fn test_graph_exists_matches_space_exists() {
        let env = TestEnv::new();

        assert_eq!(Graph::exists(env.path()), Space::exists(env.path()));

        Graph::init(env.path(), None).expect("Failed to initialize");

        assert_eq!(Graph::exists(env.path()), Space::exists(env.path()));
    }
}
