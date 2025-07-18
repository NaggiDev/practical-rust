use std::fs;
use std::path::Path;
use std::io::Write;
use tempfile::tempdir;
use assert_cmd::Command;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("file-explorer").unwrap();
    
    // Simulate sending "help" and then "exit" commands
    let mut child = cmd.write_stdin("help\nexit\n").spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    
    // Check that the help text is in the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available commands:"));
    assert!(stdout.contains("ls"));
    assert!(stdout.contains("cd <directory>"));
    assert!(stdout.contains("exit, quit"));
}

#[test]
fn test_pwd_command() {
    let mut cmd = Command::cargo_bin("file-explorer").unwrap();
    
    // Simulate sending "pwd" and then "exit" commands
    let mut child = cmd.write_stdin("pwd\nexit\n").spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    
    // Check that the current directory is displayed
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pwd"));
}

#[test]
fn test_ls_command() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // Create some test files and directories
    fs::create_dir(temp_path.join("test_dir")).unwrap();
    let mut test_file = fs::File::create(temp_path.join("test_file.txt")).unwrap();
    test_file.write_all(b"test content").unwrap();
    
    let mut cmd = Command::cargo_bin("file-explorer").unwrap();
    cmd.current_dir(temp_path);
    
    // Simulate sending "ls" and then "exit" commands
    let mut child = cmd.write_stdin("ls\nexit\n").spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    
    // Check that the directory contents are listed
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Contents of"));
}

#[test]
fn test_cd_command() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // Create a subdirectory
    fs::create_dir(temp_path.join("subdir")).unwrap();
    
    let mut cmd = Command::cargo_bin("file-explorer").unwrap();
    cmd.current_dir(temp_path);
    
    // Simulate changing directory and then exiting
    let mut child = cmd.write_stdin("cd subdir\npwd\ncd ..\npwd\nexit\n").spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    
    // Check that the directory was changed
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("subdir"));
}

#[test]
fn test_cat_command() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // Create a test file with content
    let test_content = "This is test content for the cat command";
    let file_path = temp_path.join("test_file.txt");
    let mut test_file = fs::File::create(&file_path).unwrap();
    test_file.write_all(test_content.as_bytes()).unwrap();
    
    let mut cmd = Command::cargo_bin("file-explorer").unwrap();
    cmd.current_dir(temp_path);
    
    // Simulate cat command and then exit
    let mut child = cmd.write_stdin("cat test_file.txt\nexit\n").spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    
    // Check that the file content is displayed
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test_file.txt"));
}

#[test]
fn test_mkdir_command() {
    // Create a temporary directory for testing
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    let mut cmd = Command::cargo_bin("file-explorer").unwrap();
    cmd.current_dir(temp_path);
    
    // Simulate mkdir command and then exit
    let mut child = cmd.write_stdin("mkdir new_directory\nls\nexit\n").spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    
    // Check that the directory was created
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("new_directory"));
    assert!(Path::new(&temp_path.join("new_directory")).exists());
}