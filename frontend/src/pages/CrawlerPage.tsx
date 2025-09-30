import React, { useState } from 'react';
import { Box, Paper, Typography, Alert } from '@mui/material';
import CrawlerForm from '../components/CrawlerForm';
import CrawlerResults from '../components/CrawlerResults';
import { crawlerApi } from '../api/client';
import type { CrawlRequest, CrawlResult } from '../api/client';

const CrawlerPage: React.FC = () => {
  const [result, setResult] = useState<CrawlResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (request: CrawlRequest) => {
    setLoading(true);
    setError(null);
    
    try {
      const response = await crawlerApi.crawlWebsite(request);
      setResult(response);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An unknown error occurred');
      setResult(null);
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = () => {
    if (!result) return;
    
    const dataStr = JSON.stringify(result, null, 2);
    const dataUri = 'data:application/json;charset=utf-8,'+ encodeURIComponent(dataStr);
    
    const exportFileDefaultName = `crawl-result-${new Date().toISOString()}.json`;
    
    const linkElement = document.createElement('a');
    linkElement.setAttribute('href', dataUri);
    linkElement.setAttribute('download', exportFileDefaultName);
    linkElement.click();
  };

  return (
    <Box sx={{ 
      display: 'flex', 
      minHeight: '100vh',
      width: '100%',
      p: 0
    }}>
      {/* Left Column - Parameters */}
      <Box sx={{ 
        width: '40%', 
        pr: 2,
        display: 'flex',
        flexDirection: 'column'
      }}>
        <Paper elevation={0} sx={{ p: 2, mb: 2, flexGrow: 0 }}>
          <Typography variant="h5" gutterBottom>
            Web Crawler
          </Typography>
          <CrawlerForm onSubmit={handleSubmit} isLoading={loading} />
        </Paper>

        {error && (
          <Alert severity="error" sx={{ mb: 2 }}>
            {error}
          </Alert>
        )}
      </Box>

      {/* Right Column - Results */}
      <Box sx={{ 
        width: '60%', 
        pl: 0,
        display: 'flex',
        flexDirection: 'column'
      }}>
        {result ? (
          <Paper elevation={0} sx={{ p: 0, height: '100%' }}>
            <CrawlerResults result={result} onDownload={handleDownload} />
          </Paper>
        ) : (
          <Paper elevation={0} sx={{ 
            p: 0, 
            height: '100%',
            bgcolor: '#b8c6ef'
          }}>
            {loading && (
              <Box sx={{ 
                display: 'flex', 
                justifyContent: 'center', 
                alignItems: 'center',
                height: '100%'
              }}>
                <Typography variant="body1" color="text.secondary">
                  Crawling in progress...
                </Typography>
              </Box>
            )}
          </Paper>
        )}
      </Box>
    </Box>
  );
};

export default CrawlerPage;