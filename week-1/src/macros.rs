use std::fmt::Error;

trait Serializable {
    // Searlizes qty_1 and qty_2 into a bunch of bytes
    //if struct looks like this  = Swap { qty_1: u32, qty_2: u32 }
    //then serialize will return a String representation of the struct
    // output looks like [0,0,0,1, 0,0,0,2] for Swap { qty_1: 1, qty_2: 2 }
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialize: Sized {
    // Deserializes a vector of bytes into a struct
    // if input is [0,0,0,1, 0,0,0,2] then it will return Swap { qty_1: 1, qty_2: 2 }
    fn deserialize(v: Vec<u8>) -> Result<Self, Error>;
}

struct Swap {
    qty_1: u32,
    qty_2: u32,
}

impl Serializable for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend_from_slice(&self.qty_1.to_le_bytes());
        v.extend_from_slice(&self.qty_2.to_le_bytes());
        return v;
    }
}

impl Deserialize for Swap {
    fn deserialize(v: Vec<u8>) -> Result<Self, Error> {
        if v.len() != 8 {
            return Err(Error);
        }
        let qty_1 = u32::from_le_bytes(v[0..4].try_into().unwrap());
        let qty_2 = u32::from_le_bytes(v[4..8].try_into().unwrap());
        Ok(Swap { qty_1, qty_2 })
    }
}
struct User {
    name: String,
    age: u32,
}

// Implementing the Display trait for User to use println! macro
// This allows us to print User instances directly using println!
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

// Implementing the Debug trait for User to use println! macro with {:?}
// This allows us to print User instances in a debug format
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ name: {}, age: {} }}", self.name, self.age)
    }
}

macro_rules! vector {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub fn macro_print() {
    let u = User {
        name: String::from("Yuvraj"),
        age: 19,
    };

    // Using the Display trait to print User instance
    println!("{}", u);
    // Using the Debug trait to print User instance
    println!("{:?}", u);
    // Using the vector macro to create a vector
    let v = vector![1, 2, 3, 4, 5];
    println!("Vector: {:?}", v);

    let s = Swap {
        qty_1: 10,
        qty_2: 20,
    };
    // Serializing the Swap instance
    let serialized = s.serialize(); 
    println!("Serialized Swap: {:?}", serialized);
    // Deserializing the Swap instance
    let deserialized = Swap::deserialize(serialized).unwrap();
    println!("Deserialized Swap: qty_1 = {}, qty_2 = {}", deserialized.qty_1, deserialized.qty_2);
}

// #[get("/user")] -> attribute macro
// fn create_user() {
//     sqlx::query!("INSERT INTO USERS VALUE ()"); -> This is a procedural macro
// }