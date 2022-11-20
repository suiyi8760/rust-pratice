pub fn demo_condition() {
    for i in 1..9 {
        let mut line = "".to_string();
        for j in 1..9 {
            if j <= i {
                line += &format!("{}*{}={};", i, j, i * j);
            } else {
                break;
            }
        }
        println!("{}", line);
    }
}
