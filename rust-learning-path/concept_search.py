#!/usr/bin/env python3
"""
Rust Learning Path Concept Search System

This module provides functionality to search and find concept explanations
across the entire learning path, with cross-references and related concepts.
"""

import json
import re
from typing import List, Dict, Any, Optional, Set
from pathlib import Path
import argparse


class ConceptSearchEngine:
    """Search engine for Rust learning path concepts."""
    
    def __init__(self, index_file: str = "concept_index.json"):
        """Initialize the search engine with the concept index."""
        self.index_file = Path(index_file)
        self.concepts = {}
        self.cross_references = {}
        self.learning_path = {}
        self.load_index()
    
    def load_index(self):
        """Load the concept index from JSON file."""
        try:
            with open(self.index_file, 'r', encoding='utf-8') as f:
                data = json.load(f)
                self.concepts = data.get('concepts', {})
                self.cross_references = data.get('cross_references', {})
                self.learning_path = data.get('learning_path', {})
        except FileNotFoundError:
            print(f"Index file {self.index_file} not found. Please run index generation first.")
        except json.JSONDecodeError as e:
            print(f"Error parsing index file: {e}")
    
    def search_concepts(self, query: str, exact_match: bool = False) -> List[Dict[str, Any]]:
        """
        Search for concepts based on query string.
        
        Args:
            query: Search query string
            exact_match: If True, search for exact matches only
            
        Returns:
            List of matching concepts with relevance scores
        """
        query = query.lower().strip()
        results = []
        
        for concept_id, concept_data in self.concepts.items():
            score = self._calculate_relevance_score(query, concept_id, concept_data, exact_match)
            if score > 0:
                result = {
                    'id': concept_id,
                    'score': score,
                    'concept': concept_data
                }
                results.append(result)
        
        # Sort by relevance score (descending)
        results.sort(key=lambda x: x['score'], reverse=True)
        return results
    
    def _calculate_relevance_score(self, query: str, concept_id: str, concept_data: Dict, exact_match: bool) -> float:
        """Calculate relevance score for a concept based on query."""
        score = 0.0
        
        # Exact match in concept ID (highest priority)
        if exact_match:
            if query == concept_id.replace('-', ' ') or query == concept_id:
                return 100.0
            return 0.0
        
        # Title match (high priority)
        title = concept_data.get('title', '').lower()
        if query in title:
            score += 50.0
            if query == title:
                score += 25.0
        
        # Concept ID match (high priority)
        if query in concept_id.replace('-', ' '):
            score += 40.0
        
        # Keywords match (medium priority)
        keywords = concept_data.get('keywords', [])
        for keyword in keywords:
            if query in keyword.lower():
                score += 20.0
                if query == keyword.lower():
                    score += 10.0
        
        # Description match (lower priority)
        description = concept_data.get('description', '').lower()
        if query in description:
            score += 10.0
        
        # Partial word matches
        query_words = query.split()
        for word in query_words:
            if len(word) > 2:  # Ignore very short words
                if word in title:
                    score += 5.0
                if word in concept_id:
                    score += 3.0
                for keyword in keywords:
                    if word in keyword.lower():
                        score += 2.0
        
        return score
    
    def get_concept_details(self, concept_id: str) -> Optional[Dict[str, Any]]:
        """Get detailed information about a specific concept."""
        if concept_id not in self.concepts:
            return None
        
        concept = self.concepts[concept_id].copy()
        
        # Add related concepts
        related_ids = concept.get('related_concepts', [])
        concept['related_concepts_details'] = []
        for related_id in related_ids:
            if related_id in self.concepts:
                concept['related_concepts_details'].append({
                    'id': related_id,
                    'title': self.concepts[related_id].get('title', ''),
                    'level': self.concepts[related_id].get('level', ''),
                    'description': self.concepts[related_id].get('description', '')
                })
        
        return concept
    
    def find_cross_references(self, query: str) -> List[str]:
        """Find cross-referenced concepts for a query."""
        query = query.lower().strip()
        
        for category, concept_ids in self.cross_references.items():
            if query in category.lower():
                return concept_ids
        
        return []
    
    def get_concepts_by_level(self, level: str) -> List[Dict[str, Any]]:
        """Get all concepts for a specific learning level."""
        level = level.lower()
        if level not in self.learning_path:
            return []
        
        concept_ids = self.learning_path[level]
        concepts = []
        
        for concept_id in concept_ids:
            if concept_id in self.concepts:
                concepts.append({
                    'id': concept_id,
                    'concept': self.concepts[concept_id]
                })
        
        return concepts
    
    def get_concepts_by_project(self, project_name: str) -> List[Dict[str, Any]]:
        """Get all concepts used in a specific project."""
        project_name = project_name.lower()
        results = []
        
        for concept_id, concept_data in self.concepts.items():
            projects = concept_data.get('projects', [])
            for project in projects:
                if project_name in project.lower():
                    results.append({
                        'id': concept_id,
                        'concept': concept_data
                    })
                    break
        
        return results
    
    def suggest_learning_path(self, current_concept: str) -> List[Dict[str, Any]]:
        """Suggest next concepts to learn based on current concept."""
        if current_concept not in self.concepts:
            return []
        
        current_data = self.concepts[current_concept]
        current_level = current_data.get('level', '')
        
        suggestions = []
        
        # Add related concepts from same level
        related_concepts = current_data.get('related_concepts', [])
        for related_id in related_concepts:
            if related_id in self.concepts:
                related_data = self.concepts[related_id]
                suggestions.append({
                    'id': related_id,
                    'concept': related_data,
                    'reason': 'Related concept',
                    'priority': 2
                })
        
        # Add next level concepts if current level is completed
        level_order = ['basic', 'intermediate', 'advanced', 'expert']
        if current_level in level_order:
            current_index = level_order.index(current_level)
            if current_index < len(level_order) - 1:
                next_level = level_order[current_index + 1]
                next_level_concepts = self.get_concepts_by_level(next_level)
                for concept in next_level_concepts[:3]:  # Limit to first 3
                    suggestions.append({
                        'id': concept['id'],
                        'concept': concept['concept'],
                        'reason': f'Next level ({next_level})',
                        'priority': 1
                    })
        
        # Sort by priority
        suggestions.sort(key=lambda x: x['priority'])
        return suggestions[:5]  # Return top 5 suggestions


