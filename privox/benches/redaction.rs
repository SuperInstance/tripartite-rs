use criterion::{black_box, criterion_group, criterion_main, Criterion};
use privox::{Redactor, RedactorConfig, TokenVault};

fn bench_basic_redaction(c: &mut Criterion) {
    let vault = TokenVault::in_memory().unwrap();
    let mut redactor = Redactor::new(RedactorConfig::default(), vault).unwrap();

    let text = "Contact: john@example.com or call 555-1234";

    c.bench_function("basic_redaction", |b| {
        b.iter(|| redactor.redact(black_box(text), black_box("session-1")))
    });
}

fn bench_multiple_patterns(c: &mut Criterion) {
    let vault = TokenVault::in_memory().unwrap();
    let mut redactor = Redactor::new(RedactorConfig::default(), vault).unwrap();

    let text = "Email: john@example.com, Phone: 555-1234, SSN: 123-45-6789, IP: 192.168.1.1";

    c.bench_function("multiple_patterns", |b| {
        b.iter(|| redactor.redact(black_box(text), black_box("session-2")))
    });
}

criterion_group!(benches, bench_basic_redaction, bench_multiple_patterns);
criterion_main!(benches);
