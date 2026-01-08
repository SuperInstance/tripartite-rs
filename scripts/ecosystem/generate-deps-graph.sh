#!/bin/bash
# generate-deps-graph.sh
# Generate dependency graphs in multiple formats

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUTPUT_DIR="$REPO_ROOT/docs/diagrams"

echo "==================================="
echo "Dependency Graph Generator"
echo "==================================="
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found"
    exit 1
fi

echo "Generating dependency graphs..."
echo ""

# Generate Mermaid graph
echo "1. Generating Mermaid graph..."

cat > "$OUTPUT_DIR/dependency-graph.mermaid" << 'EOF'
graph TD
    %% Styles
    classDef synesis fill:#0066cc,stroke:#004499,color:#fff
    classDef external fill:#666,stroke:#444,color:#fff

    %% Nodes
    CLI[synesis-cli]
    CORE[synesis-core]
    PRIVACY[synesis-privacy]
    KNOWLEDGE[synesis-knowledge]
    MODELS[synesis-models]
    CLOUD[synesis-cloud]

    %% Internal dependencies
    CLI --> CORE
    CLI --> KNOWLEDGE
    CLI --> PRIVACY
    CLI --> MODELS
    CLI --> CLOUD

    CORE --> PRIVACY
    CORE --> KNOWLEDGE
    CORE --> MODELS

    CLOUD --> PRIVACY
    CLOUD --> CORE

    KNOWLEDGE --> PRIVACY

    %% External dependencies
    CORE -->|tokio| TOKIO[Tokio]
    PRIVACY -->|rusqlite| SQLITE[SQLite]
    KNOWLEDGE -->|notify| NOTIFY[notify]
    CLOUD -->|quinn| QUINN[Quinn]

    %% Apply styles
    class CLI,CORE,PRIVACY,KNOWLEDGE,MODELS,CLOUD synesis
    class TOKIO,SQLITE,NOTIFY,QUINN external
EOF

echo "   Created: $OUTPUT_DIR/dependency-graph.mermaid"

# Generate PlantUML graph
echo "2. Generating PlantUML graph..."

cat > "$OUTPUT_DIR/dependency-graph.puml" << 'EOF'
@startuml Dependency Graph

!define SYNESIS #0066CC
!define EXTERNAL #666666

package "SuperInstance Ecosystem" {
  [synesis-cli] as CLI SYNESIS
  [synesis-core] as CORE SYNESIS
  [synesis-privacy] as PRIVACY SYNESIS
  [synesis-knowledge] as KNOWLEDGE SYNESIS
  [synesis-models] as MODELS SYNESIS
  [synesis-cloud] as CLOUD SYNESIS
}

package "External Dependencies" {
  [tokio] as TOKIO EXTERNAL
  [rusqlite] as SQLITE EXTERNAL
  [notify] as NOTIFY EXTERNAL
  [quinn] as QUINN EXTERNAL
}

CLI --> CORE
CLI --> KNOWLEDGE
CLI --> PRIVACY
CLI --> MODELS
CLI --> CLOUD

CORE --> PRIVACY
CORE --> KNOWLEDGE
CORE --> MODELS

CLOUD --> PRIVACY
CLOUD --> CORE

KNOWLEDGE --> PRIVACY

CORE ..> TOKIO : uses
PRIVACY ..> SQLITE : uses
KNOWLEDGE ..> NOTIFY : uses
CLOUD ..> QUINN : uses

@enduml
EOF

echo "   Created: $OUTPUT_DIR/dependency-graph.puml"

# Generate DOT graph (GraphViz)
echo "3. Generating DOT graph..."

cat > "$OUTPUT_DIR/dependency-graph.dot" << 'EOF'
digraph SuperInstanceEcosystem {
    rankdir=TB;
    node [shape=box, style=rounded];

    // Internal nodes
    cli [label="synesis-cli", fillcolor="#0066cc", style="filled,rounded", fontcolor="white"];
    core [label="synesis-core", fillcolor="#0066cc", style="filled,rounded", fontcolor="white"];
    privacy [label="synesis-privacy", fillcolor="#0066cc", style="filled,rounded", fontcolor="white"];
    knowledge [label="synesis-knowledge", fillcolor="#0066cc", style="filled,rounded", fontcolor="white"];
    models [label="synesis-models", fillcolor="#0066cc", style="filled,rounded", fontcolor="white"];
    cloud [label="synesis-cloud", fillcolor="#0066cc", style="filled,rounded", fontcolor="white"];

    // External nodes
    tokio [label="tokio", fillcolor="#666666", style="filled,rounded", fontcolor="white"];
    rusqlite [label="rusqlite", fillcolor="#666666", style="filled,rounded", fontcolor="white"];
    notify [label="notify", fillcolor="#666666", style="filled,rounded", fontcolor="white"];
    quinn [label="quinn", fillcolor="#666666", style="filled,rounded", fontcolor="white"];

    // Internal dependencies
    cli -> core;
    cli -> knowledge;
    cli -> privacy;
    cli -> models;
    cli -> cloud;

    core -> privacy;
    core -> knowledge;
    core -> models;

    cloud -> privacy;
    cloud -> core;

    knowledge -> privacy;

    // External dependencies
    core -> tokio [style=dashed, label="uses"];
    privacy -> rusqlite [style=dashed, label="uses"];
    knowledge -> notify [style=dashed, label="uses"];
    cloud -> quinn [style=dashed, label="uses"];
}
EOF

