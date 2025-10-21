# Ripgrep Core MVP Implementation Plan

## Architecture Overview

The plan implements the core ripgrep workflow through four interconnected crates:

1. **globset** - Pattern matching with glob syntax support
2. **ignore** - Directory walking with gitignore/ignore file support  
3. **regex** - Regex pattern matching via grep-matcher trait
4. **searcher** - Line-oriented search engine with configurable output

## Core Data Flow

```
Pattern → Matcher → Searcher → Walker → Files → Search → Results
```

## Implementation Strategy

### Phase 1: Core Traits and Types (matcher crate)

- Implement `Match` struct for representing byte ranges
- Implement `Matcher` trait as the core search interface
- Implement `Captures` trait for capture group support
- Add `LineTerminator` and `ByteSet` utilities
- Create `NoCaptures` and `NoError` helper types

**Key Files:**

- `crates/matcher/src/lib.rs` - Core trait definitions
- `crates/matcher/src/interpolate.rs` - String interpolation support

### Phase 2: Globset Pattern Matching

- Implement `Glob` and `GlobBuilder` for single pattern matching
- Implement `GlobSet` and `GlobSetBuilder` for multiple patterns
- Add basic optimization strategies:
  - Literal matching (exact string matches)
  - Extension matching (*.rs, *.txt)
  - Simple regex fallback
- Implement `Candidate` type for path normalization

**Key Files:**

- `crates/globset/src/lib.rs` - Main API and GlobSet implementation
- `crates/globset/src/glob.rs` - Individual glob pattern handling
- `crates/globset/src/pathutil.rs` - Path normalization utilities

### Phase 3: Regex Matcher Implementation  

- Implement `RegexMatcher` and `RegexMatcherBuilder`
- Integrate with `regex-automata` for pattern compilation
- Support basic regex features (no complex optimizations)
- Implement `RegexCaptures` for capture group extraction

**Key Files:**

- `crates/regex/src/lib.rs` - Public API
- `crates/regex/src/matcher.rs` - Core RegexMatcher implementation
- `crates/regex/src/error.rs` - Error handling

### Phase 4: Directory Walking (ignore crate)

- Implement `WalkBuilder` and `Walk` for directory traversal
- Add basic gitignore file parsing and matching
- Support `.ignore`, `.gitignore` file types
- Implement `DirEntry` for file metadata
- Add parallel walking support via `WalkParallel`

**Key Files:**

- `crates/ignore/src/lib.rs` - Main API and error types
- `crates/ignore/src/walk.rs` - Directory walking implementation
- `crates/ignore/src/gitignore.rs` - Gitignore file parsing
- `crates/ignore/src/types.rs` - File type detection

### Phase 5: Search Engine (searcher crate)

- Implement `Searcher` and `SearcherBuilder` 
- Add `Sink` trait for result handling
- Support line-oriented searching with context
- Implement basic binary detection
- Add memory mapping support for large files

**Key Files:**

- `crates/searcher/src/lib.rs` - Public API
- `crates/searcher/src/searcher/mod.rs` - Core search logic
- `crates/searcher/src/sink.rs` - Result handling interface
- `crates/searcher/src/lines.rs` - Line iteration utilities

## MVP Feature Set

### Included Features:

- Basic glob pattern matching (`*.rs`, `src/**/*.txt`)
- Regex pattern support via `regex-automata`
- Directory walking with ignore file support
- Line-oriented text search
- Basic binary file detection
- Simple parallel directory traversal
- UTF-8 text handling

### Excluded from MVP:

- Advanced globset optimization strategies (prefix/suffix matching)
- Complex regex optimizations and literal extraction
- File type detection beyond basic rules  
- Advanced binary detection algorithms
- Memory mapping optimizations
- Compression format support
- Complex capture group interpolation

## Dependencies

Core external dependencies:

- `regex-automata` - Regex engine
- `aho-corasick` - String matching algorithms  
- `walkdir` - Directory traversal
- `memchr` - Fast byte searching
- `bstr` - Byte string utilities

## Testing Strategy

Each crate will include:

- Unit tests for core functionality
- Integration tests with example patterns
- Basic performance benchmarks
- Cross-platform compatibility tests

## Success Criteria

The MVP should support:

1. Searching for regex patterns in files: `rg "pattern" path/`
2. Glob-based file filtering: `rg "pattern" --glob "*.rs"`
3. Respecting `.gitignore` and `.ignore` files
4. Basic parallel directory traversal
5. Line-oriented output with line numbers