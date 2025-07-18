# RustyNotes Extension Challenges

After completing the basic implementation of RustyNotes, challenge yourself with these extension projects to deepen your understanding of Rust concepts and enhance the application's functionality.

## Extension 1: Markdown Support

**Objective:** Add support for formatting notes with Markdown.

**Tasks:**
1. Add a dependency on a Markdown parsing crate like `pulldown-cmark`:
   ```toml
   [dependencies]
   pulldown-cmark = "0.9"
   ```

2. Implement a function to render Markdown content:
   ```rust
   use pulldown_cmark::{Parser, Options, html};

   fn render_markdown(markdown: &str) -> String {
       let mut options = Options::empty();
       options.insert(Options::ENABLE_STRIKETHROUGH);
       options.insert(Options::ENABLE_TABLES);
       
       let parser = Parser::new_ext(markdown, options);
       let mut html_output = String::new();
       html::push_html(&mut html_output, parser);
       
       html_output
   }
   ```

3. Add a command to view notes with formatted Markdown:
   ```rust
   "view-md" => {
       if parts.len() < 2 {
           println!("Usage: view-md <note_id>");
           continue;
       }
       
       let id = parts[1];
       match app.get_note(id) {
           Ok(note) => {
               println!("Title: {}", note.title);
               println!("Tags: {}", note.tags.join(", "));
               if let Some(category) = &note.category {
                   println!("Category: {}", category);
               }
               println!("\nContent (Markdown):");
               println!("{}", render_markdown(&note.content));
           },
           Err(e) => println!("Error: {}", e),
       }
   }
   ```

4. Implement a simple terminal Markdown renderer if HTML output isn't suitable.

**Concepts Applied:**
- External crate integration
- String processing
- Function composition
- Command pattern extension

## Extension 2: Note Encryption

**Objective:** Implement encryption for sensitive notes.

**Tasks:**
1. Add dependencies on encryption crates:
   ```toml
   [dependencies]
   aes-gcm = "0.10"
   rand = "0.8"
   base64 = "0.13"
   sha2 = "0.10"
   ```

2. Extend the Note struct to support encryption:
   ```rust
   pub struct Note {
       // Existing fields
       pub encrypted: bool,
       pub salt: Option<Vec<u8>>,
   }
   ```

3. Implement encryption and decryption functions:
   ```rust
   use aes_gcm::{Aes256Gcm, Key, Nonce};
   use aes_gcm::aead::{Aead, NewAead};
   use rand::Rng;
   use sha2::{Sha256, Digest};
   
   fn encrypt_content(content: &str, password: &str) -> Result<(Vec<u8>, Vec<u8>), NoteError> {
       // Generate a random salt
       let mut salt = [0u8; 16];
       rand::thread_rng().fill(&mut salt);
       
       // Derive key from password and salt
       let mut hasher = Sha256::new();
       hasher.update(password.as_bytes());
       hasher.update(&salt);
       let key = hasher.finalize();
       
       // Encrypt the content
       let cipher = Aes256Gcm::new(Key::from_slice(&key));
       let nonce = Nonce::from_slice(b"unique nonce"); // In a real app, use a proper nonce
       
       let ciphertext = cipher.encrypt(nonce, content.as_bytes())
           .map_err(|e| NoteError::Encryption(e.to_string()))?;
       
       Ok((ciphertext, salt.to_vec()))
   }
   
   fn decrypt_content(ciphertext: &[u8], password: &str, salt: &[u8]) -> Result<String, NoteError> {
       // Derive key from password and salt
       let mut hasher = Sha256::new();
       hasher.update(password.as_bytes());
       hasher.update(salt);
       let key = hasher.finalize();
       
       // Decrypt the content
       let cipher = Aes256Gcm::new(Key::from_slice(&key));
       let nonce = Nonce::from_slice(b"unique nonce"); // Must match the one used for encryption
       
       let plaintext = cipher.decrypt(nonce, ciphertext)
           .map_err(|e| NoteError::Encryption(e.to_string()))?;
       
       String::from_utf8(plaintext)
           .map_err(|e| NoteError::Encryption(e.to_string()))
   }
   ```

