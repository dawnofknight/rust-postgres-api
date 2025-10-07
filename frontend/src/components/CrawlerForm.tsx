import React, { useState } from 'react';
import { 
  Box, Button, TextField, Typography, 
  Slider 
} from '@mui/material';
import type { CrawlRequest } from '../api/client';

interface CrawlerFormProps {
  onSubmit: (request: CrawlRequest) => void;
  isLoading: boolean;
}

export default function CrawlerForm({ onSubmit, isLoading }: CrawlerFormProps) {
  const [url, setUrl] = useState('');
  const [keywords, setKeywords] = useState('');
  const [maxPages, setMaxPages] = useState<number | undefined>(5);
  const [dateFrom, setDateFrom] = useState('');
  const [dateTo, setDateTo] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    const request: CrawlRequest = {
      url,
      keywords: keywords.split(',').map(k => k.trim()).filter(k => k !== ''),
      max_pages: maxPages,
      date_from: dateFrom || null,
      date_to: dateTo || null,
    };
    
    onSubmit(request);
  };

  return (
    <Box component="form" onSubmit={handleSubmit}>
      <Box sx={{ display: 'grid', gridTemplateColumns: '1fr', gap: 3 }}>
        <Box>
          <TextField
            fullWidth
            label="Website URL"
            placeholder="https://www.newsnow.co.uk/h/"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            required
            variant="outlined"
            margin="normal"
            helperText="Enter the website URL to crawl"
          />
        </Box>

        <Box>
          <TextField
            fullWidth
            label="Keywords (comma separated)"
            placeholder="bangsamoro, keyword2, keyword3"
            value={keywords}
            onChange={(e) => setKeywords(e.target.value)}
            required
            variant="outlined"
            margin="normal"
            helperText="Enter keywords separated by commas"
          />
        </Box>

        <Box>
          <Typography gutterBottom>
            Max Pages: {maxPages}
          </Typography>
          <Slider
            value={maxPages || 0}
            onChange={(_, value) => setMaxPages(value as number)}
            step={1}
            marks
            min={1}
            max={20}
            valueLabelDisplay="auto"
          />
        </Box>

        {/* Date Range Filter */}
        <Box>
          <Typography variant="h6" gutterBottom sx={{ mt: 2, mb: 1 }}>
            Date Range Filter (Optional)
          </Typography>
          <Box sx={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 2 }}>
            <TextField
              label="From Date"
              type="date"
              value={dateFrom}
              onChange={(e) => setDateFrom(e.target.value)}
              InputLabelProps={{
                shrink: true,
              }}
              helperText="Filter pages from this date"
            />
            <TextField
              label="To Date"
              type="date"
              value={dateTo}
              onChange={(e) => setDateTo(e.target.value)}
              InputLabelProps={{
                shrink: true,
              }}
              helperText="Filter pages until this date"
            />
          </Box>
        </Box>

        <Box sx={{ mt: 2 }}>
          <Button
            type="submit"
            variant="contained"
            color="primary"
            fullWidth
            disabled={isLoading}
          >
            {isLoading ? 'Crawling...' : 'Start Crawling'}
          </Button>
        </Box>
      </Box>
    </Box>
  );
}