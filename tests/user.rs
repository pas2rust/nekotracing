use kenzu::Builder;
use nekotracing::nekotrancing;

#[derive(Debug, Builder, Clone)]
pub struct User {
    id: u128,
    name: String,
    age: u8,
}

impl User {
    #[nekotrancing]
    fn sync_user(self) -> Result<Self, String> {
        Ok(self
            .name(UserName::new("sync user")?)
            .age(UserAge::new(18)?)
            .id(UserId::new(0)?))
    }
    #[nekotrancing]
    async fn async_user(self) -> Result<Self, String> {
        Ok(self
            .name(UserName::new("async user")?)
            .age(UserAge::new(19)?)
            .id(UserId::new(1)?))
    }
}

#[test]
fn sync_user() -> Result<(), String> {
    User::new().sync_user()?;
    Ok(())
}

#[tokio::test]
async fn async_user() -> Result<(), String> {
    User::new().async_user().await?;
    Ok(())
}

#[tokio::test]
async fn read_tracing_async() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::fs;
    let content = fs::read_to_string("tracing.txt")
        .await
        .map_err(|e| format!("failed to open tracing.txt: {}", e))?;

    assert!(!content.trim().is_empty(), "tracing.txt is empty");

    let sync_count = content.matches("fn sync_user").count();
    let async_count = content.matches("async fn async_user").count();
    assert!(sync_count > 0, "no sync_user trace found in tracing.txt");
    assert!(async_count > 0, "no async_user trace found in tracing.txt");

    let mut parsed_ns: Vec<f64> = Vec::new();
    for line in content.lines() {
        if let Some(pos) = line.find("execution time=") {
            let mut i = pos + "execution time=".len();
            let chars: Vec<char> = line.chars().collect();
            let n = chars.len();

            let mut num_s = String::new();
            while i < n {
                let c = chars[i];
                if c.is_ascii_digit() || c == '.' {
                    num_s.push(c);
                    i += 1;
                } else {
                    break;
                }
            }
            if num_s.is_empty() {
                continue;
            }

            let mut unit = String::new();
            while i < n && !chars[i].is_whitespace() {
                unit.push(chars[i]);
                i += 1;
                if unit.len() >= 2 {
                    break;
                }
            }

            let value: f64 = match num_s.parse() {
                Ok(v) => v,
                Err(_) => continue,
            };

            let ns = match unit.as_str() {
                "ns" => value,
                "µs" | "us" => value * 1_000.0,
                "ms" => value * 1_000_000.0,
                "s" => value * 1_000_000_000.0,
                _ => value * 1_000.0,
            };

            parsed_ns.push(ns);
        }
    }

    assert!(
        !parsed_ns.is_empty(),
        "no 'execution time' entries were parsed from tracing.txt"
    );
    let count = parsed_ns.len();
    let sum: f64 = parsed_ns.iter().sum();
    let min = parsed_ns.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = parsed_ns.iter().cloned().fold(0.0_f64, f64::max);
    let mean = sum / (count as f64);
    eprintln!(
        "tracing.txt: sync_count={} async_count={} parsed_entries={} min_ns={} max_ns={} mean_ns={:.3}",
        sync_count, async_count, count, min as u128, max as u128, mean
    );
    assert!(min > 0.0, "some parsed time is zero or negative");

    Ok(())
}

#[test]
fn read_tracing_sync() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    let content = fs::read_to_string("tracing.txt")
        .map_err(|e| format!("failed to open tracing.txt: {}", e))?;

    assert!(!content.trim().is_empty(), "tracing.txt is empty");

    let sync_count = content.matches("fn sync_user").count();
    let async_count = content.matches("async fn async_user").count();
    assert!(sync_count > 0, "no sync_user trace found in tracing.txt");
    assert!(async_count > 0, "no async_user trace found in tracing.txt");

    let mut parsed_ns: Vec<f64> = Vec::new();
    for line in content.lines() {
        if let Some(pos) = line.find("execution time=") {
            let mut i = pos + "execution time=".len();
            let chars: Vec<char> = line.chars().collect();
            let n = chars.len();
            let mut num_s = String::new();
            while i < n {
                let c = chars[i];
                if c.is_ascii_digit() || c == '.' {
                    num_s.push(c);
                    i += 1;
                } else {
                    break;
                }
            }
            if num_s.is_empty() {
                continue;
            }
            let mut unit = String::new();
            while i < n && !chars[i].is_whitespace() {
                unit.push(chars[i]);
                i += 1;
                if unit.len() >= 2 {
                    break;
                }
            }
            let value: f64 = match num_s.parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            let ns = match unit.as_str() {
                "ns" => value,
                "µs" | "us" => value * 1_000.0,
                "ms" => value * 1_000_000.0,
                "s" => value * 1_000_000_000.0,
                _ => value * 1_000.0,
            };
            parsed_ns.push(ns);
        }
    }

    assert!(
        !parsed_ns.is_empty(),
        "no 'execution time' entries were parsed from tracing.txt"
    );
    let count = parsed_ns.len();
    let sum: f64 = parsed_ns.iter().sum();
    let min = parsed_ns.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = parsed_ns.iter().cloned().fold(0.0_f64, f64::max);
    let mean = sum / (count as f64);
    eprintln!(
        "SYNC tracing.txt: sync_count={} async_count={} parsed_entries={} min_ns={} max_ns={} mean_ns={:.3}",
        sync_count, async_count, count, min as u128, max as u128, mean
    );
    assert!(min > 0.0, "some parsed time is zero or negative");

    Ok(())
}