def format_search_results(results: List[Dict[str, Any]], max_results: int = 10) -> str:
    """Format search results for display."""
    if not results:
        return "No concepts found matching your query."
    
    output = []
    output.append(f"Found {len(results)} concept(s):\n")
    
    for i, result in enumerate(results[:max_results]):
        concept = result['concept']
        score = result['score']
        
        output.append(f"{i+1}. {concept['title']} (Score: {score:.1f})")
        output.append(f"   Level: {concept['level'].title()}")
        output.append(f"   Description: {concept['description']}")
        
        keywords = concept.get('keywords', [])
        if keywords:
            output.append(f"   Keywords: {', '.join(keywords[:5])}")
        
        projects = concept.get('projects', [])
        if projects:
            output.append(f"   Used in projects: {', '.join(projects[:3])}")
        
        output.append("")
    
    if len(results) > max_results:
        output.append(f"... and {len(results) - max_results} more results")
    
    return "\n".join(output)


def format_concept_details(concept_data: Dict[str, Any]) -> str:
    """Format detailed concept information for display."""
    if not concept_data:
        return "Concept not found."
    
    output = []
    output.append(f"=== {concept_data['title']} ===")
    output.append(f"Level: {concept_data['level'].title()}")
    output.append(f"Description: {concept_data['description']}")
    output.append("")
    
    keywords = concept_data.get('keywords', [])
    if keywords:
        output.append(f"Keywords: {', '.join(keywords)}")
        output.append("")
    
    projects = concept_data.get('projects', [])
    if projects:
        output.append(f"Used in projects: {', '.join(projects)}")
        output.append("")
    
    related = concept_data.get('related_concepts_details', [])
    if related:
        output.append("Related concepts:")
        for rel in related:
            output.append(f"  - {rel['title']} ({rel['level']}): {rel['description']}")
        output.append("")
    
    file_path = concept_data.get('file_path', '')
    section = concept_data.get('section', '')
    if file_path and section:
        output.append(f"Documentation: {file_path}#{section}")
    
    return "\n".join(output)


def main():
    """Command-line interface for the concept search system."""
    parser = argparse.ArgumentParser(description='Search Rust learning path concepts')
    parser.add_argument('query', nargs='?', help='Search query')
    parser.add_argument('--exact', action='store_true', help='Exact match only')
    parser.add_argument('--level', help='Filter by learning level (basic, intermediate, advanced, expert)')
    parser.add_argument('--project', help='Filter by project name')
    parser.add_argument('--details', help='Get detailed information about a specific concept')
    parser.add_argument('--suggest', help='Get learning path suggestions based on current concept')
    parser.add_argument('--cross-ref', help='Find cross-referenced concepts')
    parser.add_argument('--max-results', type=int, default=10, help='Maximum number of results to show')
    
    args = parser.parse_args()
    
    search_engine = ConceptSearchEngine()
    
    if args.details:
        concept_data = search_engine.get_concept_details(args.details)
        print(format_concept_details(concept_data))
    elif args.suggest:
        suggestions = search_engine.suggest_learning_path(args.suggest)
        if suggestions:
            print(f"Learning path suggestions after '{args.suggest}':")
            for suggestion in suggestions:
                concept = suggestion['concept']
                print(f"- {concept['title']} ({suggestion['reason']})")
                print(f"  {concept['description']}")
        else:
            print(f"No suggestions found for concept '{args.suggest}'")
    elif args.cross_ref:
        cross_refs = search_engine.find_cross_references(args.cross_ref)
        if cross_refs:
            print(f"Cross-referenced concepts for '{args.cross_ref}':")
            for concept_id in cross_refs:
                if concept_id in search_engine.concepts:
                    concept = search_engine.concepts[concept_id]
                    print(f"- {concept['title']}: {concept['description']}")
        else:
            print(f"No cross-references found for '{args.cross_ref}'")
    elif args.level:
        concepts = search_engine.get_concepts_by_level(args.level)
        if concepts:
            print(f"Concepts for {args.level} level:")
            for concept in concepts:
                print(f"- {concept['concept']['title']}: {concept['concept']['description']}")
        else:
            print(f"No concepts found for level '{args.level}'")
    elif args.project:
        concepts = search_engine.get_concepts_by_project(args.project)
        if concepts:
            print(f"Concepts used in project '{args.project}':")
            for concept in concepts:
                print(f"- {concept['concept']['title']}: {concept['concept']['description']}")
        else:
            print(f"No concepts found for project '{args.project}'")
    elif args.query:
        results = search_engine.search_concepts(args.query, args.exact)
        print(format_search_results(results, args.max_results))
    else:
        print("Please provide a search query or use --help for options")


if __name__ == "__main__":
    main()