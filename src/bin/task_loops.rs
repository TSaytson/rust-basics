pub(crate) fn clear_screen_os_command() -> &'static str{
  if cfg!(target_os = "linux") {return "clear"} else {return "cls"}
}

fn count(){
  println!("Count");
  for i in 1..=10{
    print!("num = {i}, ");
  }
  println!();
}

fn count_down(){
  println!("Count_Down");
  let mut count:u8 = 10;

  while count > 0{
    print!("num = {count}, ");
    count-=1;
  }
}

fn main(){
  std::process::Command::new(clear_screen_os_command()).status().unwrap();
  count();
  count_down();
}