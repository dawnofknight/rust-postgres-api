import React, { useState } from 'react';
import {
  Box,
  Button,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem
} from '@mui/material';

export type SocialSource =
  | 'tikhub-twitter'
  | 'tikhub-tiktok'
  | 'tikhub-generic'
  | 'rapidapi-instagram'
  | 'rapidapi-twitter-v24'
  | 'rapidapi-generic';

export interface SocialQueryRequest {
  source: SocialSource;
  keyword: string;
  path: string;
  method: 'GET' | 'POST';
  // Optional for generic RapidAPI
  host?: string;
  // Optional for TikHub generic
  service?: string;
  // Optional for TikHub Twitter search type
  search_type?: string;
}

interface SocialCrawlerFormProps {
  onSubmit: (request: SocialQueryRequest) => void;
  isLoading: boolean;
}

function getDefaultPath(source: SocialSource): string {
  switch (source) {
    case 'tikhub-twitter':
      return 'fetch_search_timeline';
    case 'tikhub-tiktok':
      return 'fetch_search_video';
    case 'tikhub-generic':
      return 'search';
    case 'rapidapi-instagram':
      return 'search';
    case 'rapidapi-twitter-v24':
      return 'search';
    case 'rapidapi-generic':
      return 'search';
    default:
      return 'search';
  }
}

export default function SocialCrawlerForm({ onSubmit, isLoading }: SocialCrawlerFormProps) {
  const [source, setSource] = useState<SocialSource>('rapidapi-twitter-v24');
  const [keyword, setKeyword] = useState('');
  const [host, setHost] = useState('');
  const [service, setService] = useState('twitter/web');
  const [path, setPath] = useState<string>(getDefaultPath('rapidapi-twitter-v24'));
  const [method, setMethod] = useState<'GET' | 'POST'>('GET');
  const [searchType, setSearchType] = useState<string>('Top');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const req: SocialQueryRequest = { 
      source, 
      keyword: keyword.trim(), 
      path: path.trim(), 
      method 
    };
    if (source === 'rapidapi-generic' && host.trim()) {
      req.host = host.trim();
    }
    if (source === 'tikhub-generic' && service.trim()) {
      req.service = service.trim();
    }
    if (source === 'tikhub-twitter' && searchType.trim()) {
      req.search_type = searchType.trim();
    }
    onSubmit(req);
  };

  return (
    <Box component="form" onSubmit={handleSubmit}>
      <Box sx={{ display: 'grid', gridTemplateColumns: '1fr', gap: 3 }}>
        <Box>
          <FormControl fullWidth>
            <InputLabel id="social-source-label">Source</InputLabel>
            <Select
              labelId="social-source-label"
              value={source}
              label="Source"
              onChange={(e) => {
                const next = e.target.value as SocialSource;
                setSource(next);
                setPath(getDefaultPath(next));
              }}
            >
              <MenuItem value="tikhub-twitter">TikHub - Twitter (web)</MenuItem>
              <MenuItem value="tikhub-tiktok">TikHub - TikTok (app v3)</MenuItem>
              <MenuItem value="tikhub-generic">TikHub - Generic</MenuItem>
              <MenuItem value="rapidapi-instagram">RapidAPI - Instagram</MenuItem>
              <MenuItem value="rapidapi-twitter-v24">RapidAPI - Twitter v24</MenuItem>
              <MenuItem value="rapidapi-generic">RapidAPI - Generic</MenuItem>
            </Select>
          </FormControl>
        </Box>

        {source === 'tikhub-twitter' && (
          <Box>
            <FormControl fullWidth>
              <InputLabel id="search-type-label">Search Type</InputLabel>
              <Select
                labelId="search-type-label"
                value={searchType}
                label="Search Type"
                onChange={(e) => setSearchType(e.target.value as string)}
              >
                <MenuItem value="Top">Top</MenuItem>
                <MenuItem value="Latest">Latest</MenuItem>
              </Select>
            </FormControl>
          </Box>
        )}
        {source === 'tikhub-generic' && (
          <Box>
            <TextField
              fullWidth
              label="TikHub Service"
              placeholder="twitter/web or tiktok/app/v3"
              value={service}
              onChange={(e) => setService(e.target.value)}
              variant="outlined"
              margin="normal"
              required
            />
          </Box>
        )}

        {source === 'rapidapi-generic' && (
          <Box>
            <TextField
              fullWidth
              label="RapidAPI Host"
              placeholder="example.p.rapidapi.com"
              value={host}
              onChange={(e) => setHost(e.target.value)}
              variant="outlined"
              margin="normal"
              required
            />
          </Box>
        )}

        <Box>
          <TextField
            fullWidth
            label="Endpoint Path"
            placeholder={getDefaultPath(source)}
            value={path}
            onChange={(e) => setPath(e.target.value)}
            required
            variant="outlined"
            margin="normal"
            helperText="Relative path under provider (e.g., search/tweets)"
          />
        </Box>

        <Box>
          <FormControl fullWidth>
            <InputLabel id="http-method-label">HTTP Method</InputLabel>
            <Select
              labelId="http-method-label"
              value={method}
              label="HTTP Method"
              onChange={(e) => setMethod(e.target.value as 'GET' | 'POST')}
            >
              <MenuItem value="GET">GET</MenuItem>
              <MenuItem value="POST">POST</MenuItem>
            </Select>
          </FormControl>
        </Box>

        <Box>
          <TextField
            fullWidth
            label="Keyword"
            placeholder="rustlang"
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
            required
            variant="outlined"
            margin="normal"
            helperText="Search term or query keyword"
          />
        </Box>

        <Box sx={{ mt: 2 }}>
          <Button
            type="submit"
            variant="contained"
            color="primary"
            fullWidth
            disabled={isLoading}
          >
            {isLoading ? 'Searching...' : 'Search Social'}
          </Button>
        </Box>
      </Box>
    </Box>
  );
}