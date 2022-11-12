use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion};
use rand;
use rand::distributions::{Alphanumeric, DistString};
use regex::Regex;

// The number of runs
const SAMPLE_SIZE: usize = 10_000;

// The size of the input
const PARAM_SIZE: usize = 500;

fn bench_regex(group: &mut BenchmarkGroup<WallTime>, id: &str, regex: &Regex, input: &str) {
    group.bench_function(BenchmarkId::new(id, PARAM_SIZE), |bencher| {
        bencher.iter(|| regex.captures(&input));
    });
}

fn bench_regex_control(group: &mut BenchmarkGroup<WallTime>, id: &str, regex: &Regex, input: &str) {
    group.bench_function(BenchmarkId::new(id, PARAM_SIZE), |bencher| {
        bencher.iter(|| regex.is_match(&input));
    });
}

fn regex(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let re_normal = Regex::new(r"'([^']*)'").expect("A single-quoted string regex");
    let re_possessive = Regex::new(r"'([^']*+)'").expect("A single-quoted string regex");
    let re_non_greedy = Regex::new(r"'(.*?)'").expect("A single-quoted string regex");

    let inner_str = Alphanumeric.sample_string(&mut rng, PARAM_SIZE);
    let pos_str = format!("'{inner_str}'");
    let neg_str = format!("'{inner_str}");

    {
        let mut pos_group = c.benchmark_group("capturing regexes that succeed");
        pos_group.sample_size(SAMPLE_SIZE);
        bench_regex(&mut pos_group, "normal", &re_normal, &pos_str);
        bench_regex(&mut pos_group, "possessive", &re_possessive, &pos_str);
        bench_regex(&mut pos_group, "non-greedy", &re_non_greedy, &pos_str);
    }

    {
        let mut neg_group = c.benchmark_group("capturing regexes that fail");
        neg_group.sample_size(SAMPLE_SIZE);
        bench_regex(&mut neg_group, "normal", &re_normal, &neg_str);
        bench_regex(&mut neg_group, "possessive", &re_possessive, &neg_str);
        bench_regex(&mut neg_group, "non-greedy", &re_non_greedy, &neg_str);
    }

    let re_normal = Regex::new(r"'[^']*'").expect("A single-quoted string regex");
    let re_possessive = Regex::new(r"'[^']*+'").expect("A single-quoted string regex");
    let re_non_greedy = Regex::new(r"'.*?'").expect("A single-quoted string regex");

    {
        let mut pos_ctrl_group = c.benchmark_group("non-capturing regexes that succeed");
        pos_ctrl_group.sample_size(SAMPLE_SIZE);
        bench_regex_control(&mut pos_ctrl_group, "normal", &re_normal, &pos_str);
        bench_regex_control(&mut pos_ctrl_group, "possessive", &re_possessive, &pos_str);
        bench_regex_control(&mut pos_ctrl_group, "non-greedy", &re_non_greedy, &pos_str);
    }

    {
        let mut neg_ctrl_group = c.benchmark_group("non-capturing regexes that fail");
        neg_ctrl_group.sample_size(SAMPLE_SIZE);
        bench_regex_control(&mut neg_ctrl_group, "normal", &re_normal, &neg_str);
        bench_regex_control(&mut neg_ctrl_group, "possessive", &re_possessive, &neg_str);
        bench_regex_control(&mut neg_ctrl_group, "non-greedy", &re_non_greedy, &neg_str);
    }
}

criterion_group!(benches, regex);
criterion_main!(benches);
