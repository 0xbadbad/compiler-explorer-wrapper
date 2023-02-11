#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub code: i32,
    pub stderr:: Vec<Text>,
    pub asm: Vec<Text>,
}