4. Add commands to create and view encrypted notes:
   ```rust
   "add-encrypted" => {
       // Similar to add but with password prompt
       println!("Enter password for encryption:");
       let password = read_password()?;
       
       // Rest of the add command with encryption
   }
   
   "view-encrypted" => {
       // Similar to view but with password prompt
       println!("Enter password for decryption:");
       let password = read_password()?;
       
       // Rest of the view command with decryption
   }
   ```

**Concepts Applied:**
- Cryptography basics
- Secure password handling
- Binary data manipulation
- Error handling for cryptographic operations

## Extension 3: Backup and Restore

**Objective:** Create a simple backup and restore system.

**Tasks:**
1. Implement a command to export notes to a backup file:
   ```rust
   "backup" => {
       if parts.len() < 2 {
           println!("Usage: backup <file_path>");
           continue;
       }
       
       let path = parts[1];
       match app.backup_notes(path) {
           Ok(_) => println!("Notes backed up successfully to {}", path),
           Err(e) => println!("Error backing up notes: {}", e),
       }
   }
   ```

2. Implement the backup function in the App struct:
   ```rust
   pub fn backup_notes(&self, path: &str) -> Result<(), NoteError> {
       let notes = self.storage.list_notes();
       let json = serde_json::to_string_pretty(&notes)?;
       
       let mut file = File::create(path)?;
       file.write_all(json.as_bytes())?;
       
       Ok(())
   }
   ```

3. Implement a command to restore notes from a backup:
   ```rust
   "restore" => {
       if parts.len() < 2 {
           println!("Usage: restore <file_path> [--merge|--replace]");
           continue;
       }
       
       let path = parts[1];
       let mode = if parts.len() > 2 && parts[2] == "--replace" {
           RestoreMode::Replace
       } else {
           RestoreMode::Merge
       };
       
       match app.restore_notes(path, mode) {
           Ok(count) => println!("Restored {} notes from {}", count, path),
           Err(e) => println!("Error restoring notes: {}", e),
       }
   }
   ```

4. Implement the restore function with merge and replace options:
   ```rust
   pub enum RestoreMode {
       Merge,
       Replace,
   }
   
   pub fn restore_notes(&mut self, path: &str, mode: RestoreMode) -> Result<usize, NoteError> {
       let mut file = File::open(path)?;
       let mut contents = String::new();
       file.read_to_string(&mut contents)?;
       
       let backup_notes: Vec<Note> = serde_json::from_str(&contents)?;
       
       if let RestoreMode::Replace = mode {
           self.storage.clear()?;
       }
       
       let mut count = 0;
       for note in backup_notes {
           self.storage.add_note(note)?;
           count += 1;
       }
       
       Ok(count)
   }
   ```

**Concepts Applied:**
- File I/O for different purposes
- Serialization and deserialization
- Error handling for file operations
- Command-line argument parsing

## Extension 4: Text User Interface (TUI)

**Objective:** Add a simple TUI for a more interactive experience.

**Tasks:**
1. Add a dependency on a TUI crate:
   ```toml
   [dependencies]
   cursive = "0.17"
   ```

2. Create a new file `src/tui.rs` for the TUI implementation:
   ```rust
   use cursive::Cursive;
   use cursive::views::{Dialog, EditView, ListView, SelectView, TextView};
   use cursive::traits::*;
   use crate::app::App;
   use crate::note::Note;
   use std::rc::Rc;
   use std::cell::RefCell;
   
   pub fn run_tui(app: App) {
       let app = Rc::new(RefCell::new(app));
       
       let mut siv = cursive::default();
       
       // Set up the main menu
       siv.add_layer(
           Dialog::new()
               .title("RustyNotes")
               .content(
                   ListView::new()
                       .child("List Notes", |s| show_notes_list(s, Rc::clone(&app)))
                       .child("Add Note", |s| show_add_note_dialog(s, Rc::clone(&app)))
                       .child("Search Notes", |s| show_search_dialog(s, Rc::clone(&app)))
               )
               .button("Quit", |s| s.quit())
       );
       
       siv.run();
   }
   
   fn show_notes_list(s: &mut Cursive, app: Rc<RefCell<App>>) {
       let notes = app.borrow().list_notes();
       
       let mut select = SelectView::new();
       for note in notes {
           select.add_item(format!("{}: {}", note.id, note.title), note.id.clone());
       }
       
       select.set_on_submit(move |s, id: &String| {
           show_note_details(s, Rc::clone(&app), id.clone());
       });
       
       s.add_layer(
           Dialog::new()
               .title("Notes")
               .content(select)
               .button("Back", |s| { s.pop_layer(); })
       );
   }
   
   // Implement other TUI functions...
   ```

