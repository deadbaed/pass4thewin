pub fn insert(password: &str, multi_line: bool, echo: bool, force: bool) {
    println!(
        "cmd insert: password {:?} multi_line {}, echo {} force {}",
        password, multi_line, echo, force
    );

    /*

      1. check if file exists (unless force flag is passed)
      2. get password (single line: ask twice for confirmation, multiline open notepad or terminal?)
      3. put contents in tmp file
      4. encrypt tmp file
      5. create folders if needed before
      5. move it to path
      6. add commit
      7. if echo flag is on display password

    */
}
