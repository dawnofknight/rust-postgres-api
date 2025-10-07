use spider::website::Website;
use spider::page::Page;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use url::Url;
use regex::Regex;
use chrono::{DateTime, NaiveDate};
use html2text;

// Helper function to parse date string to NaiveDate
fn parse_date_string(date_str: &str) -> Result<NaiveDate, CrawlerError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| CrawlerError::DateParsingError(format!("Invalid date format '{}': {}", date_str, e)))
}

// Helper function to validate date range
fn validate_date_range(date_from: Option<&String>, date_to: Option<&String>) -> Result<(Option<NaiveDate>, Option<NaiveDate>), CrawlerError> {
    let from_date = if let Some(from_str) = date_from {
        Some(parse_date_string(from_str)?)
    } else {
        None
    };
    
    let to_date = if let Some(to_str) = date_to {
        Some(parse_date_string(to_str)?)
    } else {
        None
    };
    
    // Validate that from_date is not after to_date
    if let (Some(from), Some(to)) = (from_date, to_date) {
        if from > to {
            return Err(CrawlerError::DateParsingError(
                "date_from cannot be after date_to".to_string()
            ));
        }
    }
    
    Ok((from_date, to_date))
}

// Helper function to extract date from page content using Spider's Page struct
fn extract_page_dates_from_spider_page(page: &Page) -> (Option<String>, Option<String>) {
    let html_content = page.get_html();
    
    // Use regex to extract meta tags for dates
    let meta_regex = Regex::new(r#"<meta[^>]*(?:property|name)="([^"]*)"[^>]*content="([^"]*)"[^>]*>"#).unwrap();
    let mut last_modified = None;
    let mut published_date = None;
    
    for cap in meta_regex.captures_iter(&html_content) {
        if let (Some(attr), Some(content)) = (cap.get(1), cap.get(2)) {
            let attr_value = attr.as_str();
            let content_value = content.as_str();
            
            match attr_value {
                "article:modified_time" | "article:updated_time" | "last-modified" | "date-modified" => {
                    last_modified = Some(content_value.to_string());
                }
                "article:published_time" | "date" | "publish-date" | "publication-date" => {
                    published_date = Some(content_value.to_string());
                }
                _ => {}
            }
        }
    }
    
    // Try to extract from time elements with datetime attribute
    let time_regex = Regex::new(r#"<time[^>]*datetime="([^"]*)"[^>]*>"#).unwrap();
    if published_date.is_none() {
        if let Some(cap) = time_regex.captures(&html_content) {
            if let Some(datetime) = cap.get(1) {
                published_date = Some(datetime.as_str().to_string());
            }
        }
    }
    
    (last_modified, published_date)
}

// Helper function to extract dates from page using proper Spider API
fn extract_dates_from_page(page: &Page) -> Vec<String> {
    let html = page.get_html();
    
    let mut dates = Vec::new();
    
    // Use regex to extract meta tags for dates
    let meta_regex = Regex::new(r#"<meta[^>]*(?:property|name)="([^"]*)"[^>]*content="([^"]*)"[^>]*>"#).unwrap();
    
    for cap in meta_regex.captures_iter(&html) {
        if let (Some(attr), Some(content)) = (cap.get(1), cap.get(2)) {
            let attr_value = attr.as_str();
            let content_value = content.as_str();
            
            if attr_value.contains("date") || attr_value.contains("time") {
                dates.push(content_value.to_string());
            }
        }
    }
    
    // Extract from time elements
    let time_regex = Regex::new(r#"<time[^>]*datetime="([^"]*)"[^>]*>"#).unwrap();
    for cap in time_regex.captures_iter(&html) {
        if let Some(datetime) = cap.get(1) {
            dates.push(datetime.as_str().to_string());
        }
    }
    
    dates
}

// Helper function to parse date from string
fn parse_date(date_str: &str) -> Option<NaiveDate> {
    DateTime::parse_from_rfc3339(date_str)
        .map(|dt| dt.naive_utc().date())
        .or_else(|_| NaiveDate::parse_from_str(date_str, "%Y-%m-%d"))
        .or_else(|_| NaiveDate::parse_from_str(date_str, "%Y/%m/%d"))
        .or_else(|_| NaiveDate::parse_from_str(date_str, "%m/%d/%Y"))
        .ok()
}

// Helper function to check if a page matches the date filter
fn matches_date_filter(
    page_dates: &[String],
    start_date: Option<&NaiveDate>,
    end_date: Option<&NaiveDate>,
) -> bool {
    if start_date.is_none() && end_date.is_none() {
        return true;
    }

    for date_str in page_dates {
        if let Some(parsed_date) = parse_date(date_str) {
            let date_matches = match (start_date, end_date) {
                (Some(start), Some(end)) => parsed_date >= *start && parsed_date <= *end,
                (Some(start), None) => parsed_date >= *start,
                (None, Some(end)) => parsed_date <= *end,
                (None, None) => true,
            };
            
            if date_matches {
                return true;
            }
        }
    }
    
    false
}

fn clean_html_text(html_text: &str) -> String {
    // Convert HTML to plain text
    let plain_text = html2text::from_read(html_text.as_bytes(), 120);
    
    // Clean up extra whitespace and normalize line breaks
    let cleaned = plain_text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    
    cleaned
}

fn calculate_relevance_score(keyword: &str, context: &str) -> f32 {
    let keyword_lower = keyword.to_lowercase();
    let context_lower = context.to_lowercase();
    
    // Count exact matches
    let exact_matches = context_lower.matches(&keyword_lower).count() as f32;
    
    // Calculate position bonus (earlier matches get higher scores)
    let position_bonus = if let Some(pos) = context_lower.find(&keyword_lower) {
        1.0 - (pos as f32 / context_lower.len() as f32)
    } else {
        0.0
    };
    
    // Calculate density (matches per word)
    let word_count = context.split_whitespace().count() as f32;
    let density = if word_count > 0.0 { exact_matches / word_count } else { 0.0 };
    
    // Combine factors
    exact_matches + (position_bonus * 0.5) + (density * 2.0)
}

#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Selector error: {0}")]
    SelectorError(String),

    #[error("Timeout error: Crawling exceeded the time limit")]
    TimeoutError,

    #[error("Date parsing error: {0}")]
    DateParsingError(String),

    #[error("Spider error: {0}")]
    SpiderError(String),

    #[error("Other error: {0}")]
    Other(String),
}

unsafe impl Send for CrawlerError {}
unsafe impl Sync for CrawlerError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlResult {
    pub results: Vec<DomainResult>,
    pub total_pages_crawled: usize,
    pub total_processing_time_ms: u64,
    pub crawl_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainResult {
    pub url: String,
    pub title: Option<String>,
    pub content: String,
    pub matches: Vec<KeywordMatch>,
    pub pages_crawled: usize,
    pub has_more_pages: bool,
    pub metadata: Option<CrawlMetadata>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlMetadata {
    pub crawl_timestamp: String,
    pub total_processing_time_ms: u64,
    pub content_summary: Option<String>,
    pub last_modified: Option<String>,
    pub published_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeywordMatch {
    pub keyword: String,
    pub context: String,
    pub cleaned_text: String,
    pub count: usize,
    pub relevance_score: Option<f32>,
    pub source_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlRequest {
    pub url: String,
    pub keywords: Vec<String>,
    pub max_depth: Option<usize>,
    pub max_time_seconds: Option<u64>,
    pub follow_pagination: Option<bool>,
    pub max_pages: Option<usize>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

// Helper function to parse multiple URLs from comma-separated string
fn parse_urls(url_string: &str) -> Result<Vec<Url>, CrawlerError> {
    let mut urls = Vec::new();
    
    for url_part in url_string.split(',') {
        let trimmed_url = url_part.trim();
        if !trimmed_url.is_empty() {
            // Add protocol if missing
            let full_url = if trimmed_url.starts_with("http://") || trimmed_url.starts_with("https://") {
                trimmed_url.to_string()
            } else {
                format!("https://{}", trimmed_url)
            };
            
            match Url::parse(&full_url) {
                Ok(parsed_url) => urls.push(parsed_url),
                Err(e) => {
                    // Try with http:// if https:// failed
                    if full_url.starts_with("https://") {
                        let http_url = full_url.replace("https://", "http://");
                        match Url::parse(&http_url) {
                            Ok(parsed_url) => urls.push(parsed_url),
                            Err(_) => return Err(CrawlerError::UrlError(e)),
                        }
                    } else {
                        return Err(CrawlerError::UrlError(e));
                    }
                }
            }
        }
    }
    
    if urls.is_empty() {
        return Err(CrawlerError::Other("No valid URLs provided".to_string()));
    }
    
    Ok(urls)
}

pub async fn crawl_website(request: &CrawlRequest) -> Result<CrawlResult, CrawlerError> {
    let start_processing_time = Instant::now();
    
    // Validate date range if provided
    let (date_from, date_to) = validate_date_range(request.date_from.as_ref(), request.date_to.as_ref())?;
    
    // Parse multiple URLs from the comma-separated string
    let urls = parse_urls(&request.url)?;
    
    let mut domain_results = Vec::new();
    let mut total_pages_crawled = 0;
    
    // Process each domain using Spider
    for base_url in urls {
        let domain_result = crawl_single_domain_with_spider(&base_url, request, start_processing_time, date_from, date_to).await;
        
        match domain_result {
            Ok(result) => {
                total_pages_crawled += result.pages_crawled;
                domain_results.push(result);
            }
            Err(err) => {
                // Create an error result for this domain
                let error_result = DomainResult {
                    url: base_url.to_string(),
                    title: None,
                    content: String::new(),
                    matches: Vec::new(),
                    pages_crawled: 0,
                    has_more_pages: false,
                    metadata: None,
                    error: Some(err.to_string()),
                };
                domain_results.push(error_result);
            }
        }
    }
    
    // Create metadata
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs();
    
    Ok(CrawlResult {
        results: domain_results,
        total_pages_crawled,
        total_processing_time_ms: start_processing_time.elapsed().as_millis() as u64,
        crawl_timestamp: format!("{}", timestamp),
    })
}

async fn crawl_single_domain_with_spider(
    base_url: &Url,
    request: &CrawlRequest,
    start_processing_time: Instant,
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
) -> Result<DomainResult, CrawlerError> {
    let start_time = Instant::now();
    let time_limit = request.max_time_seconds.map(Duration::from_secs);
    
    // Create Spider website instance
    let mut website = Website::new(base_url.as_str());
    
    // Configure Spider based on request parameters
    if let Some(max_pages) = request.max_pages {
        website.configuration.budget = Some(spider::hashbrown::HashMap::from([
            (spider::case_insensitive_string::CaseInsensitiveString::from("*"), max_pages as u32)
        ]));
    }
    
    if let Some(max_depth) = request.max_depth {
        website.configuration.depth = max_depth;
    }
    
    // Set request delay to be respectful
    website.configuration.delay = 1000; // 1 second delay between requests
    
    // Enable subdomains if needed
    website.configuration.subdomains = true;
    
    // Scrape the website to get pages with content
    website.scrape().await;
    
    // Process the scraped pages
    let pages = website.get_pages();
    let mut all_matches = Vec::new();
    let mut full_content = String::new();
    let mut page_title = None;
    let mut pages_crawled = 0;
    let mut has_more_pages = false;
    
    if let Some(pages) = pages {
        for (index, page) in pages.iter().enumerate() {
            // Check time limit
            if let Some(limit) = time_limit {
                if start_time.elapsed() > limit {
                    has_more_pages = true;
                    break;
                }
            }
            
            // Extract page dates for filtering
            let page_dates = extract_dates_from_page(page);
            
            // Check if the page matches the date filter
            if !matches_date_filter(&page_dates, date_from.as_ref(), date_to.as_ref()) {
                continue;
            }
            
            // Extract title from the first matching page
            if page_title.is_none() {
                if let Some(metadata) = page.get_metadata() {
                    if let Some(title) = &metadata.title {
                        page_title = Some(title.to_string());
                    }
                }
            }
            
            // Process page content for keyword matches
            let html_content = page.get_html();
            let cleaned_content = clean_html_text(&html_content);
            
            // Add to full content
            if !full_content.is_empty() {
                full_content.push_str("\n\n--- Next Page ---\n\n");
            }
            full_content.push_str(&cleaned_content);
            
            // Search for keywords in the cleaned content
            for keyword in &request.keywords {
                let keyword_lower = keyword.to_lowercase();
                let content_lower = cleaned_content.to_lowercase();
                
                if content_lower.contains(&keyword_lower) {
                    let count = content_lower.matches(&keyword_lower).count();
                    
                    // Extract context around keyword matches
                    let words: Vec<&str> = cleaned_content.split_whitespace().collect();
                    let mut contexts = Vec::new();
                    
                    for (i, word) in words.iter().enumerate() {
                        if word.to_lowercase().contains(&keyword_lower) {
                            let start = i.saturating_sub(5);
                            let end = std::cmp::min(i + 6, words.len());
                            let context = words[start..end].join(" ");
                            contexts.push(context);
                        }
                    }
                    
                    for context in contexts {
                        let relevance_score = calculate_relevance_score(keyword, &context);
                        
                        all_matches.push(KeywordMatch {
                            keyword: keyword.clone(),
                            context: context.clone(),
                            cleaned_text: context,
                            count,
                            relevance_score: Some(relevance_score),
                            source_url: page.get_url().to_string(),
                        });
                    }
                }
            }
            
            pages_crawled += 1;
            
            // Check if we've reached max pages
            if let Some(max_pages) = request.max_pages {
                if pages_crawled >= max_pages {
                    has_more_pages = index + 1 < pages.len();
                    break;
                }
            }
        }
        
        // Check if there were more pages available than we processed
        if let Some(max_pages) = request.max_pages {
            has_more_pages = has_more_pages || pages.len() > max_pages;
        }
    }
    
    // Create metadata
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs();
    
    let metadata = CrawlMetadata {
        crawl_timestamp: format!("{}", timestamp),
        total_processing_time_ms: start_processing_time.elapsed().as_millis() as u64,
        content_summary: if full_content.len() > 500 {
            Some(format!("{}...", &full_content[..500]))
        } else {
            Some(full_content.clone())
        },
        last_modified: None, // Could be extracted from first page if needed
        published_date: None, // Could be extracted from first page if needed
    };
    
    Ok(DomainResult {
        url: base_url.to_string(),
        title: page_title,
        content: full_content,
        matches: all_matches,
        pages_crawled,
        has_more_pages,
        metadata: Some(metadata),
        error: None,
    })
}