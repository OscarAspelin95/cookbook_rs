mod schema;
use schema::User;
use tempfile;

fn main() {
    let user = User::mock();

    // Temporary file
    let tmpdir = tempfile::TempDir::new().expect("Failed to create temp dir.");
    let file_path = tmpdir.path().join("test.json");

    // Write to json.
    User::write(&user, &file_path);
    assert!(file_path.exists());
    println!("Wrote user instance to {}", file_path.display());

    // Read json.
    let user_from_json = User::read(&file_path);
    println!("Successfully read json into {:?}", user_from_json);
}
