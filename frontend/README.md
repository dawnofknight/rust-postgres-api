# Web Crawler Frontend

Modern React frontend for the Rust Web Crawler API. Built with TypeScript, Vite, and Tailwind CSS for a responsive and intuitive crawling interface.

## 🚀 Features

### Core Functionality
- **Intuitive Crawler Interface**: Clean, user-friendly form for web crawling
- **Real-time Results**: Live display of crawl results with keyword matches
- **Responsive Design**: Mobile-first design that works on all devices
- **Modern UI Components**: Beautiful, accessible components with Tailwind CSS
- **Type Safety**: Full TypeScript integration for robust development

### User Experience
- **Simplified Form**: Streamlined interface with essential crawling options
- **Visual Feedback**: Loading states, progress indicators, and result visualization
- **Error Handling**: Clear error messages and validation feedback
- **Keyword Highlighting**: Visual emphasis on matched keywords in results
- **Export Options**: Easy copying and sharing of crawl results

## 🏗️ Tech Stack

- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite for fast development and optimized builds
- **Styling**: Tailwind CSS for utility-first styling
- **HTTP Client**: Axios for API communication
- **Development**: Hot Module Replacement (HMR) for instant updates
- **Code Quality**: ESLint with TypeScript-aware rules

## 📁 Project Structure

```
frontend/
├── src/
│   ├── api/                    # API client and types
│   │   └── client.ts           # HTTP client with CrawlRequest interface
│   ├── components/             # React components
│   │   ├── CrawlerForm.tsx     # Main crawler form component
│   │   └── CrawlerPage.tsx     # Main page layout
│   ├── App.tsx                 # Root application component
│   ├── main.tsx                # Application entry point
│   └── index.css               # Global styles and Tailwind imports
├── public/                     # Static assets
├── index.html                  # HTML template
├── package.json                # Dependencies and scripts
├── tailwind.config.js          # Tailwind CSS configuration
├── tsconfig.json               # TypeScript configuration
├── vite.config.ts              # Vite configuration
└── README.md                   # This file
```

## ⚡ Quick Start

### Prerequisites
- **Node.js** (v18 or higher)
- **npm** or **yarn**

### Development Setup

1. **Install Dependencies**:
   ```bash
   cd frontend
   npm install
   ```

2. **Start Development Server**:
   ```bash
   npm run dev
   ```
   
   Frontend runs on: `http://localhost:5173` (or next available port)

3. **Build for Production**:
   ```bash
   npm run build
   ```

4. **Preview Production Build**:
   ```bash
   npm run preview
   ```

## 🔧 Configuration

### API Integration

The frontend connects to the Rust backend API. Update the base URL in <mcfile name="client.ts" path="frontend/src/api/client.ts"></mcfile>:

```typescript
const API_BASE_URL = 'http://localhost:8081';
```

### Environment Variables

Create a `.env` file for environment-specific configuration:

```bash
VITE_API_BASE_URL=http://localhost:8081
VITE_APP_TITLE=Web Crawler
```

## 🎨 Components Overview

### CrawlerForm Component

The main form component handles user input and API communication:

- **URL Input**: Target website URL validation
- **Keywords**: Multiple keyword input with array handling
- **Max Pages**: Slider control for crawl depth (1-20 pages)
- **Date Filters**: Optional date range filtering
- **Submit Handling**: Form validation and API request management

### CrawlerPage Component

The main page layout that orchestrates the crawling interface:

- **Form Integration**: Embeds the CrawlerForm component
- **Results Display**: Shows crawl results in a structured format
- **Loading States**: Visual feedback during API requests
- **Error Handling**: User-friendly error messages

## 📡 API Integration

### CrawlRequest Interface

```typescript
interface CrawlRequest {
  url: string;
  keywords: string[];
  max_pages?: number;
  date_from?: string | null;
  date_to?: string | null;
}
```

### API Client

The API client handles all backend communication:

- **Type Safety**: Full TypeScript interfaces for requests/responses
- **Error Handling**: Comprehensive error catching and user feedback
- **Request Validation**: Client-side validation before API calls
- **Response Processing**: Structured handling of crawl results

## 🎯 Recent Updates & Improvements

### ✅ Completed Features
- **Simplified Interface**: Removed complex options (max_depth, follow_pagination, etc.)
- **Streamlined API**: Updated to match backend's simplified `/crawl` endpoint
- **Better UX**: Improved form layout and user interaction
- **Type Safety**: Enhanced TypeScript integration throughout
- **Responsive Design**: Mobile-friendly interface

### 🔧 Technical Improvements
- **API Alignment**: Perfect sync with backend CrawlRequest structure
- **Form Validation**: Client-side validation for better UX
- **Error Handling**: Comprehensive error states and messages
- **Performance**: Optimized component rendering and state management

### 🧪 Tested Features
- ✅ Form submission and validation
- ✅ API communication with backend
- ✅ Results display and formatting
- ✅ Error handling and user feedback
- ✅ Responsive design across devices
- ✅ TypeScript compilation and type checking

## 🛠️ Development

### Available Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Start development server with HMR |
| `npm run build` | Build for production |
| `npm run preview` | Preview production build locally |
| `npm run lint` | Run ESLint for code quality |
| `npm run type-check` | Run TypeScript compiler check |

### Code Quality

- **ESLint**: Configured with React and TypeScript rules
- **TypeScript**: Strict type checking enabled
- **Prettier**: Code formatting (if configured)
- **Git Hooks**: Pre-commit hooks for code quality (if configured)

### Adding New Features

1. **Components**: Create new components in `src/components/`
2. **API Types**: Update interfaces in `src/api/client.ts`
3. **Styling**: Use Tailwind CSS classes for consistent design
4. **Testing**: Add tests for new functionality (when test setup is added)

## 🎨 Styling & Design

### Tailwind CSS

The project uses Tailwind CSS for styling:

- **Utility-First**: Compose designs using utility classes
- **Responsive**: Mobile-first responsive design system
- **Customizable**: Easy theme customization in `tailwind.config.js`
- **Performance**: Purged CSS for minimal bundle size

### Design System

- **Colors**: Consistent color palette for UI elements
- **Typography**: Readable font hierarchy and spacing
- **Components**: Reusable component patterns
- **Accessibility**: ARIA labels and keyboard navigation support

## 🔮 Future Enhancements

### Planned Features
- **Advanced Filters**: More sophisticated filtering options
- **Result Export**: CSV, JSON export functionality
- **Crawl History**: Save and manage previous crawl results
- **Real-time Updates**: WebSocket integration for live crawl progress
- **Batch Operations**: Multiple URL crawling interface
- **Analytics Dashboard**: Crawl statistics and insights

### Technical Improvements
- **Testing Suite**: Unit and integration tests with Jest/Vitest
- **State Management**: Redux or Zustand for complex state
- **PWA Features**: Offline support and app-like experience
- **Performance**: Code splitting and lazy loading
- **Accessibility**: Enhanced screen reader support
- **Internationalization**: Multi-language support

## 🤝 Contributing

### Development Guidelines

1. **Code Style**: Follow existing patterns and ESLint rules
2. **TypeScript**: Maintain strict type safety
3. **Components**: Keep components focused and reusable
4. **API Integration**: Update types when backend changes
5. **Documentation**: Update README for significant changes

### Best Practices

- **Component Design**: Single responsibility principle
- **State Management**: Use React hooks effectively
- **Performance**: Optimize re-renders and bundle size
- **Accessibility**: Ensure keyboard navigation and screen reader support
- **Error Boundaries**: Implement error boundaries for robust UX

## 📝 License

MIT License - see LICENSE file for details.

---

## 🔗 Related Documentation

- **Backend API**: See `../backend/README.md` for API documentation
- **Main Project**: See `../README.md` for overall project information
- **Deployment**: Docker and production deployment guides in main README
