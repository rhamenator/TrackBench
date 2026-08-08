use track_bench::{Job, Task};

fn main() {
    let job = Job::new(vec![
        Task {
            id: "intake".into(),
            title: "Intake".into(),
            duration_minutes: 30,
            completed: true,
            depends_on: vec![],
        },
        Task {
            id: "work".into(),
            title: "Perform work".into(),
            duration_minutes: 120,
            completed: false,
            depends_on: vec!["intake".into()],
        },
    ])
    .unwrap();
    println!(
        "ready: {:?}; critical path: {} minutes",
        job.ready().iter().map(|task| &task.id).collect::<Vec<_>>(),
        job.critical_path_minutes().unwrap()
    );
}
