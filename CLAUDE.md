# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a document management system (`doc_man_db`) written in Rust, designed to manage organizational documents with complex folder path management, considering organizational changes and personnel transfers. The system is implemented as a hybrid Tauri desktop app + web application with GraphQL API.

## Architecture

- **Backend**: Rust with Axum web framework, SQLx for database access
- **Frontend**: SvelteKit + TypeScript + Tailwind CSS (located in `ui/` directory)
- **Desktop**: Tauri application (planned integration with SvelteKit frontend)
- **Database**: SQLite (development) → SQL Server (production migration planned)
- **API**: GraphQL for efficient data fetching
- **Authentication**: Windows AD integration (with JSON fallback for development)

## Key Development Commands

### Backend Development (Rust)

```bash
# Build project
cargo build

# Run development server  
cargo run

# Run with release optimizations
cargo build --release

# Update dependencies
cargo update

# Clean build artifacts
cargo clean
```

### Frontend Development (SvelteKit)

```bash
# Navigate to UI directory
cd ui

# Install dependencies
npm install

# Run development server
npm run dev

# Run development server with auto-open browser
npm run dev -- --open

# Build for production
npm run build

# Preview production build
npm run preview

# TypeScript type checking
npm run check

# TypeScript type checking with watch mode
npm run check:watch
```

### Testing & Quality

```bash
# Backend testing
cargo test                    # Run all tests
cargo test --lib             # Unit tests only
cargo test --test '*'        # Integration tests only
cargo test models::          # Run specific test module
cargo fmt                    # Format code
cargo clippy                 # Lint code
cargo audit                  # Security audit

# Frontend type checking
cd ui && npm run check       # TypeScript type checking
```

### Database Operations

```bash
# Install SQLx CLI (if not installed)
cargo install sqlx-cli --no-default-features --features sqlite,postgres

# Run migrations
sqlx migrate run --database-url sqlite://./data/dev.db

# Create new migration
sqlx migrate add create_documents_table

# Revert last migration
sqlx migrate revert --database-url sqlite://./data/dev.db

# Check migration status
sqlx migrate info --database-url sqlite://./data/dev.db
```

## Project Structure

### Backend (Rust)

```text
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── app.rs               # Application configuration
├── error.rs             # Error handling types
├── routes.rs            # Route configuration
├── graphql/            # GraphQL implementation
│   ├── mod.rs          
│   ├── schema.rs        # GraphQL schema definition
│   ├── types.rs         # GraphQL type definitions
│   └── resolvers.rs     # GraphQL resolvers
├── handlers/           # HTTP and GraphQL handlers
│   ├── mod.rs
│   ├── business.rs      # Business logic handlers
│   ├── graphql.rs       # GraphQL endpoint handlers
│   └── http.rs          # REST API handlers
├── models/             # Data models
│   ├── mod.rs
│   ├── document.rs      # Document entity
│   ├── document_type.rs # Document type definitions
│   └── document_number_generation.rs  # Number generation rules
├── repositories/       # Database access layer
│   ├── mod.rs
│   ├── document_repository.rs
│   └── document_number_rule_repository.rs
├── services/           # Business logic
│   ├── mod.rs
│   ├── document_service.rs
│   └── document_number_generator.rs
└── migrations/         # Database schema migrations

tests/                  # Test organization
├── integration/        # Integration tests  
├── unit/              # Unit tests
│   ├── models/        # Model unit tests
│   └── repositories/  # Repository unit tests
```

### Frontend (SvelteKit)

```text
ui/
├── src/
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ui/              # Basic UI components (Button, Input, Select)
│   │   │   ├── layout/          # Layout components (Header, Navigation)
│   │   │   ├── dashboard/       # Dashboard components (Stats, SystemStatus, ActivityFeed)
│   │   │   ├── mobile/          # Mobile-responsive components
│   │   │   ├── notifications/   # Notification system components
│   │   │   └── testing/         # UI/UX testing components
│   │   ├── stores/             # Svelte stores for state management
│   │   └── utils/              # Utility functions (touch, accessibility, responsive)
│   ├── routes/                 # SvelteKit file-based routing
│   │   ├── documents/          # Document management pages
│   │   ├── organization/       # Organization management pages
│   │   └── notifications/      # Notification management pages
│   └── app.html               # HTML template
├── package.json               # Dependencies and scripts
├── tailwind.config.js         # Tailwind CSS configuration
└── svelte.config.js          # SvelteKit configuration

docs/                          # Design documentation
```

