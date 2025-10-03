import { useState } from 'react';
import { Box, Paper, Typography, Alert, Button } from '@mui/material';
import SocialCrawlerForm from '../components/SocialCrawlerForm';
import type { SocialQueryRequest } from '../components/SocialCrawlerForm';
import { socialApi } from '../api/client';

export default function SocialCrawlerPage() {
  const [result, setResult] = useState<unknown | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (request: SocialQueryRequest) => {
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      let data: unknown;
      const keyword = (request.keyword || '').trim();
      if (!keyword) {
        throw new Error('Keyword is required');
      }

      const { path, method } = request;
      switch (request.source) {
        case 'tikhub-twitter': {
          const search_type = (request.search_type || 'Top').trim();
          data = await socialApi.tikhubTwitter(path, { keyword, search_type }, method);
          break;
        }
        case 'tikhub-tiktok': {
          // TikHub TikTok web expects `keyword` plus optional count/offset
          data = await socialApi.tikhubTiktok(path, { keyword }, method);
          break;
        }
        case 'tikhub-generic': {
          const service = request.service || '';
          if (!service) {
            throw new Error('TikHub service is required for generic source');
          }
          data = await socialApi.tikhubGeneric(service, path, { q: keyword }, method);
          break;
        }
        case 'rapidapi-instagram': {
          data = await socialApi.rapidInstagram(path, { query: keyword }, method);
          break;
        }
        case 'rapidapi-twitter-v24': {
          data = await socialApi.rapidTwitterV24(path, { query: keyword }, method);
          break;
        }
        case 'rapidapi-generic': {
          const host = request.host || '';
          if (!host) {
            throw new Error('RapidAPI host is required for generic source');
          }
          data = await socialApi.rapidGeneric(host, path, { query: keyword }, method);
          break;
        }
        default:
          throw new Error('Unsupported source');
      }

      setResult(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An unknown error occurred');
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = () => {
    if (!result) return;
    const dataStr = JSON.stringify(result, null, 2);
    const dataUri = 'data:application/json;charset=utf-8,' + encodeURIComponent(dataStr);
    const exportFileDefaultName = `social-result-${new Date().toISOString()}.json`;
    const linkElement = document.createElement('a');
    linkElement.setAttribute('href', dataUri);
    linkElement.setAttribute('download', exportFileDefaultName);
    linkElement.click();
  };

  return (
    <Box sx={{ display: 'flex', minHeight: '100vh', width: '100%', p: 0 }}>
      {/* Left Column - Parameters */}
      <Box sx={{ width: '40%', pr: 2, display: 'flex', flexDirection: 'column' }}>
        <Paper elevation={0} sx={{ p: 2, mb: 2, flexGrow: 0 }}>
          <Typography variant="h5" gutterBottom>
            Social Media Search
          </Typography>
          <SocialCrawlerForm onSubmit={handleSubmit} isLoading={loading} />
        </Paper>

        {error && (
          <Alert severity="error" sx={{ mb: 2 }}>
            {error}
          </Alert>
        )}
      </Box>

      {/* Right Column - Results */}
      <Box sx={{ width: '60%', pl: 0, display: 'flex', flexDirection: 'column' }}>
        {result ? (
          <Paper elevation={0} sx={{ p: 2, height: '100%', bgcolor: 'grey.100' }}>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 2 }}>
              <Typography variant="h6">Results</Typography>
              <Box>
                <Button variant="outlined" onClick={handleDownload}>Download JSON</Button>
              </Box>
            </Box>
            <pre style={{ whiteSpace: 'pre-wrap', wordBreak: 'break-word', margin: 0 }}>
              {JSON.stringify(result, null, 2)}
            </pre>
          </Paper>
        ) : (
          <Paper elevation={0} sx={{ p: 0, height: '100%', bgcolor: '#cfe8fc' }}>
            {loading && (
              <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100%' }}>
                <Typography variant="body1" color="text.secondary">
                  Fetching social data...
                </Typography>
              </Box>
            )}
          </Paper>
        )}
      </Box>
    </Box>
  );
}