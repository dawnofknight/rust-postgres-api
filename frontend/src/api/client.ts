import axios from 'axios';

// Define the API base URL
const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8081';

// Define types based on backend API
export interface CrawlRequest {
  url: string;
  keywords: string[];
  max_depth?: number;
  max_time_seconds?: number;
  follow_pagination?: boolean;
  max_pages?: number;
  date_from?: string; // ISO 8601 date string (YYYY-MM-DD)
  date_to?: string;   // ISO 8601 date string (YYYY-MM-DD)
}

export interface KeywordMatch {
  keyword: string;
  context: string;
  cleaned_text: string;
  count: number;
  relevance_score?: number;
  source_url: string; // URL where this keyword match was found
}

export interface CrawlMetadata {
  crawl_timestamp: string;
  total_processing_time_ms: number;
  content_summary?: string;
  last_modified?: string; // ISO 8601 date string for page last modified date
  published_date?: string; // ISO 8601 date string for page published date
}

export interface DomainResult {
  url: string;
  title?: string;
  matches: KeywordMatch[];
  pages_crawled: number;
  has_more_pages: boolean;
  metadata?: CrawlMetadata;
  error?: string;
}

export interface CrawlResult {
  results: DomainResult[];
  total_pages_crawled: number;
  total_processing_time_ms: number;
  crawl_timestamp: string;
}

// API client for crawler
export const crawlerApi = {
  crawlWebsite: async (request: CrawlRequest): Promise<CrawlResult> => {
    try {
      const response = await axios.post(`${API_URL}/crawl`, request);
      return response.data;
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        throw new Error(error.response.data.error || 'Failed to crawl website');
      }
      throw new Error('Failed to connect to the server');
    }
  }
};

// API client for health check
export const healthApi = {
  checkHealth: async (): Promise<{ status: string }> => {
    try {
      const response = await axios.get(`${API_URL}/health`);
      return response.data;
    } catch {
      throw new Error('Health check failed');
    }
  }
};