// LLM_runner.rs - Handles all LLM API interactions

use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Gemini API request structures
#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}

#[derive(Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize)]
pub struct Part {
    pub text: String,
}

// Gemini API response structures
#[derive(Deserialize, Debug)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    pub content: ResponseContent,
}

#[derive(Deserialize, Debug)]
pub struct ResponseContent {
    pub parts: Vec<ResponsePart>,
}

#[derive(Deserialize, Debug)]
pub struct ResponsePart {
    pub text: String,
}

// Analysis result structures
#[derive(Debug, Clone)]
pub struct ContentAnalysis {
    pub summary: String,
    pub sentiment: String,
    pub key_topics: String,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct SentimentResult {
    pub label: String,
    pub confidence: f64,
    pub explanation: String,
}

// Main LLM runner struct
pub struct LLMRunner {
    client: Client,
    api_key: String,
    base_url: String,
}

impl LLMRunner {
    // Initialize the LLM runner
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = env::var("LLM_API_KEY")
            .map_err(|_| "LLM_API_KEY must be set in .env file")?;
        
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent".to_string(),
        })
    }

    // Generic method to send prompts to LLM
    pub async fn send_prompt(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}?key={}", self.base_url, self.api_key);
        
        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let gemini_response: GeminiResponse = response.json().await?;
            
            if let Some(candidate) = gemini_response.candidates.first() {
                if let Some(part) = candidate.content.parts.first() {
                    return Ok(part.text.clone());
                }
            }
        } else {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }
        
        Err("No response from LLM".into())
    }

    // Analyze web content comprehensively
    pub async fn analyze_web_content(
        &self, 
        title: &str, 
        content: &str, 
        url: &str
    ) -> Result<ContentAnalysis, Box<dyn std::error::Error>> {
        
        // Truncate content to avoid API limits (Gemini has token limits)
        let truncated_content = if content.len() > 3000 {
            &content[..3000]
        } else {
            content
        };

        let prompt = format!(
            "Analyze this web content and provide structured analysis:\n\n\
            URL: {}\n\
            Title: {}\n\
            Content: {}\n\n\
            Please provide analysis in this exact format:\n\
            SUMMARY: [2-3 sentence summary]\n\
            SENTIMENT: [POSITIVE/NEGATIVE/NEUTRAL with brief explanation]\n\
            TOPICS: [comma-separated key topics/themes]\n\
            CATEGORY: [main category like Technology, News, Business, Education, etc.]\n\n\
            Be concise and accurate.",
            url, title, truncated_content
        );

        let response = self.send_prompt(&prompt).await?;
        
        // Parse structured response
        let mut summary = String::new();
        let mut sentiment = String::new();
        let mut topics = String::new();
        let mut category = String::new();

        for line in response.lines() {
            let line = line.trim();
            if line.starts_with("SUMMARY:") {
                summary = line.replace("SUMMARY:", "").trim().to_string();
            } else if line.starts_with("SENTIMENT:") {
                sentiment = line.replace("SENTIMENT:", "").trim().to_string();
            } else if line.starts_with("TOPICS:") {
                topics = line.replace("TOPICS:", "").trim().to_string();
            } else if line.starts_with("CATEGORY:") {
                category = line.replace("CATEGORY:", "").trim().to_string();
            }
        }

        // Fallback to raw response if parsing fails
        if summary.is_empty() {
            summary = response.clone();
        }
        if sentiment.is_empty() {
            sentiment = "NEUTRAL".to_string();
        }
        if topics.is_empty() {
            topics = "General".to_string();
        }
        if category.is_empty() {
            category = "General".to_string();
        }

        Ok(ContentAnalysis {
            summary,
            sentiment,
            key_topics: topics,
            category,
        })
    }

    // Specific sentiment analysis
    pub async fn analyze_sentiment(&self, text: &str) -> Result<SentimentResult, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Analyze the sentiment of the following text. Return your analysis as a JSON object \
            with three keys: \"label\" (string: \"POSITIVE\", \"NEGATIVE\", or \"NEUTRAL\"), \
            \"confidence\" (float: a score between 0.0 and 1.0 indicating certainty), \
            and \"explanation\" (string: a brief explanation of the sentiment).\n\n\
            Text: \"{}\"",
            text
        );
        let llm_response_text = self.send_prompt(&prompt).await?;

        // Try to parse structured JSON response
        match serde_json::from_str::<ParsedSentimentLLMResponse>(&llm_response_text) {
            Ok(parsed) => Ok(SentimentResult {
                label: parsed.label,
                confidence: parsed.confidence * 100.0, // Assuming LLM gives 0.0-1.0
                explanation: parsed.explanation,
            }),
            Err(e) => {
                eprintln!("Failed to parse sentiment JSON from LLM: {}. Raw response: {}", e, llm_response_text);
                // Fallback to simpler parsing or default
                Ok(SentimentResult {
                    label: if llm_response_text.to_lowercase().contains("positive") {
                        "POSITIVE".to_string()
                    } else if llm_response_text.to_lowercase().contains("negative") {
                        "NEGATIVE".to_string()
                    } else {
                        "NEUTRAL".to_string()
                    },
                    confidence: 50.0,
                    explanation: llm_response_text, // Or a generic "Could not parse detailed sentiment"
                })
            }
        }
    }

    // Summarize content
    pub async fn summarize_content(&self, content: &str, max_sentences: u32) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Summarize the following content in exactly {} sentences. \
            Focus on the most important information:\n\n{}",
            max_sentences,
            if content.len() > 4000 { &content[..4000] } else { content }
        );

        self.send_prompt(&prompt).await
    }

    // Extract key topics/themes
    pub async fn extract_topics(&self, content: &str, max_topics: u32) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Extract the top {} key topics or themes from this content. \
            Return only the topics, one per line:\n\n{}",
            max_topics,
            if content.len() > 4000 { &content[..4000] } else { content }
        );

        let response = self.send_prompt(&prompt).await?;
        
        let topics: Vec<String> = response
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .take(max_topics as usize)
            .collect();

        Ok(topics)
    }

    // Classify content category
    pub async fn classify_content(&self, title: &str, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Classify this web content into one main category. \
            Choose from: Technology, News, Business, Education, Entertainment, \
            Sports, Health, Science, Politics, Lifestyle, Other\n\n\
            Title: {}\n\
            Content: {}\n\n\
            Return only the category name:",
            title,
            if content.len() > 2000 { &content[..2000] } else { content }
        );

        self.send_prompt(&prompt).await
    }

    // Check if content is relevant to specific keywords
    pub async fn check_relevance(&self, content: &str, keywords: &[&str]) -> Result<f64, Box<dyn std::error::Error>> {
        let keywords_str = keywords.join(", ");
        let prompt = format!(
            "Rate how relevant this content is to these keywords: {}\n\
            Content: {}\n\n\
            Provide a relevance score from 0-100 where:\n\
            0 = Not relevant at all\n\
            50 = Somewhat relevant\n\
            100 = Highly relevant\n\n\
            Return only the number:",
            keywords_str,
            if content.len() > 3000 { &content[..3000] } else { content }
        );

        let response = self.send_prompt(&prompt).await?;
        let score = response.trim().parse::<f64>().unwrap_or(0.0);
        Ok(score.min(100.0).max(0.0))
    }
}

// Helper functions
impl LLMRunner {
    // Test connection to LLM
    pub async fn test_connection(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let test_prompt = "Reply with 'OK' if you receive this message.";
        let response = self.send_prompt(test_prompt).await?;
        Ok(response.to_uppercase().contains("OK"))
    }

    // Get model info/status
    pub async fn get_model_info(&self) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = "What model are you and what are your capabilities?";
        self.send_prompt(prompt).await
    }
}