3. Update `main.rs` to support both CLI and TUI modes:
   ```rust
   fn main() -> Result<(), NoteError> {
       let args: Vec<String> = std::env::args().collect();
       
       let app = App::new(None)?;
       
       if args.len() > 1 && args[1] == "--tui" {
           // Run in TUI mode
           tui::run_tui(app);
           Ok(())
       } else {
           // Run in CLI mode (existing implementation)
           run_cli(app)
       }
   }
   ```

**Concepts Applied:**
- External crate integration for UI
- Event-driven programming
- Shared ownership with Rc and RefCell
- Separation of UI and business logic

## Extension 5: Note Templates

**Objective:** Add support for note templates to streamline creation of similar notes.

**Tasks:**
1. Implement a template system:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct Template {
       pub name: String,
       pub title_template: String,
       pub content_template: String,
       pub tags: Vec<String>,
       pub category: Option<String>,
   }
   
   impl Template {
       pub fn new(name: String, title_template: String, content_template: String, 
                 tags: Vec<String>, category: Option<String>) -> Self {
           Self {
               name,
               title_template,
               content_template,
               tags,
               category,
           }
       }
       
       pub fn create_note(&self, variables: &HashMap<String, String>) -> Note {
           let title = replace_variables(&self.title_template, variables);
           let content = replace_variables(&self.content_template, variables);
           
           Note::new(title, content, self.tags.clone(), self.category.clone())
       }
   }
   
   fn replace_variables(template: &str, variables: &HashMap<String, String>) -> String {
       let mut result = template.to_string();
       
       for (key, value) in variables {
           let placeholder = format!("{{{}}}", key);
           result = result.replace(&placeholder, value);
       }
       
       result
   }
   ```

2. Add template management to the App struct:
   ```rust
   pub fn add_template(&mut self, template: Template) -> Result<(), NoteError> {
       self.templates.insert(template.name.clone(), template);
       self.save_templates()?;
       Ok(())
   }
   
   pub fn get_template(&self, name: &str) -> Option<&Template> {
       self.templates.get(name)
   }
   
   pub fn list_templates(&self) -> Vec<&Template> {
       self.templates.values().collect()
   }
   
   pub fn create_note_from_template(&mut self, template_name: &str, 
                                   variables: HashMap<String, String>) -> Result<String, NoteError> {
       let template = self.get_template(template_name)
           .ok_or_else(|| NoteError::NotFound(format!("Template not found: {}", template_name)))?;
           
       let note = template.create_note(&variables);
       let id = note.id.clone();
       
       self.storage.add_note(note)?;
       
       Ok(id)
   }
   ```

3. Add commands for template management:
   ```rust
   "add-template" => {
       println!("Enter template name:");
       let name = read_line()?;
       
       println!("Enter title template (use {variable} for placeholders):");
       let title_template = read_line()?;
       
       println!("Enter content template (use {variable} for placeholders):");
       let content_template = read_line()?;
       
       // Rest of the command implementation
   }
   
   "use-template" => {
       if parts.len() < 2 {
           println!("Usage: use-template <template_name>");
           continue;
       }
       
       let template_name = parts[1];
       
       // Get the template
       let template = match app.get_template(template_name) {
           Some(t) => t,
           None => {
               println!("Template not found: {}", template_name);
               continue;
           }
       };
       
       // Collect variables
       let mut variables = HashMap::new();
       
       // Extract variables from template and prompt for values
       // Implementation details...
       
       // Create note from template
       match app.create_note_from_template(template_name, variables) {
           Ok(id) => println!("Note created with ID: {}", id),
           Err(e) => println!("Error: {}", e),
       }
   }
   ```

**Concepts Applied:**
- String templating and variable substitution
- HashMap for variable storage
- Complex data structures
- User input processing

Each of these extensions builds on the core RustyNotes application and introduces new Rust concepts and patterns. Choose the extensions that interest you most, or implement all of them for a comprehensive learning experience!