use reqwest::blocking::Client;
use scraper::{Html,Selector} ;
use std::io::{self,Write};


fn main() -> Result<(), Box<dyn std::error::Error>>{

    print!("Enter the website URL "); // input URL 
    io::stdout().flush()?; 

    let mut url_input = String::new(); 
    io::stdin().read_line(&mut url_input)?; 
    let mut url = url_input.trim().to_string(); 

    if url.is_empty(){
        println!("No URL provided. Existing."); 
        return Ok(()); 
    }
    
    if !url.starts_with("http://") && !url.starts_with("https://"){
        url = format!("https://{}", url); 
        println!("Auto-corrected URL to: {}",url); 
    }

    print!("enter the css selector to search for (e.g., 'h1','.classes-name','#id'): ");
    io::stdout().flush()?; 

    let mut selector_input = String::new(); 
    io::stdin().read_line(&mut selector_input)?;
    let title_selector_str = selector_input.trim(); 

    if title_selector_str.is_empty(){
        println!("No selector provided. Exiting.");
        return Ok(()); 
    }

    println!("Please kindly wait...");
    println!("Fetching URL = {}", url);


    let client = Client::builder()
        .user_agent("My Rust web scrapper 1.0")
        .build()?; 
    
    let response_text = client.get(url).send()?.text()?;
    println!("successfully fetched the page !"); 

    let document = Html::parse_document(&response_text); 
    println!("HTML parsed successfully"); 
    
    let title_selector = Selector::parse(title_selector_str)
        .map_err(|e| format!("Failed to parse selector '{}': {:?}", title_selector_str, e))?;

    println!("Looking for elements matching selector: '{}'", title_selector_str);

    let mut found_elements = 0; 
    for element in document.select(&title_selector){
        let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string(); 
        if !text.is_empty(){
            println!("Found: {}",text); 
            found_elements += 1; 
        }
    }
    
    if found_elements == 0 {
        println!("No elements found matching the selector '{}'", title_selector_str);
    }else{
        println!("Total Element found: {}", found_elements);
    }

    Ok(())
}
