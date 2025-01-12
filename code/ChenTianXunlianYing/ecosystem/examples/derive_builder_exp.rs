use derive_builder::Builder;


#[derive(Debug, Builder)]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(default = "24")]
    age: u32,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

impl User {
    fn build() -> UserBuilder {
        UserBuilder::default()
    }
}
fn main() -> anyhow::Result<()>{
    let user = User::build()
        .name("Akira")
        .skill("progamming")
        .skill("create Bug")
        .build()
        .unwrap();
    
    Ok(())
}