## Key Design Patterns

### Multi-Database Support

The system is designed to support SQLite → SQL Server migration:

- Use SQLx for database abstraction
- Design queries to work across both databases
- Configuration-based database switching

### Rule-Based Systems

- Document number generation follows configurable rules
- Network path generation uses template-based rules
- Historical rule support for legacy document formats

### Organizational Management

- Track department restructuring over time
- Manage personnel transfer history
- Handle complex permission inheritance

### Frontend Architecture

- **Component-based design**: Modular Svelte components with clear separation of concerns
- **State management**: Svelte stores for reactive state management across components
- **Responsive design**: Mobile-first approach with Tailwind CSS and touch gesture support
- **Type safety**: TypeScript integration throughout the frontend codebase
- **Notification system**: Multi-channel notifications (Email, Teams, In-app) with template management

## Development Phases

1. **Phase 1**: Basic functionality with JSON authentication + SQLite
2. **Phase 2**: Windows AD integration + Web interface  
3. **Phase 3**: Full feature set + SQL Server migration capability
4. **Phase 4** (✅ Completed): Complete SvelteKit UI implementation with notifications, responsive design, and testing components

## Important Implementation Notes

### Database Design

- Use UTC timestamps for all datetime fields
- Implement soft deletes with `is_active` flags for AD sync
- Support multiple document numbering formats (CTA-2508001, 技術-25001, etc.)
- Maintain audit trails for all document operations

### Security Considerations

- Implement role-based access control based on department membership
- Support confidentiality levels (internal/external, importance levels, personal data flags)
- Log all access and modification operations
- Path display control based on confidentiality levels

### File System Integration

- Network drive path validation and existence checking
- Monthly batch processes for file existence verification
- Approval document checking (filename pattern: [document_number]-審査承認.pdf)

### Frontend Implementation Guidelines

- **Component reusability**: Create generic components in `ui/src/lib/components/ui/`
- **Responsive design**: Use mobile-first approach with Tailwind CSS breakpoints
- **Type safety**: Define TypeScript interfaces for all data structures
- **State management**: Use Svelte stores for shared state across components
- **Touch support**: Implement touch gestures for mobile devices using custom TouchHandler
- **Accessibility**: Follow WCAG guidelines with proper ARIA attributes and keyboard navigation

### Performance Requirements

- Search response within 2 seconds
- Support up to 10 concurrent users
- Handle 50,000+ document records
- Batch processing: 10,000 files/hour for existence checking
- Frontend performance: Responsive UI with virtual scrolling for large datasets

## Testing Strategy

- Unit tests for business logic and utilities
- Integration tests for database operations
- API tests for GraphQL endpoints
- E2E tests with Playwright for user workflows
- Load testing for performance validation

## Deployment Considerations

- Windows Server 2019+ environment
- HTTPS-only communication
- Daily database backups
- Integrated Windows authentication
- Teams/Email notification integration

## Common Development Tasks

### Backend Development Workflow

When implementing new features:

1. Start with model definitions in `src/models/`
2. Create repository layer for data access
3. Implement business logic in services
4. Add GraphQL schema and resolvers
5. Create comprehensive tests
6. Update migration files for schema changes

When adding new document rules:

1. Update rule configuration tables
2. Implement rule validation logic
3. Add historical rule support
4. Test with legacy document formats

### Frontend Development Workflow

When adding new UI features:

1. Create reusable components in appropriate `ui/src/lib/components/` subdirectory
2. Implement TypeScript interfaces for data structures
3. Add responsive design considerations using Tailwind CSS
4. Create corresponding Svelte stores for state management
5. Implement proper accessibility features
6. Add mobile-responsive components when needed

### Working with the Notification System

The frontend includes a comprehensive notification system:

- **NotificationCenter**: Real-time notification display with unread count
- **NotificationToast**: Auto-dismissing toast notifications
- **NotificationService**: Multi-channel delivery (Email, Teams, In-app)
- **Template management**: Dynamic notification templates with variable substitution

## Documentation

Extensive design documentation is available in the `docs/` directory:

- `requirements.md`: Detailed functional requirements
- `design-basic.md`: System architecture and technical design
- Task-specific design documents for each development phase

The system handles complex organizational document management with historical data preservation and flexible rule-based processing.
