//! Batch insight execution with concurrent processing

use crate::features::insights::command::execute_insight;
use crate::features::insights::model::{BatchInsightItem, BatchInsightRequest, BatchInsightResponse};
use futures::future;

/// Tauri command: execute_batch_insights
/// Executes multiple insights concurrently and returns all results
/// 
/// # Arguments
/// * `request` - BatchInsightRequest containing a list of InsightRequest items
/// 
/// Returns BatchInsightResponse with results for each insight (partial success allowed)
#[tauri::command]
pub async fn execute_batch_insights(
    request: BatchInsightRequest,
) -> Result<BatchInsightResponse, String> {
    let requests = request.requests.clone();

    // Spawn concurrent tasks for each insight execution
    // Since execute_insight is synchronous, we run it in a blocking task
    let tasks: Vec<_> = request.requests
        .into_iter()
        .map(|req| {
            tokio::spawn(async move {
                // Execute the insight in a blocking task since it's synchronous
                tokio::task::spawn_blocking(move || {
                    execute_insight(req)
                })
                .await
                .map_err(|e| format!("Blocking task failed: {}", e))?
            })
        })
        .collect();

    // Wait for all tasks to complete
    let results = future::join_all(tasks).await;

    // Collect results, handling errors per insight
    let batch_items: Vec<BatchInsightItem> = results
        .into_iter()
        .enumerate()
        .map(|(index, task_result)| {
            let insight_id = requests
                .get(index)
                .map(|r| r.insight_id.clone())
                .unwrap_or_else(|| format!("unknown_{}", index));

            match task_result {
                Ok(Ok(response)) => {
                    // Successfully executed insight
                    if response.success {
                        BatchInsightItem::success(
                            insight_id,
                            response.data.unwrap_or(serde_json::Value::Array(vec![])),
                            response.columns,
                        )
                    } else {
                        BatchInsightItem::error(
                            insight_id,
                            response.error.unwrap_or_else(|| "Unknown error".to_string()),
                        )
                    }
                }
                Ok(Err(e)) => {
                    // Tauri command returned an error
                    BatchInsightItem::error(insight_id, e)
                }
                Err(e) => {
                    // Join handle error
                    BatchInsightItem::error(insight_id, format!("Join error: {}", e))
                }
            }
        })
        .collect();

    Ok(BatchInsightResponse {
        results: batch_items,
    })
}

