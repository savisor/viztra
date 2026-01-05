# Insights Feature Lifecycle

## Overview

The insights system provides a scalable, factory-based architecture for executing dynamic queries on Parquet files. Each insight is a self-contained module that validates parameters, executes Polars queries, and returns typed results.

## Components

### Core Infrastructure

1. **`insight_trait.rs`** - Defines the `Insight` trait that all insights must implement
2. **`registry.rs`** - Stores and retrieves insights by identifier (HashMap-based)
3. **`factory.rs`** - Manages the global insight registry and provides lookup functions
4. **`validator.rs`** - Validates parameters against JSON Schema (Draft 7)
5. **`command.rs`** - Tauri command handler that orchestrates insight execution
6. **`model.rs`** - Request/Response data structures

### Insight Implementation Structure

Each insight follows this structure:
```
insights/deals/{insight_name}/
├── mod.rs      # Insight trait implementation
├── params.rs   # Parameter struct with JSON Schema
├── result.rs   # Result struct (optional, can reuse existing types)
└── query.rs    # Polars query logic
```

## Lifecycle Flow

### 1. Application Startup

**Location**: `src/lib.rs`

```rust
pub fn run() {
    // Step 1: Initialize registry (registers all insights)
    features::insights::factory::initialize_registry();
    
    // Step 2: Register Tauri command
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            execute_insight  // ← Makes command available to frontend
        ])
        .run(...)
}
```

**What happens**:
- `initialize_registry()` creates a new `InsightRegistry`
- All insights are registered via `registry.register(Insight::new())`
- Registry is stored in a `OnceLock` for thread-safe global access
- Tauri command `execute_insight` is registered

### 2. Insight Registration

**Location**: `src/features/insights/factory.rs`

```rust
pub fn initialize_registry() -> &'static InsightRegistry {
    REGISTRY.get_or_init(|| {
        let mut registry = InsightRegistry::new();
        
        // Register each insight here
        registry.register(ProfitBySymbolInsight::new());
        registry.register(FilteredEntriesInsight::new());
        
        registry
    })
}
```

**What happens**:
- Each insight is instantiated and registered
- Insight identifier (e.g., `"deals.profit_by_symbol"`) is used as the HashMap key
- Insight is wrapped in `Arc<dyn Insight>` for shared ownership

### 3. Frontend Request

**Location**: Frontend calls `useInsight` hook

```typescript
const { data, isLoading, error } = useInsight(
  "deals.filtered_entries",
  { account_number: "5043757397" },
  { autoFetch: true }
);
```

**What happens**:
- Hook constructs `InsightRequest` with `insight_id` and `parameters`
- Calls Tauri command: `invoke("execute_insight", { request })`

### 4. Command Handler Execution

**Location**: `src/features/insights/command.rs`

**Execution Order**:

```rust
#[tauri::command]
pub fn execute_insight(request: InsightRequest) -> Result<InsightResponse, String> {
    // Step 1: Lookup insight by identifier
    let insight = get_insight(&request.insight_id)?;
    
    // Step 2: Get JSON Schema from insight
    let schema = insight.parameter_schema();
    
    // Step 3: Validate parameters against schema
    ParameterValidator::validate(&schema, &request.parameters)?;
    
    // Step 4: Insight-specific validation (if any)
    insight.validate_parameters(&request.parameters)?;
    
    // Step 5: Execute the insight
    let data = insight.execute(request.parameters)?;
    
    // Step 6: Extract column names from result
    let columns = extract_columns(&data);
    
    // Step 7: Return response
    Ok(InsightResponse::success(data, columns))
}
```

### 5. Parameter Validation

**Location**: `src/features/insights/validator.rs`

**Two-stage validation**:

1. **JSON Schema Validation** (via `ParameterValidator`):
   - Compiles JSON Schema using `jsonschema` crate (Draft 7)
   - Validates parameter structure, types, and constraints
   - Returns detailed error messages for invalid parameters

2. **Insight-Specific Validation** (via `insight.validate_parameters()`):
   - Allows custom validation logic per insight
   - Typically deserializes to parameter struct to validate structure
   - Can add business logic validation

### 6. Insight Execution

**Location**: `src/features/insights/deals/{insight_name}/mod.rs`

**Execution Flow**:

```rust
impl Insight for MyInsight {
    fn execute(&self, params: Value) -> Result<Value, AppError> {
        // Step 1: Deserialize parameters to typed struct
        let params: MyParams = serde_json::from_value(params)?;
        
        // Step 2: Execute query (reads Parquet, applies filters, aggregates)
        let results = execute_query(&params)?;
        
        // Step 3: Serialize results to JSON
        let json_results: Vec<Value> = results
            .into_iter()
            .map(|r| serde_json::to_value(r))
            .collect();
        
        // Step 4: Return as JSON array
        Ok(Value::Array(json_results))
    }
}
```

