use std::{
    collections::BTreeMap,
    env, fs,
    io::Write as _,
    path::Path,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{bail, format_err};
use xshell::{cmd, Shell};

use crate::flags;

type Unit = String;

impl flags::Metrics {
    pub(crate) fn run(self, sh: &Shell) -> anyhow::Result<()> {
        let mut metrics = Metrics::new(sh)?;
        if !self.dry_run {
            sh.remove_path("./target/release")?;
        }
        if !Path::new("./target/crablangc-perf").exists() {
            sh.create_dir("./target/crablangc-perf")?;
            cmd!(sh, "git clone https://github.com/crablang/crablangc-perf.git ./target/crablangc-perf")
                .run()?;
        }
        {
            let _d = sh.push_dir("./target/crablangc-perf");
            let revision = &metrics.perf_revision;
            cmd!(sh, "git reset --hard {revision}").run()?;
        }

        let _env = sh.push_env("RA_METRICS", "1");

        {
            // https://github.com/crablang/crablang-analyzer/issues/9997
            let _d = sh.push_dir("target/crablangc-perf/collector/benchmarks/webrender");
            cmd!(sh, "cargo update -p url --precise 1.6.1").run()?;
        }
        metrics.measure_build(sh)?;
        metrics.measure_analysis_stats_self(sh)?;
        metrics.measure_analysis_stats(sh, "ripgrep")?;
        metrics.measure_analysis_stats(sh, "webrender")?;
        metrics.measure_analysis_stats(sh, "diesel/diesel")?;

        if !self.dry_run {
            let _d = sh.push_dir("target");
            let metrics_token = env::var("METRICS_TOKEN").unwrap();
            cmd!(
                sh,
                "git clone --depth 1 https://{metrics_token}@github.com/crablang-analyzer/metrics.git"
            )
            .run()?;

            {
                let mut file =
                    fs::File::options().append(true).open("target/metrics/metrics.json")?;
                writeln!(file, "{}", metrics.json())?;
            }

            let _d = sh.push_dir("metrics");
            cmd!(sh, "git add .").run()?;
            cmd!(sh, "git -c user.name=Bot -c user.email=dummy@example.com commit --message 📈")
                .run()?;
            cmd!(sh, "git push origin master").run()?;
        }
        eprintln!("{metrics:#?}");
        Ok(())
    }
}

impl Metrics {
    fn measure_build(&mut self, sh: &Shell) -> anyhow::Result<()> {
        eprintln!("\nMeasuring build");
        cmd!(sh, "cargo fetch").run()?;

        let time = Instant::now();
        cmd!(sh, "cargo build --release --package crablang-analyzer --bin crablang-analyzer").run()?;
        let time = time.elapsed();
        self.report("build", time.as_millis() as u64, "ms".into());
        Ok(())
    }
    fn measure_analysis_stats_self(&mut self, sh: &Shell) -> anyhow::Result<()> {
        self.measure_analysis_stats_path(sh, "self", ".")
    }
    fn measure_analysis_stats(&mut self, sh: &Shell, bench: &str) -> anyhow::Result<()> {
        self.measure_analysis_stats_path(
            sh,
            bench,
            &format!("./target/crablangc-perf/collector/benchmarks/{bench}"),
        )
    }
    fn measure_analysis_stats_path(
        &mut self,
        sh: &Shell,
        name: &str,
        path: &str,
    ) -> anyhow::Result<()> {
        eprintln!("\nMeasuring analysis-stats/{name}");
        let output =
            cmd!(sh, "./target/release/crablang-analyzer -q analysis-stats --memory-usage {path}")
                .read()?;
        for (metric, value, unit) in parse_metrics(&output) {
            self.report(&format!("analysis-stats/{name}/{metric}"), value, unit.into());
        }
        Ok(())
    }
}

fn parse_metrics(output: &str) -> Vec<(&str, u64, &str)> {
    output
        .lines()
        .filter_map(|it| {
            let entry = it.split(':').collect::<Vec<_>>();
            match entry.as_slice() {
                ["METRIC", name, value, unit] => Some((*name, value.parse().unwrap(), *unit)),
                _ => None,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Metrics {
    host: Host,
    timestamp: SystemTime,
    revision: String,
    perf_revision: String,
    metrics: BTreeMap<String, (u64, Unit)>,
}

#[derive(Debug)]
struct Host {
    os: String,
    cpu: String,
    mem: String,
}

impl Metrics {
    fn new(sh: &Shell) -> anyhow::Result<Metrics> {
        let host = Host::new(sh)?;
        let timestamp = SystemTime::now();
        let revision = cmd!(sh, "git rev-parse HEAD").read()?;
        let perf_revision = "c52ee623e231e7690a93be88d943016968c1036b".into();
        Ok(Metrics { host, timestamp, revision, perf_revision, metrics: BTreeMap::new() })
    }

    fn report(&mut self, name: &str, value: u64, unit: Unit) {
        self.metrics.insert(name.into(), (value, unit));
    }

    fn json(&self) -> String {
        let mut buf = String::new();
        self.to_json(write_json::object(&mut buf));
        buf
    }

    fn to_json(&self, mut obj: write_json::Object<'_>) {
        self.host.to_json(obj.object("host"));
        let timestamp = self.timestamp.duration_since(UNIX_EPOCH).unwrap();
        obj.number("timestamp", timestamp.as_secs() as f64);
        obj.string("revision", &self.revision);
        obj.string("perf_revision", &self.perf_revision);
        let mut metrics = obj.object("metrics");
        for (k, (value, unit)) in &self.metrics {
            metrics.array(k).number(*value as f64).string(unit);
        }
    }
}

impl Host {
    fn new(sh: &Shell) -> anyhow::Result<Host> {
        if cfg!(not(target_os = "linux")) {
            bail!("can only collect metrics on Linux ");
        }

        let os = read_field(sh, "/etc/os-release", "PRETTY_NAME=")?.trim_matches('"').to_string();

        let cpu = read_field(sh, "/proc/cpuinfo", "model name")?
            .trim_start_matches(':')
            .trim()
            .to_string();

        let mem = read_field(sh, "/proc/meminfo", "MemTotal:")?;

        return Ok(Host { os, cpu, mem });

        fn read_field(sh: &Shell, path: &str, field: &str) -> anyhow::Result<String> {
            let text = sh.read_file(path)?;

            text.lines()
                .find_map(|it| it.strip_prefix(field))
                .map(|it| it.trim().to_string())
                .ok_or_else(|| format_err!("can't parse {}", path))
        }
    }
    fn to_json(&self, mut obj: write_json::Object<'_>) {
        obj.string("os", &self.os).string("cpu", &self.cpu).string("mem", &self.mem);
    }
}
