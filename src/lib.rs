use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub duration_minutes: u32,
    pub completed: bool,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Job {
    tasks: BTreeMap<String, Task>,
}

impl Job {
    pub fn new(tasks: Vec<Task>) -> Result<Self, String> {
        let tasks: BTreeMap<_, _> = tasks
            .into_iter()
            .map(|task| (task.id.clone(), task))
            .collect();
        for task in tasks.values() {
            for dependency in &task.depends_on {
                if !tasks.contains_key(dependency) {
                    return Err(format!("{} depends on unknown task {dependency}", task.id));
                }
            }
        }
        let job = Self { tasks };
        job.critical_path_minutes()?;
        Ok(job)
    }

    pub fn ready(&self) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| {
                !task.completed && task.depends_on.iter().all(|id| self.tasks[id].completed)
            })
            .collect()
    }

    pub fn critical_path_minutes(&self) -> Result<u32, String> {
        let mut memo = BTreeMap::new();
        let mut active = BTreeSet::new();
        let mut longest = 0;
        for id in self.tasks.keys() {
            longest = longest.max(self.path_to(id, &mut memo, &mut active)?);
        }
        Ok(longest)
    }

    fn path_to(
        &self,
        id: &str,
        memo: &mut BTreeMap<String, u32>,
        active: &mut BTreeSet<String>,
    ) -> Result<u32, String> {
        if let Some(value) = memo.get(id) {
            return Ok(*value);
        }
        if !active.insert(id.to_owned()) {
            return Err(format!("task dependency cycle includes {id}"));
        }
        let task = &self.tasks[id];
        let before = task
            .depends_on
            .iter()
            .map(|dependency| self.path_to(dependency, memo, active))
            .max()
            .unwrap_or(Ok(0))?;
        active.remove(id);
        let total = before + task.duration_minutes;
        memo.insert(id.to_owned(), total);
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_queue_and_critical_path_follow_dependencies() {
        let job = Job::new(vec![
            Task {
                id: "design".into(),
                title: "Design".into(),
                duration_minutes: 60,
                completed: true,
                depends_on: vec![],
            },
            Task {
                id: "build".into(),
                title: "Build".into(),
                duration_minutes: 120,
                completed: false,
                depends_on: vec!["design".into()],
            },
            Task {
                id: "test".into(),
                title: "Test".into(),
                duration_minutes: 30,
                completed: false,
                depends_on: vec!["build".into()],
            },
        ])
        .unwrap();
        assert_eq!(job.ready()[0].id, "build");
        assert_eq!(job.critical_path_minutes().unwrap(), 210);
    }
}
