fn foo1<'a>(x: &'a u32, y: &'a u32) -> &'a u32 {
    x
}

fn foo2<'a, 'b>(x: &'a u32, y: &'b u32) -> &'a u32 {
    x
}

// Rule1: If there are two parameters, each parameter is assigned to their own lifetimes
// fn rule1(x: &u32, y: &u32) -> &u32 { //Missing lifetime specifier
//     &(x + y)
// }

fn rule1<'a>(x: &'a u32, y: &'a u32) -> &'a u32 {
    x
}

fn rule11<'a>(x: &'a u32, y: &'a u32) -> &'a u32 {
    if x > y {
        x
    } else {
        y
    }
}

//You'll not be able to return a value owned by a function. Instead return owned types such as String
fn rule12<'a>(x: &'a u32, y: &'a u32) -> u32 {
    x + y
}

//Rule 2 : If there is only one parameter, then the output parameter gets assigned the same lifetime of the single parameter
fn rule2(x: &u32) -> &u32 {
    x
}

struct Hello {}

impl Hello {
    //Rule 3 : If there are multiple life time parameters but if there is a self parameter (because the function is a method),
    // the output parameter gets assigned the self lifetime.
    fn rule3<'a, 'b>(&self, x: &'a u32, _y: &'b u32) -> &u32 {
        &5
    }
}

fn main() {
    let result1 = foo1(&1, &2);
    println!("{}", result1);
    let result2 = foo2(&1, &2);
    println!("{}", result2);
}
