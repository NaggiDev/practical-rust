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