import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { AppBar, Toolbar, Typography, Box, CssBaseline, ThemeProvider, createTheme, Button } from '@mui/material';
import CrawlerPage from './pages/CrawlerPage';
import SocialCrawlerPage from './pages/SocialCrawlerPage';

const theme = createTheme({
  palette: {
    primary: {
      main: '#1976d2',
    },
    secondary: {
      main: '#dc004e',
    },
  },
});

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Router>
        <Box sx={{ 
          display: 'flex', 
          flexDirection: 'column', 
          minHeight: '100vh',
          width: '100%'
        }}>
          <AppBar position="static">
            <Toolbar>
              <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                Web Crawler
              </Typography>
              <Box sx={{ display: 'flex', gap: 1 }}>
                <Button color="inherit" component={Link} to="/">Web</Button>
                <Button color="inherit" component={Link} to="/social">Social</Button>
              </Box>
            </Toolbar>
          </AppBar>
          
          <Box 
            component="main" 
          >
            <Routes>
              <Route path="/" element={<CrawlerPage />} />
              <Route path="/social" element={<SocialCrawlerPage />} />
            </Routes>
          </Box>
        </Box>
      </Router>
    </ThemeProvider>
  );
}

export default App;
