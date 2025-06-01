//mod LLM_runner;
mod LLM_run;

use std::env;
use dotenv::dotenv;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use std::io::{self, Write};
//use crate::LLM_run::LLMRunner;
use crate::LLM_run::{LLMRunner, ContentAnalysis};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    print!("Enter the URL: "); 
    io::stdout().flush()?;

    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input); 
    let mut url = url_input.trim().to_string();

    if url.is_empty(){
        println!("No URL provided. Existing "); 
        return Ok(()); 
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        url = format!("https://{}", url);
        println!("Auto-corrected URL: {}", url);
     }

     println!("Enter the CSS selector for the main content (e.g., 'article', '.content-body', '#main-text'): ");
     io::stdout().flush()?; 
     let mut selector_input = String::new();
     io::stdin().read_line(&mut selector_input);
     let content_selector_str = selector_input.trim(); 

     if content_selector_str.is_empty(){
        println!("No content selector provided. Existing ");
        return Ok(());
     }

     println!("Please Kindly wait ..."); 
     println!("Fetching URL: {}", url);

     let client = Client::builder()
        .user_agent("My Rust Web Scraper with LLM 1.0")
        .build()?; 

     let response_text = client.get(&url).send().await?.text().await?;
     println!("Successfully fetched URL: {}", url);

     let document = Html::parse_document(&response_text);
     println!("HTML parsed successfully");

     let title_selector = Selector::parse("title").unwrap(); 
     let page_title = document
        .select(&title_selector)
        .next()
        .map(|element| element.text().collect::<String>())
        .unwrap_or_else(|| "Unknown".to_string());

     println!("Page title: {}", page_title);

     let content_selector = Selector::parse(content_selector_str)
        .map_err(|e| format!("Failed to parse content selector '{}': {:?}", content_selector_str, e))?;
     println!("Looking for content elements matching selector: {}", content_selector_str); 

     let mut scrapper_content_parts = Vec::new();
     for element in document.select(&content_selector) {
         let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
         if !text.is_empty() {
             scrapper_content_parts.push(text);
         }
     }
     if scrapper_content_parts.is_empty(){
        println!("No content found matching selector: '{}'. Cannot perform LLm analysis on selected content", content_selector_str);
        return Ok(());
     }

     let combined_scrapped_content = scrapper_content_parts.join("\n\n ---- \n\n");
     println!("Total characters in selected content: {}", combined_scrapped_content.len()); 
     if combined_scrapped_content.len() > 500 {
        println!("Snipped of selected content: \n{}...", &combined_scrapped_content[..500]);
     }else{
        println!("Selected content: \n{}", combined_scrapped_content);
     }

    println!("\nInitializing LLM Runner...");
    match LLMRunner::new() {
        Ok(llm_runner) => {
            println!("LLM Runner initialized.");

            // 4. Perform LLM Analysis
            println!("\nRequesting LLM analysis for the scraped content...");
            match llm_runner.analyze_web_content(
                &page_title,
                &combined_scrapped_content,
                &url,
            ).await {
                Ok(analysis) => {
                    println!("\n--- LLM Content Analysis ---");
                    println!("URL: {}", url);
                    println!("Page Title: {}", page_title);
                    println!("\nSummary:\n{}", analysis.summary);
                    println!("\nSentiment:\n{}", analysis.sentiment);
                    println!("\nKey Topics:\n{}", analysis.key_topics);
                    println!("\nCategory:\n{}", analysis.category);
                    println!("--- End of Analysis ---");
                }
                Err(e) => {
                    eprintln!("\nError during LLM analysis: {}", e);
                }
            }

            if !combined_scrapped_content.is_empty() {
                let snippet_for_sentiment = if combined_scrapped_content.len() > 500 {
                    &combined_scrapped_content[..500]
                } else {
                    &combined_scrapped_content
                };
                println!("\nRequesting specific sentiment analysis for a snippet...");
                 match llm_runner.analyze_sentiment(snippet_for_sentiment).await {
                     Ok(sentiment_result) => {
                         println!("\n--- LLM Snippet Sentiment Analysis ---");
                         println!("Label: {}", sentiment_result.label);
                         println!("Confidence: {:.2}%", sentiment_result.confidence);
                         println!("Explanation: {}", sentiment_result.explanation);
                         println!("--- End of Snippet Sentiment Analysis ---");
                     }
                     Err(e) => {
                         eprintln!("\nError during LLM sentiment analysis: {}", e);
                     }
                 }
            }

        }
        Err(e) => {
            eprintln!("Failed to initialize LLM Runner: {}", e);
            eprintln!("Please ensure your GEMINI_API_KEY is set in a .env file or environment variables.");
        }
    }
    Ok(())
}
