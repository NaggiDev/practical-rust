// Quiz Framework for Rust Learning Path
// This module provides interactive quizzes for testing concept understanding

use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

/// Types of quiz questions
#[derive(Debug, Clone)]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    CodeCompletion,
    CodeOutput,
    ConceptExplanation,
}

/// A single quiz question
#[derive(Debug, Clone)]
pub struct Question {
    pub id: String,
    pub question_type: QuestionType,
    pub concept: String,
    pub difficulty: Difficulty,
    pub question_text: String,
    pub options: Vec<String>,
    pub correct_answer: String,
    pub explanation: String,
    pub code_snippet: Option<String>,
}

/// Difficulty levels for questions
#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Beginner => write!(f, "Beginner"),
            Difficulty::Intermediate => write!(f, "Intermediate"),
            Difficulty::Advanced => write!(f, "Advanced"),
            Difficulty::Expert => write!(f, "Expert"),
        }
    }
}

/// Quiz result for a single question
#[derive(Debug, Clone)]
pub struct QuizResult {
    pub question_id: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub is_correct: bool,
    pub time_taken_seconds: u64,
}

/// Quiz session containing multiple questions
#[derive(Debug)]
pub struct Quiz {
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
    pub results: Vec<QuizResult>,
    pub current_question: usize,
}

impl Quiz {
    pub fn new(title: String, description: String) -> Self {
        Self {
            title,
            description,
            questions: Vec::new(),
            results: Vec::new(),
            current_question: 0,
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }

    pub fn start_interactive(&mut self) -> QuizSummary {
        println!("\n🎯 {}", self.title);
        println!("📝 {}", self.description);
        println!("📊 {} questions total\n", self.questions.len());

        for (index, question) in self.questions.iter().enumerate() {
            self.current_question = index;
            let result = self.ask_question(question, index + 1);
            self.results.push(result);
        }

        self.generate_summary()
    }

    fn ask_question(&self, question: &Question, question_num: usize) -> QuizResult {
        let start_time = std::time::Instant::now();
        
        println!("Question {}/{}", question_num, self.questions.len());
        println!("Concept: {} | Difficulty: {}", question.concept, question.difficulty);
        println!("─────────────────────────────────────────");
        
        if let Some(code) = &question.code_snippet {
            println!("Code:");
            println!("```rust");
            println!("{}", code);
            println!("```\n");
        }
        
        println!("{}\n", question.question_text);
        
        match question.question_type {
            QuestionType::MultipleChoice => {
                for (i, option) in question.options.iter().enumerate() {
                    println!("{}. {}", (b'A' + i as u8) as char, option);
                }
                println!();
            }
            QuestionType::TrueFalse => {
                println!("A. True");
                println!("B. False\n");
            }
            _ => {}
        }
        
        let user_answer = loop {
            print!("Your answer: ");
            if io::stdout().flush().is_err() {
                eprintln!("Warning: Could not flush output");
            }
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let answer = input.trim().to_uppercase();
                    if self.validate_answer_format(&answer, &question.question_type) {
                        break answer;
                    } else {
                        println!("Invalid answer format. Please try again.");
                        continue;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}. Please try again.", e);
                    continue;
                }
            }
        };
        
        let time_taken = start_time.elapsed().as_secs();
        let is_correct = self.check_answer(&user_answer, &question.correct_answer);
        
        if is_correct {
            println!("✅ Correct!");
        } else {
            println!("❌ Incorrect.");
            println!("The correct answer is: {}", question.correct_answer);
        }
        
        println!("💡 Explanation: {}\n", question.explanation);
        println!("Press Enter to continue...");
        let mut _continue = String::new();
        if io::stdin().read_line(&mut _continue).is_err() {
            eprintln!("Warning: Could not read continuation input");
        }
        
        QuizResult {
            question_id: question.id.clone(),
            user_answer,
            correct_answer: question.correct_answer.clone(),
            is_correct,
            time_taken_seconds: time_taken,
        }
    }

    fn validate_answer_format(&self, answer: &str, question_type: &QuestionType) -> bool {
        match question_type {
            QuestionType::MultipleChoice => {
                matches!(answer, "A" | "B" | "C" | "D")
            }
            QuestionType::TrueFalse => {
                matches!(answer, "A" | "B" | "TRUE" | "FALSE")
            }
            _ => !answer.is_empty()
        }
    }

    fn check_answer(&self, user_answer: &str, correct_answer: &str) -> bool {
        user_answer.trim().to_uppercase() == correct_answer.trim().to_uppercase()
    }

    fn generate_summary(&self) -> QuizSummary {
        let total_questions = self.questions.len();
        let correct_answers = self.results.iter().filter(|r| r.is_correct).count();
        let total_time: u64 = self.results.iter().map(|r| r.time_taken_seconds).sum();
        
        QuizSummary {
            quiz_title: self.title.clone(),
            total_questions,
            correct_answers,
            incorrect_answers: total_questions - correct_answers,
            score_percentage: if total_questions > 0 { 
                (correct_answers as f64 / total_questions as f64) * 100.0 
            } else { 
                0.0 
            },
            total_time_seconds: total_time,
            average_time_per_question: if total_questions > 0 { 
                total_time as f64 / total_questions as f64 
            } else { 
                0.0 
            },
        }
    }
}

