import React, { useState, useMemo } from 'react';
import { 
  Box, Typography, Paper, Accordion, AccordionSummary, 
  AccordionDetails, Chip, LinearProgress, Button,
  FormControl, InputLabel, Select, MenuItem, OutlinedInput,
  Card, CardContent, Divider, Badge
} from '@mui/material';
import type { SelectChangeEvent } from '@mui/material';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import DownloadIcon from '@mui/icons-material/Download';
import FilterListIcon from '@mui/icons-material/FilterList';
import LabelIcon from '@mui/icons-material/Label';
import type { CrawlResult, DomainResult, KeywordMatch } from '../api/client';

interface CrawlerResultsProps {
  result: CrawlResult | null;
  onDownload: () => void;
}

// Color palette for keyword tags
const KEYWORD_COLORS = [
  '#1976d2', '#388e3c', '#f57c00', '#d32f2f', '#7b1fa2',
  '#0288d1', '#689f38', '#fbc02d', '#c2185b', '#5d4037',
  '#455a64', '#e64a19', '#00796b', '#303f9f', '#8bc34a'
];

export default function CrawlerResults({ result, onDownload }: CrawlerResultsProps) {
  const [expandedMatch, setExpandedMatch] = useState<string | false>(false);
  const [selectedKeywords, setSelectedKeywords] = useState<string[]>([]);
  const [expandedDomain, setExpandedDomain] = useState<number | false>(false);

  const handleMatchChange = (panel: string) => (_: React.SyntheticEvent, isExpanded: boolean) => {
    setExpandedMatch(isExpanded ? panel : false);
  };

  const handleDomainToggle = (domainIndex: number) => {
    setExpandedDomain(expandedDomain === domainIndex ? false : domainIndex);
  };

  const handleKeywordFilterChange = (event: SelectChangeEvent<typeof selectedKeywords>) => {
    const value = event.target.value;
    setSelectedKeywords(typeof value === 'string' ? value.split(',') : value);
  };

  // Extract all unique keywords and create color mapping
  const keywordData = useMemo(() => {
    if (!result?.results) return { allKeywords: [], keywordColorMap: new Map(), keywordStats: new Map() };
    
    const allKeywords = new Set<string>();
    const keywordStats = new Map<string, { totalCount: number, domainsFound: number, avgRelevance: number }>();
    
    result.results.forEach(domain => {
      const domainKeywords = new Set<string>();
      domain.matches.forEach(match => {
        allKeywords.add(match.keyword);
        domainKeywords.add(match.keyword);
        
        const current = keywordStats.get(match.keyword) || { totalCount: 0, domainsFound: 0, avgRelevance: 0 };
        current.totalCount += match.count;
        if (!domainKeywords.has(match.keyword)) {
          current.domainsFound += 1;
        }
        if (match.relevance_score !== undefined) {
          current.avgRelevance = (current.avgRelevance + match.relevance_score) / 2;
        }
        keywordStats.set(match.keyword, current);
      });
      
      domainKeywords.forEach(keyword => {
        const current = keywordStats.get(keyword)!;
        current.domainsFound = Math.max(current.domainsFound, 1);
      });
    });

    const keywordArray = Array.from(allKeywords).sort();
    const keywordColorMap = new Map<string, string>();
    keywordArray.forEach((keyword, index) => {
      keywordColorMap.set(keyword, KEYWORD_COLORS[index % KEYWORD_COLORS.length]);
    });

    return { allKeywords: keywordArray, keywordColorMap, keywordStats };
  }, [result]);

  // Filter matches based on selected keywords
  const filteredResults = useMemo(() => {
    if (!result?.results || selectedKeywords.length === 0) return result?.results || [];
    
    return result.results.map(domain => ({
      ...domain,
      matches: domain.matches.filter(match => selectedKeywords.includes(match.keyword))
    }));
  }, [result, selectedKeywords]);

  const getKeywordChip = (keyword: string, size: 'small' | 'medium' = 'small', showStats = false) => {
    const color = keywordData.keywordColorMap.get(keyword) || '#666';
    const stats = keywordData.keywordStats.get(keyword);
    
    return (
      <Chip
        key={keyword}
        label={showStats && stats ? `${keyword} (${stats.totalCount})` : keyword}
        size={size}
        sx={{
          backgroundColor: color,
          color: 'white',
          fontWeight: 'bold',
          '&:hover': {
            backgroundColor: color,
            opacity: 0.8
          }
        }}
        icon={showStats ? <LabelIcon sx={{ color: 'white !important' }} /> : undefined}
      />
    );
  };

  console.log(result);
  if (!result) return null;

  const displayResults = selectedKeywords.length > 0 ? filteredResults : result.results;

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

      {/* Keyword Overview Section */}
      {keywordData.allKeywords.length > 0 && (
        <Card sx={{ mb: 3 }}>
          <CardContent>
            <Typography variant="h6" gutterBottom sx={{ display: 'flex', alignItems: 'center' }}>
              <LabelIcon sx={{ mr: 1 }} />
              Keyword Overview ({keywordData.allKeywords.length} keywords found)
            </Typography>
            
            <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2, mb: 2 }}>
              {keywordData.allKeywords.map(keyword => {
                const stats = keywordData.keywordStats.get(keyword)!;
                return (
                  <Box key={keyword} sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
                    <Badge 
                      badgeContent={stats.totalCount} 
                      color="secondary"
                      sx={{
                        '& .MuiBadge-badge': {
                          backgroundColor: keywordData.keywordColorMap.get(keyword),
                          color: 'white'
                        }
                      }}
                    >
                      {getKeywordChip(keyword, 'medium')}
                    </Badge>
                    <Typography variant="caption" display="block" sx={{ textAlign: 'center', mt: 0.5 }}>
                      {stats.domainsFound} domain{stats.domainsFound !== 1 ? 's' : ''}
                      {stats.avgRelevance > 0 && (
                        <span> • {Math.round(stats.avgRelevance * 100)}% rel.</span>
                      )}
                    </Typography>
                  </Box>
                );
              })}
            </Box>

            <Divider sx={{ my: 2 }} />

            {/* Keyword Filter */}
            <FormControl fullWidth size="small">
              <InputLabel id="keyword-filter-label">
                <FilterListIcon sx={{ mr: 1, fontSize: 16 }} />
                Filter by Keywords (optional)
              </InputLabel>
              <Select
                labelId="keyword-filter-label"
                multiple
                value={selectedKeywords}
                onChange={handleKeywordFilterChange}
                input={<OutlinedInput label="Filter by Keywords (optional)" />}
                renderValue={(selected) => (
                  <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5 }}>
                    {selected.map((keyword) => getKeywordChip(keyword))}
                  </Box>
                )}
              >
                {keywordData.allKeywords.map((keyword) => (
                  <MenuItem key={keyword} value={keyword}>
                    <Box sx={{ display: 'flex', alignItems: 'center', width: '100%' }}>
                      {getKeywordChip(keyword)}
                      <Typography sx={{ ml: 1, flexGrow: 1 }}>
                        {keywordData.keywordStats.get(keyword)?.totalCount} occurrences
                      </Typography>
                    </Box>
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
          </CardContent>
        </Card>
      )}

      {displayResults && displayResults.length > 0 ? (
        displayResults.map((domain: DomainResult, domainIndex: number) => (
        <Box key={domainIndex} sx={{ mb: 4 }}>
          <Paper elevation={2} sx={{ p: 2, mb: 3 }}>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
              <Box sx={{ flexGrow: 1 }}>
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
              </Box>
              
              {/* Domain Keywords Summary */}
              {!domain.error && domain.matches.length > 0 && (
                <Box sx={{ ml: 2, minWidth: 200 }}>
                  <Typography variant="subtitle2" gutterBottom>
                    Keywords Found ({domain.matches.length}):
                  </Typography>
                  <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5 }}>
                    {Array.from(new Set(domain.matches.map(m => m.keyword))).map(keyword => 
                      getKeywordChip(keyword)
                    )}
                  </Box>
                </Box>
              )}
            </Box>
          </Paper>

          {!domain.error && (
            <>
              <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
                <Typography variant="h6">
                  Keyword Matches for {domain.url}
                </Typography>
                <Button
                  size="small"
                  onClick={() => handleDomainToggle(domainIndex)}
                  sx={{ textTransform: 'none' }}
                >
                  {expandedDomain === domainIndex ? 'Collapse All' : 'Expand All'}
                </Button>
              </Box>
              
              {domain.matches.length === 0 ? (
                <Paper elevation={1} sx={{ p: 2, mb: 3 }}>
                  <Typography>
                    {selectedKeywords.length > 0 
                      ? `No matches found for selected keywords: ${selectedKeywords.join(', ')}`
                      : 'No keyword matches found.'
                    }
                  </Typography>
                </Paper>
              ) : (
                domain.matches.map((match: KeywordMatch, index: number) => (
                  <Accordion 
                    key={`${domainIndex}-${match.keyword}-${index}`}
                    expanded={expandedMatch === `match-${domainIndex}-${index}` || expandedDomain === domainIndex}
                    onChange={handleMatchChange(`match-${domainIndex}-${index}`)}
                    sx={{ mb: 1 }}
                  >
                    <AccordionSummary expandIcon={<ExpandMoreIcon />}>
                      <Box sx={{ display: 'flex', alignItems: 'center', width: '100%' }}>
                        {getKeywordChip(match.keyword)}
                        <Typography sx={{ flexGrow: 1, ml: 2 }}>
                          <strong>{match.count}</strong> occurrence{match.count !== 1 ? 's' : ''}
                        </Typography>
                        {match.relevance_score !== undefined && (
                          <Box sx={{ width: '30%', display: 'flex', alignItems: 'center' }}>
                            <Typography variant="body2" sx={{ mr: 1 }}>
                              Relevance: {Math.round(match.relevance_score * 100)}%
                            </Typography>
                            <LinearProgress 
                              variant="determinate" 
                              value={match.relevance_score * 100} 
                              sx={{ 
                                flexGrow: 1,
                                '& .MuiLinearProgress-bar': {
                                  backgroundColor: keywordData.keywordColorMap.get(match.keyword)
                                }
                              }}
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
            {selectedKeywords.length > 0 
              ? `No results found for selected keywords: ${selectedKeywords.join(', ')}`
              : 'No domain results available.'
            }
          </Typography>
        </Paper>
      )}
    </Box>
  );
}