struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Refactoring with struct
struct Rectangle{
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Rect{
    width:u32,
    height:u32,
}

impl Rect{
    fn my_area(&self) ->u32{
        self.width * self.height
    }
}

fn main() {
    println!("struct!");
    let width1 = 30;
    let height1 = 50;

    // method syntax
    let my_rect = Rect{
        width:40,
        height:20,
    };

 println!(
        "The area of the REC is {} square pixels.",
        my_rect.my_area()
    );
    // tuple
    let rect1 = (10, 50);
    println!("area of rect using tupple is {} square pixels",area_dimensions(rect1));
    println!("the area of the rectangle is {} square pixels.", area(width1, height1));

    // Refactoring with struct
    let rectangle1 = &Rectangle{
        width: 10,
        height: 10,
    };

    println!("the area of rectangle using struct is {} square pixel", area_struct(rectangle1));

    let mut user1 = User{
        active:true,
        username: String::from("someusername123"),
        email: String::from("someexample@gmail.com"),
        sign_in_count: 1,
    };

    let name = String::from("vince");
    let mail = String::from("bisi@mail.com");
    build_user(name, mail);


}

fn build_user(email: String, username: String) ->User{
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count:1,
    }
}

fn area(width:u32, height:u32) ->u32{
    width * height
}

// using tuple
fn area_dimensions(dimensions: (u32, u32))->u32{
    dimensions.0 * dimensions.1
}

// using struct refractoring
fn area_struct(rectangle: &Rectangle) ->u32{
    rectangle.width * rectangle.height
}