/// Summary of quiz performance
#[derive(Debug)]
pub struct QuizSummary {
    pub quiz_title: String,
    pub total_questions: usize,
    pub correct_answers: usize,
    pub incorrect_answers: usize,
    pub score_percentage: f64,
    pub total_time_seconds: u64,
    pub average_time_per_question: f64,
}

impl fmt::Display for QuizSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n🎯 Quiz Results: {}", self.quiz_title)?;
        writeln!(f, "═══════════════════════════════════════")?;
        writeln!(f, "📊 Score: {}/{} ({:.1}%)", 
                 self.correct_answers, self.total_questions, self.score_percentage)?;
        writeln!(f, "✅ Correct: {}", self.correct_answers)?;
        writeln!(f, "❌ Incorrect: {}", self.incorrect_answers)?;
        writeln!(f, "⏱️  Total time: {}s", self.total_time_seconds)?;
        writeln!(f, "⏱️  Average per question: {:.1}s", self.average_time_per_question)?;
        
        let grade = match self.score_percentage {
            p if p >= 90.0 => "🏆 Excellent! You've mastered these concepts!",
            p if p >= 80.0 => "🎉 Great job! You have a solid understanding!",
            p if p >= 70.0 => "👍 Good work! Review the missed concepts.",
            p if p >= 60.0 => "📚 Keep studying! You're making progress.",
            _ => "📖 More practice needed. Review the concepts and try again.",
        };
        
        writeln!(f, "\n{}", grade)?;
        
        if self.score_percentage < 80.0 {
            writeln!(f, "\n💡 Tip: Review the explanations for incorrect answers and practice more!")?;
        }
        
        Ok(())
    }
}

/// Quiz bank containing all available quizzes
pub struct QuizBank {
    quizzes: HashMap<String, Quiz>,
}

impl QuizBank {
    pub fn new() -> Self {
        let mut bank = Self {
            quizzes: HashMap::new(),
        };
        bank.initialize_quizzes();
        bank
    }

    pub fn get_quiz(&self, quiz_id: &str) -> Option<&Quiz> {
        self.quizzes.get(quiz_id)
    }

    pub fn get_quiz_mut(&mut self, quiz_id: &str) -> Option<&mut Quiz> {
        self.quizzes.get_mut(quiz_id)
    }

    pub fn list_available_quizzes(&self) -> Vec<&String> {
        self.quizzes.keys().collect()
    }

    pub fn run_quiz(&mut self, quiz_id: &str) -> Result<QuizSummary, String> {
        match self.quizzes.get_mut(quiz_id) {
            Some(quiz) => Ok(quiz.start_interactive()),
            None => Err(format!("Quiz '{}' not found. Available quizzes: {:?}", 
                               quiz_id, self.list_available_quizzes()))
        }
    }

    fn initialize_quizzes(&mut self) {
        // Initialize all quizzes with questions
        self.create_basic_level_quizzes();
        self.create_intermediate_level_quizzes();
        self.create_advanced_level_quizzes();
        self.create_expert_level_quizzes();
    }

