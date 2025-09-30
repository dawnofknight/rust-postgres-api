import React, { useState } from 'react';
import { 
  Box, Typography, Paper, Accordion, AccordionSummary, 
  AccordionDetails, Chip, LinearProgress, Button
} from '@mui/material';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import DownloadIcon from '@mui/icons-material/Download';
import type { CrawlResult, DomainResult, KeywordMatch } from '../api/client';

interface CrawlerResultsProps {
  result: CrawlResult | null;
  onDownload: () => void;
}

export default function CrawlerResults({ result, onDownload }: CrawlerResultsProps) {
  const [expandedMatch, setExpandedMatch] = useState<string | false>(false);

  const handleChange = (panel: string) => (_: React.SyntheticEvent, isExpanded: boolean) => {
    setExpandedMatch(isExpanded ? panel : false);
  };
  console.log(result);
  if (!result) return null;

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
        <Typography variant="h6">Crawl Results</Typography>
        <Button 
          variant="contained" 
          color="primary" 
          startIcon={<DownloadIcon />}
          onClick={onDownload}
        >
          Download JSON
        </Button>
      </Box>

      {result.results && result.results.length > 0 ? (
        result.results.map((domain: DomainResult, domainIndex: number) => (
        <Box key={domainIndex} sx={{ mb: 4 }}>
          <Paper elevation={1} sx={{ p: 2, mb: 3 }}>
            <Typography variant="h6" gutterBottom>
              Domain {domainIndex + 1}
            </Typography>
            <Typography variant="subtitle1">
              <strong>URL:</strong> {domain.url}
            </Typography>
            {domain.title && (
              <Typography variant="subtitle1">
                <strong>Title:</strong> {domain.title}
              </Typography>
            )}
            {domain.error ? (
              <Typography variant="subtitle1" color="error">
                <strong>Error:</strong> {domain.error}
              </Typography>
            ) : (
              <>
                <Typography variant="subtitle1">
                  <strong>Pages Crawled:</strong> {domain.pages_crawled}
                  {domain.has_more_pages && " (more pages available)"}
                </Typography>
                {domain.metadata && (
                  <Typography variant="subtitle1">
                    <strong>Processing Time:</strong> {(domain.metadata.total_processing_time_ms / 1000).toFixed(2)} seconds
                  </Typography>
                )}
              </>
            )}
          </Paper>

          {!domain.error && (
            <>
              <Typography variant="h6" gutterBottom>Keyword Matches for {domain.url}</Typography>
              
              {domain.matches.length === 0 ? (
                <Paper elevation={1} sx={{ p: 2, mb: 3 }}>
                  <Typography>No keyword matches found.</Typography>
                </Paper>
              ) : (
                domain.matches.map((match: KeywordMatch, index: number) => (
                  <Accordion 
                    key={`${domainIndex}-${match.keyword}-${index}`}
                    expanded={expandedMatch === `match-${domainIndex}-${index}`}
                    onChange={handleChange(`match-${domainIndex}-${index}`)}
                    sx={{ mb: 1 }}
                  >
                    <AccordionSummary expandIcon={<ExpandMoreIcon />}>
                      <Box sx={{ display: 'flex', alignItems: 'center', width: '100%' }}>
                        <Chip 
                          label={match.keyword} 
                          color="primary" 
                          size="small" 
                          sx={{ mr: 2 }} 
                        />
                        <Typography sx={{ flexGrow: 1 }}>
                          {match.count} occurrences
                        </Typography>
                        {match.relevance_score !== undefined && (
                          <Box sx={{ width: '30%', display: 'flex', alignItems: 'center' }}>
                            <Typography variant="body2" sx={{ mr: 1 }}>
                              Relevance: {Math.round(match.relevance_score * 100)}%
                            </Typography>
                            <LinearProgress 
                              variant="determinate" 
                              value={match.relevance_score * 100} 
                              sx={{ flexGrow: 1 }}
                            />
                          </Box>
                        )}
                      </Box>
                    </AccordionSummary>
                    <AccordionDetails>
                      <Typography variant="subtitle2" gutterBottom>Context:</Typography>
                      <Paper elevation={0} sx={{ p: 2, bgcolor: 'grey.100', mb: 2 }}>
                        <Typography variant="body2" component="pre" sx={{ whiteSpace: 'pre-wrap' }}>
                          {match.context}
                        </Typography>
                      </Paper>
                      
                      <Typography variant="subtitle2" gutterBottom>Cleaned Text:</Typography>
                      <Paper elevation={0} sx={{ p: 2, bgcolor: 'grey.100' }}>
                        <Typography variant="body2">
                          {match.cleaned_text}
                        </Typography>
                      </Paper>
                    </AccordionDetails>
                  </Accordion>
                ))
              )}
            </>
          )}
        </Box>
      ))
      ) : (
        <Paper elevation={1} sx={{ p: 2, mb: 3 }}>
          <Typography variant="h6" gutterBottom>
            No domain results available.
          </Typography>
        </Paper>
      )}
    </Box>
  );
}