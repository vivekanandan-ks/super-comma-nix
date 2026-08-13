pub fn resolve(spec: &str, scope: Option<&str>, flake: &str) -> (String, String, String) {
    let (pkg, bin) = spec.split_once(':').unwrap_or((spec, ""));
    let (attr, base) = if pkg.starts_with("latest.")
        || pkg.starts_with("tip.")
        || pkg.starts_with("versions.")
        || pkg
            .split('.')
            .next()
            .unwrap_or("")
            .chars()
            .all(|c| c.is_ascii_digit())
    {
        (
            pkg.to_string(),
            pkg.split('.').last().unwrap_or(pkg).to_string(),
        )
    } else if let Some((p, v)) = pkg.split_once('.') {
        (format!("versions.{}.\"{}\"", p, v), p.to_string())
    } else {
        let rel = scope.unwrap_or("latest");
        (format!("{}.{}", rel, pkg), pkg.to_string())
    };
    (format!("{}#{}", flake, attr), bin.to_string(), base)
}
