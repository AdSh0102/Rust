#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message
{
    fn call(&self)
    {
        println!("{:?}", self);
    }
}

fn main()
{
    let m: Message = Message::Write(String::from("Hello, World"));
    m.call();
}