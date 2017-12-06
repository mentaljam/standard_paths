/// Read system paths if not provided.
macro_rules! check_paths {
    ($paths:ident) => {
        if $paths.is_empty() {
            let path_var = match env::var("PATH") {
                Ok(var) => var,
                _ => return None
            };
            $paths = env::split_paths(&path_var).collect();
        }
        $paths.retain(|p| !p.to_str().unwrap().is_empty() && p.is_dir());
        let $paths = {
            let mut paths2 = Vec::new();
            for path in $paths {
                if !paths2.contains(&path) {
                    paths2.push(path.canonicalize().unwrap());
                }
            }
            paths2
        };
    };
}
