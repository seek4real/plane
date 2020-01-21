// message


pub enum Message
{
    Quit,
    Move {x:i32, y:i32},
    ChangeColor(i32, i32, i32),
    Write(String),
}

// pub struct QuitMessage;
// pub struct MoveMessage{
//     x: i32,
//     y: i32,
// }
// pub struct WriteMessage(String);
// pub struct ChangeColorMessage(i32, i32, i32);

impl Message{
    pub fn call(&self)->&Message{
        println!("quit!");
        return self;
    }
}