### 7. Query Execution

**Location**: `src/features/insights/deals/{insight_name}/query.rs`

**Typical Pattern**:

```rust
pub fn execute_query(params: &MyParams) -> Result<Vec<MyResult>, AppError> {
    // Step 1: Determine files to read
    let files = get_files(params)?;
    
    // Step 2: Read Parquet files as LazyFrames
    let lf = LazyFrame::scan_parquet(&file_path, ...)?;
    
    // Step 3: Apply filters at query level (before collect)
    let filtered = lf.filter(col("entry").neq(lit(0)));
    
    // Step 4: Apply aggregations, sorting, etc.
    let result = filtered
        .group_by([col("symbol")])
        .agg([...])
        .sort([...])
        .collect()?;
    
    // Step 5: Convert DataFrame to result structs
    let results = convert_to_structs(result)?;
    
    Ok(results)
}
```

**Important**: Filters should be applied at the LazyFrame level (before `.collect()`) for optimal performance. This allows Polars to push filters down to the Parquet reader.

### 8. Response Construction

**Location**: `src/features/insights/command.rs`

```rust
// Extract column names from first result object
let columns = extract_columns(&data);

// Create success response
InsightResponse::success(data, columns)
```

**Response Structure**:
```rust
InsightResponse {
    success: true,
    data: Some(Value::Array([...])),  // Array of result objects
    error: None,
    columns: vec!["symbol", "total_profit", ...]  // For table rendering
}
```

### 9. Frontend Receives Response

**Location**: Frontend `useInsight` hook

```typescript
if (response.success && response.data) {
    setData(response.data as TResult[]);
    setColumns(response.columns);
} else {
    setError(response.error || "Unknown error occurred");
}
```

## Creating a New Insight

### Step 1: Create Module Structure

```
src-tauri/src/features/insights/deals/my_insight/
├── mod.rs
├── params.rs
├── result.rs (optional)
└── query.rs
```

### Step 2: Define Parameters (`params.rs`)

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MyInsightParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    pub min_value: Option<f64>,
}
```

### Step 3: Define Result (`result.rs`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyInsightResult {
    pub symbol: String,
    pub total: f64,
}
```

### Step 4: Implement Query (`query.rs`)

```rust
pub fn execute_query(params: &MyInsightParams) -> Result<Vec<MyInsightResult>, AppError> {
    // Polars query logic here
    // Apply filters at LazyFrame level
    // Return typed results
}
```

### Step 5: Implement Insight Trait (`mod.rs`)

```rust
impl Insight for MyInsight {
    fn identifier(&self) -> &'static str {
        "deals.my_insight"
    }
    
    fn parameter_schema(&self) -> Value {
        let schema = schemars::schema_for!(MyInsightParams);
        serde_json::to_value(schema).unwrap()
    }
    
    fn validate_parameters(&self, params: &Value) -> Result<(), AppError> {
        let _: MyInsightParams = serde_json::from_value(params.clone())?;
        Ok(())
    }
    
    fn execute(&self, params: Value) -> Result<Value, AppError> {
        let params: MyInsightParams = serde_json::from_value(params)?;
        let results = execute_query(&params)?;
        let json_results: Vec<Value> = results
            .into_iter()
            .map(|r| serde_json::to_value(r).unwrap())
            .collect();
        Ok(Value::Array(json_results))
    }
}
```

### Step 6: Register Insight

**Location**: `src/features/insights/factory.rs`

```rust
pub fn initialize_registry() -> &'static InsightRegistry {
    REGISTRY.get_or_init(|| {
        let mut registry = InsightRegistry::new();
        registry.register(MyInsight::new());  // ← Add here
        registry
    })
}
```

**Location**: `src/features/insights/deals/mod.rs`

```rust
pub mod my_insight;  // ← Add module
```

## Key Principles

1. **Filter at Query Level**: Always apply filters on `LazyFrame` before `.collect()` for optimal performance
2. **Type Safety**: Use typed parameter and result structs, serialize to JSON only at boundaries
3. **Validation**: Two-stage validation (JSON Schema + insight-specific)
4. **Modularity**: Each insight is self-contained in its own folder
5. **Factory Pattern**: Dynamic dispatch via registry lookup by identifier
6. **Error Handling**: All errors return `InsightResponse` with error message, never panic

## File Locations Summary

```
src-tauri/src/features/insights/
├── mod.rs              # Module exports
├── insight_trait.rs    # Insight trait definition
├── registry.rs         # Insight storage and lookup
├── factory.rs          # Registry initialization and management
├── validator.rs        # JSON Schema parameter validation
├── command.rs          # Tauri command handler
├── model.rs            # Request/Response types
└── deals/              # Domain-specific insights
    ├── mod.rs
    ├── profit_by_symbol/
    └── filtered_entries/
```

