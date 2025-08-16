pub async fn build_project_stub() -> Result<String, String> {
    Ok("Build OK: produced dist/MyGame.z64".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
