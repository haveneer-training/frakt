pub fn setup_env() {
    // let env_file_path: std::path::PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    // println!("Valeur de env_file_path : {}", env_file_path.display());
    dotenv::from_path("../.env").ok();  //chemin a mettre doit etre depuis la racine du crate donc considérer que l'on se trouve dans frakt/shared pour définir le chemin.
}


#[test]
fn test_setup_env() {
    // Appelez la fonction setup_env pour charger les variables d'environnement
    setup_env();

    // Vérifiez les valeurs des variables d'environnement
    assert_eq!(std::env::var("HOST_ADDRESS"), Ok(String::from("localhost")));
    assert_eq!(std::env::var("PORT"), Ok(String::from("8787")));
    assert_eq!(std::env::var("NTILES"), Ok(String::from("4")));
}