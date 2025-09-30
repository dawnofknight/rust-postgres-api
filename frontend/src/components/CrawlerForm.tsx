import React, { useState } from 'react';
import { 
  Box, Button, TextField, Typography, 
  FormControlLabel, Checkbox, Slider 
} from '@mui/material';
import type { CrawlRequest } from '../api/client';

interface CrawlerFormProps {
  onSubmit: (request: CrawlRequest) => void;
  isLoading: boolean;
}

export default function CrawlerForm({ onSubmit, isLoading }: CrawlerFormProps) {
  const [url, setUrl] = useState('');
  const [keywords, setKeywords] = useState('');
  const [maxDepth, setMaxDepth] = useState<number | undefined>(2);
  const [maxTimeSeconds, setMaxTimeSeconds] = useState<number | undefined>(30);
  const [followPagination, setFollowPagination] = useState(true);
  const [maxPages, setMaxPages] = useState<number | undefined>(10);
  const [dateFrom, setDateFrom] = useState('');
  const [dateTo, setDateTo] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    const request: CrawlRequest = {
      url,
      keywords: keywords.split(',').map(k => k.trim()).filter(k => k !== ''),
      max_depth: maxDepth,
      max_time_seconds: maxTimeSeconds,
      follow_pagination: followPagination,
      max_pages: maxPages,
      date_from: dateFrom || undefined,
      date_to: dateTo || undefined
    };
    
    onSubmit(request);
  };

  return (
    <Box component="form" onSubmit={handleSubmit}>
      <Box sx={{ display: 'grid', gridTemplateColumns: '1fr', gap: 3 }}>
        <Box>
          <TextField
            fullWidth
            label="Website URLs (comma separated)"
            placeholder="https://example.com, https://another-site.com"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            required
            variant="outlined"
            margin="normal"
            helperText="Enter multiple URLs separated by commas"
          />
        </Box>

        <Box>
          <TextField
            fullWidth
            label="Keywords (comma separated)"
            placeholder="keyword1, keyword2, keyword3"
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
            Max Crawl Depth: {maxDepth}
          </Typography>
          <Slider
            value={maxDepth || 0}
            onChange={(_, value) => setMaxDepth(value as number)}
            step={1}
            marks
            min={1}
            max={5}
            valueLabelDisplay="auto"
          />
        </Box>

        <Box>
          <Typography gutterBottom>
            Max Time (seconds): {maxTimeSeconds}
          </Typography>
          <Slider
            value={maxTimeSeconds || 0}
            onChange={(_, value) => setMaxTimeSeconds(value as number)}
            step={10}
            marks
            min={10}
            max={3600}
            valueLabelDisplay="auto"
          />
        </Box>

        <Box>
          <Typography gutterBottom>
            Max Pages: {maxPages}
          </Typography>
          <Slider
            value={maxPages || 0}
            onChange={(_, value) => setMaxPages(value as number)}
            step={5}
            marks
            min={5}
            max={50}
            valueLabelDisplay="auto"
          />
        </Box>

        <Box>
          <FormControlLabel
            control={
              <Checkbox
                checked={followPagination}
                onChange={(e) => setFollowPagination(e.target.checked)}
                color="primary"
              />
            }
            label="Follow Pagination Links"
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