    fn create_basic_level_quizzes(&mut self) {
        // Basic Variables and Data Types Quiz
        let mut basic_vars_quiz = Quiz::new(
            "Basic Variables and Data Types".to_string(),
            "Test your understanding of Rust variables, mutability, and basic data types".to_string(),
        );

        basic_vars_quiz.add_question(Question {
            id: "basic_var_1".to_string(),
            question_type: QuestionType::MultipleChoice,
            concept: "Variables".to_string(),
            difficulty: Difficulty::Beginner,
            question_text: "Which keyword is used to declare a mutable variable in Rust?".to_string(),
            options: vec![
                "var".to_string(),
                "let".to_string(),
                "mut".to_string(),
                "let mut".to_string(),
            ],
            correct_answer: "D".to_string(),
            explanation: "In Rust, variables are immutable by default. To make them mutable, you use 'let mut variable_name'.".to_string(),
            code_snippet: None,
        });

        basic_vars_quiz.add_question(Question {
            id: "basic_var_2".to_string(),
            question_type: QuestionType::CodeOutput,
            concept: "Variables".to_string(),
            difficulty: Difficulty::Beginner,
            question_text: "What will this code print?".to_string(),
            options: vec![
                "5".to_string(),
                "10".to_string(),
                "Compilation error".to_string(),
                "Runtime error".to_string(),
            ],
            correct_answer: "C".to_string(),
            explanation: "This code will not compile because 'x' is immutable by default, and we're trying to reassign it.".to_string(),
            code_snippet: Some("let x = 5;\nx = 10;\nprintln!(\"{}\", x);".to_string()),
        });

        basic_vars_quiz.add_question(Question {
            id: "basic_types_1".to_string(),
            question_type: QuestionType::MultipleChoice,
            concept: "Data Types".to_string(),
            difficulty: Difficulty::Beginner,
            question_text: "What is the default integer type in Rust?".to_string(),
            options: vec![
                "i32".to_string(),
                "i64".to_string(),
                "u32".to_string(),
                "isize".to_string(),
            ],
            correct_answer: "A".to_string(),
            explanation: "The default integer type in Rust is i32, which is a 32-bit signed integer.".to_string(),
            code_snippet: None,
        });

        self.quizzes.insert("basic_variables".to_string(), basic_vars_quiz);

        // Basic Ownership Quiz
        let mut basic_ownership_quiz = Quiz::new(
            "Basic Ownership Concepts".to_string(),
            "Test your understanding of Rust's ownership system".to_string(),
        );

        basic_ownership_quiz.add_question(Question {
            id: "ownership_1".to_string(),
            question_type: QuestionType::TrueFalse,
            concept: "Ownership".to_string(),
            difficulty: Difficulty::Beginner,
            question_text: "In Rust, each value has exactly one owner at any given time.".to_string(),
            options: vec!["True".to_string(), "False".to_string()],
            correct_answer: "A".to_string(),
            explanation: "True. This is one of Rust's fundamental ownership rules - each value has exactly one owner.".to_string(),
            code_snippet: None,
        });

        basic_ownership_quiz.add_question(Question {
            id: "ownership_2".to_string(),
            question_type: QuestionType::CodeOutput,
            concept: "Ownership".to_string(),
            difficulty: Difficulty::Beginner,
            question_text: "What happens when this code is compiled?".to_string(),
            options: vec![
                "Prints: Hello".to_string(),
                "Compilation error".to_string(),
                "Runtime error".to_string(),
                "Prints nothing".to_string(),
            ],
            correct_answer: "B".to_string(),
            explanation: "This code will not compile because 's1' is moved to 's2', and then we try to use 's1' again, which is not allowed.".to_string(),
            code_snippet: Some("let s1 = String::from(\"Hello\");\nlet s2 = s1;\nprintln!(\"{}\", s1);".to_string()),
        });

        self.quizzes.insert("basic_ownership".to_string(), basic_ownership_quiz);
    }

    fn create_intermediate_level_quizzes(&mut self) {
        // Structs and Enums Quiz
        let mut structs_enums_quiz = Quiz::new(
            "Structs and Enums".to_string(),
            "Test your understanding of Rust structs and enums".to_string(),
        );

        structs_enums_quiz.add_question(Question {
            id: "struct_1".to_string(),
            question_type: QuestionType::MultipleChoice,
            concept: "Structs".to_string(),
            difficulty: Difficulty::Intermediate,
            question_text: "Which syntax is used to create an instance of a struct?".to_string(),
            options: vec![
                "new Person()".to_string(),
                "Person::new()".to_string(),
                "Person { name: \"Alice\", age: 30 }".to_string(),
                "struct Person(\"Alice\", 30)".to_string(),
            ],
            correct_answer: "C".to_string(),
            explanation: "Struct instances are created using the struct name followed by curly braces containing field values.".to_string(),
            code_snippet: Some("struct Person {\n    name: String,\n    age: u32,\n}".to_string()),
        });

        structs_enums_quiz.add_question(Question {
            id: "enum_1".to_string(),
            question_type: QuestionType::MultipleChoice,
            concept: "Enums".to_string(),
            difficulty: Difficulty::Intermediate,
            question_text: "What is the correct way to match on an enum variant?".to_string(),
            options: vec![
                "if color == Color::Red".to_string(),
                "match color { Color::Red => ... }".to_string(),
                "switch(color) { case Red: ... }".to_string(),
                "color.match(Red => ...)".to_string(),
            ],
            correct_answer: "B".to_string(),
            explanation: "Pattern matching with 'match' is the idiomatic way to handle enum variants in Rust.".to_string(),
            code_snippet: Some("enum Color {\n    Red,\n    Green,\n    Blue,\n}".to_string()),
        });

        self.quizzes.insert("structs_enums".to_string(), structs_enums_quiz);

        // Traits Quiz
        let mut traits_quiz = Quiz::new(
            "Traits and Implementations".to_string(),
            "Test your understanding of Rust traits".to_string(),
        );

        traits_quiz.add_question(Question {
            id: "trait_1".to_string(),
            question_type: QuestionType::TrueFalse,
            concept: "Traits".to_string(),
            difficulty: Difficulty::Intermediate,
            question_text: "A trait can have default implementations for its methods.".to_string(),
            options: vec!["True".to_string(), "False".to_string()],
            correct_answer: "A".to_string(),
            explanation: "True. Traits can provide default implementations that can be overridden by implementing types.".to_string(),
            code_snippet: None,
        });

        self.quizzes.insert("traits".to_string(), traits_quiz);
    }

