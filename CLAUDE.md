# DCADMIN Development Guide

## Build Commands
- Development: `npm run tauri dev`
- Build: `npm run tauri build`
- Check TypeScript: `npm run check`
- Lint: `npm run lint`
- Format: `npm run format`

## Code Style Guidelines
- **TypeScript**: Strict mode enabled
- **Formatting**: Use Prettier with tabs, single quotes, no trailing commas, 100 chars width
- **Naming**: camelCase for variables/functions, PascalCase for components/types
- **Imports**: Group imports by source (stdlib, packages, local)
- **Error Handling**: Use typed error handling, avoid throwing generic errors
- **Components**: Svelte components, typed props using TypeScript
- **State Management**: Use Svelte stores

## Project Structure
- `src/`: SvelteKit frontend
- `src-tauri/`: Rust backend
- `static/`: Static assets