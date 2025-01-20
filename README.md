# Rust + React Monorepo with Turborepo

This monorepo contains a Rust backend and React frontend, managed with Turborepo.

## Project Structure
- `apps/frontend`: React application (Vite + TypeScript)
- `apps/backend`: Rust backend service (Axum)

## Getting Started

### Prerequisites
- Node.js
- Rust
- Cargo watch (optional, for development)

### Installation
1. Install dependencies:
   ```bash
   npm install
   cd apps/frontend && npm install
   cd ../backend && cargo build
   ```

### Development
Run both frontend and backend in development mode:
```bash
npm run dev
```

Or run them separately:
- Frontend: `cd apps/frontend && npm run dev`
- Backend: `cd apps/backend && cargo run`

### Building
Build all applications:
```bash
npm run build
```

## URLs
- Frontend: http://localhost:5173
- Backend: http://localhost:3001
