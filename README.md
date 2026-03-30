# Urban Twin Lab

Urban Twin Lab is a city operations simulation workspace. It combines a map-driven web interface with a Rust API for scenario state and time-series ingestion.

## Current scope
- Define simulation scenarios
- Compare baseline and proposed interventions
- Track key metrics over time

## Tech stack
- Frontend: SvelteKit + MapLibre + WebGL
- Backend: Rust API
- Data: TimescaleDB

## Repository layout
- docs/: scenario model and architecture notes
- src/web/: Svelte app
- src/api/: Rust service
- tests/: scenario and API checks
- infra/: database and deployment assets
- scripts/: data import and tooling scripts

## Quick start
1. Run Rust API from src/api.
2. Run Svelte app from src/web.
3. Use the default scenario form and inspect API responses.

## Roadmap
1. Add map layers and simulation playback.
2. Add TimescaleDB integration for metrics history.
3. Add scenario comparison and export.
4. Add performance tuning for high-volume map rendering.
