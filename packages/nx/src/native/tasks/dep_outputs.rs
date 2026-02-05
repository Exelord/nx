use crate::native::tasks::types::HashInstruction;
use crate::native::tasks::types::{Task, TaskGraph};

pub(super) fn get_dep_output(
    task: &Task,
    task_graph: &TaskGraph,
    dependent_tasks_output_files: &str,
    transitive: bool,
) -> anyhow::Result<Vec<HashInstruction>> {
    if !task_graph.dependencies.contains_key(task.id.as_str()) {
        return Ok(vec![]);
    }

    let mut inputs: Vec<HashInstruction> = vec![];
    for task_dep in &task_graph.dependencies[task.id.as_str()] {
        let child_task = &task_graph.tasks[task_dep.as_str()];

        if let Some(ref task_hash) = child_task.hash {
            // Use the child task's pre-computed hash as a proxy for its output files.
            // The cache contract guarantees: same hash -> same outputs,
            // so we can avoid reading and re-hashing output files from disk.
            inputs.push(HashInstruction::TaskHash(
                dependent_tasks_output_files.to_string(),
                task_hash.clone(),
            ));
        } else if !child_task.outputs.is_empty() {
            inputs.push(HashInstruction::TaskOutput(
                dependent_tasks_output_files.to_string(),
                child_task.outputs.clone(),
            ));
        }

        if transitive {
            inputs.extend(get_dep_output(
                child_task,
                task_graph,
                dependent_tasks_output_files,
                transitive,
            )?);
        }
    }

    Ok(inputs)
}