echo "   Created: $OUTPUT_DIR/dependency-graph.dot"

# Generate ASCII art graph for quick reference
echo "4. Generating ASCII graph..."

cat > "$OUTPUT_DIR/dependency-graph.txt" << 'EOF'
SuperInstance Ecosystem - Dependency Graph
==========================================

Internal Dependencies:
-----------------------

    ┌─────────────┐
    │ synesis-cli│
    └──────┬──────┘
           │
     ┌─────┼─────┬─────┬─────┐
     │     │     │     │     │
     ▼     ▼     ▼     ▼     ▼
┌────────┐ ┌──────────┐ ┌──────────┐
│  CLI   │ │  synesis │ │ synesis  │
│        │ │  -core   │ │ -privacy │
└────────┘ └────┬─────┘ └──────────┘
                │
      ┌─────────┼─────────┐
      │         │         │
      ▼         ▼         ▼
 ┌────────┐ ┌────────┐ ┌────────┐
 │ synesis│ │ synesis│ │ synesis│
 │-knowle │ │ -models │ │ -cloud │
 │  dge   │ └────────┘ └────────┘
 └────────┘
    │
    ▼
┌────────────┐
│ synesis    │
│ -privacy   │
└────────────┘

External Dependencies:
-----------------------

synesis-core     → tokio (async runtime)
synesis-privacy  → rusqlite (database)
synesis-knowledge → notify (file watching)
synesis-cloud    → quinn (QUIC protocol)

Legend:
  ───  Direct dependency
  ───. Uses (external)
EOF

echo "   Created: $OUTPUT_DIR/dependency-graph.txt"

# Generate usage graph
echo "5. Generating usage graph..."

cat > "$OUTPUT_DIR/usage-graph.mermaid" << 'EOF'
graph LR
    %% Styles
    classDef user fill:#00cc66,stroke:#009944,color:#fff
    classDef tool fill:#0066cc,stroke:#004499,color:#fff

    %% Nodes
    ENDUSER[End User]
    DEV[Developer]
    CLI[CLI Tool]

    ENDUSER -->|uses| CLI
    DEV -->|integrates| CORE[synesis-core]
    DEV -->|integrates| PRIVACY[synesis-privacy]
    DEV -->|integrates| KNOWLEDGE[synesis-knowledge]

    CLI -->|uses| CORE
    CLI -->|uses| PRIVACY
    CLI -->|uses| KNOWLEDGE

    %% Apply styles
    class ENDUSER,DEV user
    class CLI,CORE,PRIVACY,KNOWLEDGE tool
EOF

echo "   Created: $OUTPUT_DIR/usage-graph.mermaid"

echo ""
echo "==================================="
echo "Graph Generation Complete"
echo "==================================="
echo ""
echo "Generated files:"
echo "  ✓ Mermaid:   $OUTPUT_DIR/dependency-graph.mermaid"
echo "  ✓ PlantUML:  $OUTPUT_DIR/dependency-graph.puml"
echo "  ✓ DOT:       $OUTPUT_DIR/dependency-graph.dot"
echo "  ✓ ASCII:     $OUTPUT_DIR/dependency-graph.txt"
echo "  ✓ Usage:     $OUTPUT_DIR/usage-graph.mermaid"
echo ""
echo "Viewing graphs:"
echo "  - Mermaid: Paste into https://mermaid.live"
echo "  - PlantUML: Paste into http://www.plantuml.com/plantuml"
echo "  - DOT: Use GraphViz: dot -Tpng dependency-graph.dot -o graph.png"
echo "  - ASCII: View in text editor or cat"
echo ""
echo "✅ Graph generation complete!"
