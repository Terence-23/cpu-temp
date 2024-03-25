use std::process::Command;



fn main() {

    let mut command = Command::new("sensors");
    command.arg("-A").arg("coretemp-isa-0000");
    let data = String::from_utf8(command.output().expect("No output").stdout).unwrap();
    let mut max_t = -400.0;
    let mut min_t = 300.0;
    let mut iter = data.split("\n");
    _=iter.next();
    for l in iter{
        let temp_s = &l.split(":").collect::<Vec<_>>();
        let l_text = match temp_s.get(1){
            Some(v) => v,
            None => {break;}
        }.split("°").next().unwrap().trim();
        
        let num = l_text.parse::<f64>().unwrap();
        //println!("{}",num);
        if num < min_t{
            min_t = num;
        }
        if num > max_t{
            max_t = num;
        }
    }
    println!("Cpu temp max: {}, min: {}", max_t, min_t);
}
