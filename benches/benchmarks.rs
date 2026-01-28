use std::fs;

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use tempfile::NamedTempFile;

use crate::{action::ActionRef, parser::WorkflowFile};

fn benchmark_action_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_parsing");

    let test_cases = vec![
        "actions/checkout@v4",
        "actions/setup-node@v3",
        "docker/build-push-action@v5",
        "actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11",
    ];

    for case in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(case), case, |b, &case| {
            b.iter(|| ActionRef::parse(black_box(case)));
        });
    }

    group.finish();
}

fn benchmark_workflow_parsing(c: &mut Criterion) {
    let workflow_content = r#"
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v3
      - uses: actions/cache@v3
      - uses: docker/build-push-action@v5
      - uses: github/codeql-action/analyze@v2
  "#;

    let temp = NamedTempFile::new().unwrap();
    fs::write(temp.path(), workflow_content).unwrap();

    c.bench_function("parse_workflow", |b| {
        b.iter(|| WorkflowFile::parse(black_box(temp.path())));
    });
}

fn benchmark_large_workflow(c: &mut Criterion) {
    // Generate a large workflow with many actions
    let mut workflow_content = String::from("name: Large\non: [push]\njobs:\n");

    for i in 0..50 {
        workflow_content.push_str(&format!(
            "  job{}:\n    runs-on: ubuntu-latest\n    steps:\n",
            i
        ));
        for j in 0..10 {
            workflow_content.push_str(&format!("      - uses: actions/checkout@v{}\n", j % 4 + 1));
        }
    }

    let temp = NamedTempFile::new().unwrap();
    fs::write(temp.path(), &workflow_content).unwrap();

    c.bench_function("parse_large_workflow", |b| {
        b.iter(|| WorkflowFile::parse(black_box(temp.path())));
    });
}

criterion_group!(
    benches,
    benchmark_action_parsing,
    benchmark_workflow_parsing,
    benchmark_large_workflow
);
criterion_main!(benches);