    fn create_advanced_level_quizzes(&mut self) {
        // Concurrency Quiz
        let mut concurrency_quiz = Quiz::new(
            "Concurrency and Threading".to_string(),
            "Test your understanding of Rust's concurrency features".to_string(),
        );

        concurrency_quiz.add_question(Question {
            id: "concurrency_1".to_string(),
            question_type: QuestionType::MultipleChoice,
            concept: "Concurrency".to_string(),
            difficulty: Difficulty::Advanced,
            question_text: "Which type is used to share data safely between threads?".to_string(),
            options: vec![
                "Rc<T>".to_string(),
                "Arc<T>".to_string(),
                "Box<T>".to_string(),
                "RefCell<T>".to_string(),
            ],
            correct_answer: "B".to_string(),
            explanation: "Arc<T> (Atomically Reference Counted) is used for sharing data between threads safely.".to_string(),
            code_snippet: None,
        });

        self.quizzes.insert("concurrency".to_string(), concurrency_quiz);
    }

    fn create_expert_level_quizzes(&mut self) {
        // Async Programming Quiz
        let mut async_quiz = Quiz::new(
            "Async Programming".to_string(),
            "Test your understanding of Rust's async programming features".to_string(),
        );

        async_quiz.add_question(Question {
            id: "async_1".to_string(),
            question_type: QuestionType::MultipleChoice,
            concept: "Async".to_string(),
            difficulty: Difficulty::Expert,
            question_text: "What keyword is used to wait for an async operation to complete?".to_string(),
            options: vec![
                "wait".to_string(),
                "await".to_string(),
                "yield".to_string(),
                "async".to_string(),
            ],
            correct_answer: "B".to_string(),
            explanation: "The 'await' keyword is used to wait for async operations to complete.".to_string(),
            code_snippet: None,
        });

        self.quizzes.insert("async".to_string(), async_quiz);
    }
}

/// Interactive quiz runner
pub fn run_interactive_quiz_session() {
    let mut quiz_bank = QuizBank::new();
    
    loop {
        println!("\n🎯 Rust Learning Path - Concept Validation Quizzes");
        println!("═══════════════════════════════════════════════════");
        println!("Available quizzes:");
        
        let available_quizzes = quiz_bank.list_available_quizzes();
        for (i, quiz_id) in available_quizzes.iter().enumerate() {
            println!("{}. {}", i + 1, quiz_id.replace("_", " ").to_uppercase());
        }
        
        println!("{}. Exit", available_quizzes.len() + 1);
        
        print!("\nSelect a quiz (1-{}): ", available_quizzes.len() + 1);
        if io::stdout().flush().is_err() {
            eprintln!("Warning: Could not flush output");
        }
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<usize>() {
                    Ok(choice) => {
                        if choice == available_quizzes.len() + 1 {
                            println!("Thanks for using the quiz system! 👋");
                            break;
                        } else if choice > 0 && choice <= available_quizzes.len() {
                            let quiz_id = available_quizzes[choice - 1];
                            match quiz_bank.run_quiz(quiz_id) {
                                Ok(summary) => {
                                    println!("{}", summary);
                                    
                                    println!("\nPress Enter to return to the main menu...");
                                    let mut _continue = String::new();
                                    if io::stdin().read_line(&mut _continue).is_err() {
                                        eprintln!("Warning: Could not read continuation input");
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error running quiz: {}", e);
                                }
                            }
                        } else {
                            println!("Invalid choice. Please select a number between 1 and {}.", available_quizzes.len() + 1);
                        }
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}. Please try again.", e);
            }
        }
    }
}