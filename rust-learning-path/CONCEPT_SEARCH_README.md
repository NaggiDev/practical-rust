# Rust Learning Path - Concept Search & Indexing System

This system provides comprehensive search and cross-referencing capabilities for all Rust concepts covered in the learning path.

## Features

- **Full-text concept search** with relevance scoring
- **Cross-references** between related concepts
- **Learning path suggestions** based on current progress
- **Project-based filtering** to find concepts used in specific projects
- **Level-based browsing** (Basic, Intermediate, Advanced, Expert)
- **Web interface** for easy browsing and searching
- **Command-line tools** for advanced usage

## Files Overview

### Core Files

- `concept_index.json` - The main concept index database
- `concept_search.py` - Python command-line search tool
- `concept_search.html` - Web-based search interface
- `build_concept_index.py` - Script to rebuild the index from CONCEPTS.md files

### Generated Index Structure

The `concept_index.json` contains:
- **concepts**: Detailed information about each concept
- **cross_references**: Related concept groupings by category
- **learning_path**: Concepts organized by difficulty level
- **metadata**: Index statistics and timestamps

## Usage

### Web Interface (Recommended)

1. Open `concept_search.html` in your web browser
2. Use the search box to find concepts by name, keyword, or description
3. Apply filters by level, project, or use exact matching
4. Click "Cross References" to explore related concept categories
5. Click "Learning Path" to browse concepts by difficulty level

### Command Line Interface

#### Basic Search
```bash
python concept_search.py "ownership"
python concept_search.py "async programming"
```

#### Advanced Search Options
```bash
# Exact match only
python concept_search.py "ownership" --exact

# Filter by learning level
python concept_search.py --level intermediate

# Filter by project
python concept_search.py --project "web-scraper"

# Get detailed information about a concept
python concept_search.py --details "ownership-basics"

# Get learning path suggestions
python concept_search.py --suggest "ownership-basics"

# Find cross-referenced concepts
python concept_search.py --cross-ref "memory"

# Limit number of results
python concept_search.py "trait" --max-results 5
```

### Rebuilding the Index

To update the index after adding new concepts or modifying existing ones:

```bash
# Basic rebuild
python build_concept_index.py

# Verbose output with statistics
python build_concept_index.py --verbose

# Specify custom paths
python build_concept_index.py --root /path/to/learning-path --output custom_index.json
```

## Concept Index Structure

Each concept in the index contains:

```json
{
  "concept-id": {
    "title": "Human-readable title",
    "level": "basic|intermediate|advanced|expert",
    "description": "Brief description of the concept",
    "file_path": "path/to/CONCEPTS.md",
    "section": "markdown-section-id",
    "keywords": ["keyword1", "keyword2", ...],
    "related_concepts": ["related-concept-id", ...],
    "projects": ["project-name", ...]
  }
}
```

## Cross-Reference Categories

The system automatically categorizes concepts into these cross-reference groups:

- **ownership**: Ownership, borrowing, lifetimes, references
- **error-handling**: Result, Option, error types, panic handling
- **concurrency**: Threading, async/await, synchronization
- **memory**: Memory management, smart pointers, allocators
- **types**: Structs, enums, traits, generics
- **collections**: Vectors, HashMaps, iterators
- **functions**: Functions, closures, methods
- **testing**: Unit tests, integration tests, benchmarks
- **advanced**: Advanced and expert-level concepts

## Search Algorithm

The search system uses a relevance scoring algorithm that considers:

1. **Exact matches** in concept titles (highest priority)
2. **Partial matches** in concept titles
3. **Keyword matches** (medium priority)
4. **Description matches** (lower priority)
5. **Word-level matches** across all fields

Results are ranked by relevance score and can be filtered by level, project, or exact matching.

## Integration with Learning Path

### Project Integration

Concepts are automatically linked to projects based on their file locations:

- `basic/module1/calculator/` → calculator project
- `intermediate/library-management-system/` → library-management-system project
- `advanced/thread-pool/` → thread-pool project

### Learning Path Progression

The system suggests next concepts to learn based on:
- Related concepts at the same level
- Entry-level concepts from the next difficulty level
- Prerequisites and dependencies between concepts

## Extending the System

### Adding New Concepts

1. Add concepts to appropriate `CONCEPTS.md` files
2. Use standard markdown headers (## or ###) for concept sections
3. Include clear descriptions and code examples
4. Rebuild the index: `python build_concept_index.py`

### Customizing Search

The search algorithm can be customized by modifying the `_calculate_relevance_score` method in `concept_search.py`.

### Adding New Cross-Reference Categories

Edit the `build_cross_references` method in `build_concept_index.py` to add new categorization rules.

## Troubleshooting

### Index Not Loading
- Ensure `concept_index.json` exists in the same directory
- Run `python build_concept_index.py` to regenerate the index

### Search Not Working
- Check that the web interface can access `concept_index.json`
- Verify the JSON file is valid using a JSON validator

### Missing Concepts
- Ensure new concepts are added to `CONCEPTS.md` files
- Rebuild the index after adding new content
- Check that markdown headers are properly formatted

### Performance Issues
- The index is optimized for up to 1000+ concepts
- For very large learning paths, consider implementing database storage
- Web interface loads the entire index in memory for fast searching

## API Reference

### ConceptSearchEngine Class

```python
from concept_search import ConceptSearchEngine

# Initialize
engine = ConceptSearchEngine("concept_index.json")

# Search concepts
results = engine.search_concepts("ownership", exact_match=False)

# Get concept details
details = engine.get_concept_details("ownership-basics")

# Find cross-references
cross_refs = engine.find_cross_references("memory")

# Get concepts by level
concepts = engine.get_concepts_by_level("intermediate")

# Get concepts by project
concepts = engine.get_concepts_by_project("web-scraper")

# Get learning suggestions
suggestions = engine.suggest_learning_path("ownership-basics")
```

## Contributing

To contribute to the concept search system:

1. Follow the existing concept documentation format
2. Add comprehensive keywords and descriptions
3. Link related concepts appropriately
4. Test the search functionality after changes
5. Update this README if adding new features

## License

This concept search system is part of the Rust Learning Path project and follows the same license terms.