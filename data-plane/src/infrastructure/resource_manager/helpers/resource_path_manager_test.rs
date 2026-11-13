use crate::infrastructure::resource_manager::helpers::resource_path_manager::ResourcePathManager;

fn manager(root: &str) -> ResourcePathManager {
    ResourcePathManager {
        root: root.to_string(),
    }
}

#[test]
fn get_path_returns_job_resource_path() {
    assert_eq!(
        manager("/var/lib/gt-cloud/resources").get_path("job-123"),
        "/var/lib/gt-cloud/resources/job-123"
    );
}

#[test]
fn get_path_avoids_duplicate_separator() {
    assert_eq!(
        manager("/var/lib/gt-cloud/resources/").get_path("job-123"),
        "/var/lib/gt-cloud/resources/job-123"
    );
}

#[test]
fn get_path_supports_filesystem_root() {
    assert_eq!(manager("/").get_path("job-123"), "/job-123");
}

#[test]
fn get_path_is_deterministic() {
    let path_manager = manager("/resources");

    assert_eq!(
        path_manager.get_path("job-123"),
        path_manager.get_path("job-123")
    );
}
