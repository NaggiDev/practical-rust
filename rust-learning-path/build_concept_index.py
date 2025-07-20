#!/usr/bin/env python3
"""
Build Concept Index Script

This script scans all CONCEPTS.md files in the Rust learning path
and builds a comprehensive searchable index with cross-references.
"""

import json
import re
from pathlib import Path
from typing import Dict, List, Set, Any
import argparse


class ConceptIndexBuilder:
    """Builds a comprehensive concept index from all CONCEPTS.md files."""
    
    def __init__(self, root_path: str = "."):
        self.root_path = Path(root_path)
        self.concepts = {}
        self.cross_references = {}
        self.learning_path = {
            'basic': [],
            'intermediate': [],
            'advanced': [],
            'expert': []
        }
        self.project_concepts = {}
        
    def scan_concepts_files(self) -> List[Path]:
        """Find all CONCEPTS.md files in the learning path."""
        concepts_files = []
        
        # Find all CONCEPTS.md files recursively
        for concepts_file in self.root_path.rglob("CONCEPTS.md"):
            concepts_files.append(concepts_file)
            
        return concepts_files
    
    def extract_concepts_from_file(self, file_path: Path) -> Dict[str, Any]:
        """Extract concept information from a CONCEPTS.md file."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
            return {}
        
        concepts = {}
        
        # Determine level from file path
        level = self._determine_level_from_path(file_path)
        
        # Extract concepts using markdown headers
        concept_sections = self._parse_markdown_sections(content)
        
        for section in concept_sections:
            concept_id = self._generate_concept_id(section['title'])
            if concept_id:
                concept_data = {
                    'title': section['title'],
                    'level': level,
                    'description': self._extract_description(section['content']),
                    'file_path': str(file_path.relative_to(self.root_path)),
                    'section': concept_id,
                    'keywords': self._extract_keywords(section['content']),
                    'related_concepts': self._extract_related_concepts(section['content']),
                    'projects': self._determine_projects_from_path(file_path)
                }
                concepts[concept_id] = concept_data
        
        return concepts
    
    def _determine_level_from_path(self, file_path: Path) -> str:
        """Determine the learning level from the file path."""
        path_str = str(file_path).lower()
        
        if 'basic' in path_str:
            return 'basic'
        elif 'intermediate' in path_str:
            return 'intermediate'
        elif 'advanced' in path_str:
            return 'advanced'
        elif 'expert' in path_str:
            return 'expert'
        else:
            # Default based on main CONCEPTS.md structure
            return 'basic'
    
    def _determine_projects_from_path(self, file_path: Path) -> List[str]:
        """Determine which projects use concepts from this file."""
        projects = []
        path_parts = file_path.parts
        
        # Extract project names from path
        project_indicators = [
            'calculator', 'file-explorer', 'text-processor', 'todo-app',
            'library-management-system', 'cli-database-tool', 'custom-data-structure',
            'multi-threaded-web-scraper', 'thread-pool', 'c-library-binding',
            'custom-memory-allocator', 'dsl-project', 'async-network-server',
            'compiler-plugin', 'custom-runtime', 'high-performance-data-processing',
            'capstone-project', 'capstone-distributed-analysis'
        ]
        
        for part in path_parts:
            for indicator in project_indicators:
                if indicator in part.lower():
                    projects.append(indicator)
                    break
        
        return projects
    
    def _parse_markdown_sections(self, content: str) -> List[Dict[str, str]]:
        """Parse markdown content into sections based on headers."""
        sections = []
        
        # Split by headers (### or ##)
        header_pattern = r'^(#{2,4})\s+(.+)$'
        lines = content.split('\n')
        
        current_section = None
        current_content = []
        
        for line in lines:
            header_match = re.match(header_pattern, line)
            if header_match:
                # Save previous section
                if current_section:
                    sections.append({
                        'title': current_section,
                        'content': '\n'.join(current_content)
                    })
                
                # Start new section
                current_section = header_match.group(2).strip()
                current_content = []
            else:
                if current_section:
                    current_content.append(line)
        
        # Save last section
        if current_section:
            sections.append({
                'title': current_section,
                'content': '\n'.join(current_content)
            })
        
        return sections
    
    def _generate_concept_id(self, title: str) -> str:
        """Generate a concept ID from the title."""
        if not title:
            return ""
        
        # Convert to lowercase and replace spaces/special chars with hyphens
        concept_id = re.sub(r'[^\w\s-]', '', title.lower())
        concept_id = re.sub(r'[-\s]+', '-', concept_id)
        concept_id = concept_id.strip('-')
        
        return concept_id
    
    def _extract_description(self, content: str) -> str:
        """Extract the first meaningful paragraph as description."""
        lines = content.strip().split('\n')
        
        for line in lines:
            line = line.strip()
            if line and not line.startswith('#') and not line.startswith('```') and len(line) > 20:
                # Clean up the line
                line = re.sub(r'\*\*(.+?)\*\*', r'\1', line)  # Remove bold
                line = re.sub(r'\*(.+?)\*', r'\1', line)      # Remove italic
                line = re.sub(r'`(.+?)`', r'\1', line)        # Remove code
                return line[:200] + ('...' if len(line) > 200 else '')
        
        return "No description available."
    
    def _extract_keywords(self, content: str) -> List[str]:
        """Extract keywords from the content."""
        keywords = set()
        
        # Extract code keywords (things in backticks)
        code_keywords = re.findall(r'`([^`]+)`', content)
        for keyword in code_keywords:
            if len(keyword) < 30 and not keyword.startswith('http'):
                keywords.add(keyword.lower())
        
        # Extract common Rust terms
        rust_terms = [
            'ownership', 'borrowing', 'lifetime', 'trait', 'impl', 'struct', 'enum',
            'match', 'option', 'result', 'vec', 'string', 'slice', 'reference',
            'mutable', 'immutable', 'async', 'await', 'thread', 'mutex', 'arc',
            'box', 'rc', 'refcell', 'unsafe', 'macro', 'generic', 'closure',
            'iterator', 'collect', 'map', 'filter', 'fold', 'unwrap', 'expect'
        ]
        
        content_lower = content.lower()
        for term in rust_terms:
            if term in content_lower:
                keywords.add(term)
        
        return sorted(list(keywords))[:10]  # Limit to 10 keywords
    
    def _extract_related_concepts(self, content: str) -> List[str]:
        """Extract related concepts mentioned in the content."""
        related = set()
        
        # Look for concept references in the content
        concept_patterns = [
            r'see also:?\s*([^.\n]+)',
            r'related to:?\s*([^.\n]+)',
            r'builds on:?\s*([^.\n]+)',
            r'requires:?\s*([^.\n]+)'
        ]
        
        for pattern in concept_patterns:
            matches = re.findall(pattern, content, re.IGNORECASE)
            for match in matches:
                # Extract individual concepts
                concepts = re.split(r'[,;]', match)
                for concept in concepts:
                    concept = concept.strip()
                    if concept:
                        concept_id = self._generate_concept_id(concept)
                        if concept_id:
                            related.add(concept_id)
        
        return sorted(list(related))[:5]  # Limit to 5 related concepts
    
    def build_cross_references(self):
        """Build cross-reference mappings between concepts."""
        self.cross_references = {
            'ownership': [],
            'error-handling': [],
            'concurrency': [],
            'memory': [],
            'types': [],
            'collections': [],
            'functions': [],
            'testing': [],
            'advanced': []
        }
        
        # Categorize concepts
        for concept_id, concept_data in self.concepts.items():
            keywords = concept_data.get('keywords', [])
            title_lower = concept_data['title'].lower()
            
            # Ownership concepts
            if any(kw in keywords for kw in ['ownership', 'borrowing', 'reference', 'lifetime']):
                self.cross_references['ownership'].append(concept_id)
            
            # Error handling concepts
            if any(kw in keywords for kw in ['result', 'option', 'error', 'unwrap', 'expect']):
                self.cross_references['error-handling'].append(concept_id)
            
            # Concurrency concepts
            if any(kw in keywords for kw in ['thread', 'async', 'await', 'mutex', 'arc']):
                self.cross_references['concurrency'].append(concept_id)
            
            # Memory concepts
            if any(kw in keywords for kw in ['box', 'rc', 'heap', 'stack', 'allocator']):
                self.cross_references['memory'].append(concept_id)
            
            # Type concepts
            if any(kw in keywords for kw in ['struct', 'enum', 'trait', 'generic']):
                self.cross_references['types'].append(concept_id)
            
            # Collection concepts
            if any(kw in keywords for kw in ['vec', 'hashmap', 'iterator', 'collect']):
                self.cross_references['collections'].append(concept_id)
            
            # Function concepts
            if any(kw in keywords for kw in ['closure', 'fn', 'impl']):
                self.cross_references['functions'].append(concept_id)
            
            # Testing concepts
            if 'test' in keywords or 'testing' in title_lower:
                self.cross_references['testing'].append(concept_id)
            
            # Advanced concepts
            if concept_data['level'] in ['advanced', 'expert']:
                self.cross_references['advanced'].append(concept_id)
    
    def build_learning_path(self):
        """Build the learning path structure."""
        for concept_id, concept_data in self.concepts.items():
            level = concept_data['level']
            if level in self.learning_path:
                self.learning_path[level].append(concept_id)
        
        # Sort concepts within each level
        for level in self.learning_path:
            self.learning_path[level].sort()
    
    def build_index(self) -> Dict[str, Any]:
        """Build the complete concept index."""
        print("Scanning for CONCEPTS.md files...")
        concepts_files = self.scan_concepts_files()
        print(f"Found {len(concepts_files)} CONCEPTS.md files")
        
        # Extract concepts from each file
        for file_path in concepts_files:
            print(f"Processing {file_path}")
            file_concepts = self.extract_concepts_from_file(file_path)
            self.concepts.update(file_concepts)
        
        print(f"Extracted {len(self.concepts)} concepts")
        
        # Build cross-references and learning path
        self.build_cross_references()
        self.build_learning_path()
        
        # Create the final index structure
        index = {
            'concepts': self.concepts,
            'cross_references': self.cross_references,
            'learning_path': self.learning_path,
            'metadata': {
                'total_concepts': len(self.concepts),
                'files_processed': len(concepts_files),
                'last_updated': self._get_timestamp()
            }
        }
        
        return index
    
    def _get_timestamp(self) -> str:
        """Get current timestamp."""
        from datetime import datetime
        return datetime.now().isoformat()
    
    def save_index(self, index: Dict[str, Any], output_file: str = "concept_index.json"):
        """Save the index to a JSON file."""
        output_path = self.root_path / output_file
        
        try:
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(index, f, indent=2, ensure_ascii=False)
            print(f"Index saved to {output_path}")
        except Exception as e:
            print(f"Error saving index: {e}")


def main():
    """Main function for command-line usage."""
    parser = argparse.ArgumentParser(description='Build concept index for Rust learning path')
    parser.add_argument('--root', default='.', help='Root directory of the learning path')
    parser.add_argument('--output', default='concept_index.json', help='Output file name')
    parser.add_argument('--verbose', '-v', action='store_true', help='Verbose output')
    
    args = parser.parse_args()
    
    builder = ConceptIndexBuilder(args.root)
    index = builder.build_index()
    builder.save_index(index, args.output)
    
    if args.verbose:
        print("\nIndex Statistics:")
        print(f"Total concepts: {index['metadata']['total_concepts']}")
        print(f"Files processed: {index['metadata']['files_processed']}")
        print(f"Cross-reference categories: {len(index['cross_references'])}")
        print(f"Learning levels: {len(index['learning_path'])}")
        
        print("\nConcepts by level:")
        for level, concepts in index['learning_path'].items():
            print(f"  {level}: {len(concepts)} concepts")


if __name__ == "__main__":
    main()