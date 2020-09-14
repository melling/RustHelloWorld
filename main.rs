struct Word {
  category: String,
  english: String,
  spanish: String,
  active: bool
  
}

fn main() {
  println!("Hello World!");
  for_loop();
  populate_words();
  
 // let fizz = fizz_buzz(15);
 let _name = get_name();
 
 ownership_examples();
 
 string_slices();
 array_slices();
}

fn populate_words() {
  
  let word1 = Word {
    spanish: String::from("manzana"),
    english: String::from("apple"),
    category: String::from("all"),
active: true

  };
  println!("{}, english={}, spanish={}, {}", word1.category, word1.english, word1.spanish, word1.active);
  
  let w2 = create_word("colors".to_string(), "red".to_string(), "rojo".to_string());
    println!("{}, english={}, spanish={}, {}", w2.category, w2.english, w2.spanish, w2.active);

}

fn create_word(cat:String, english:String, spanish:String) -> Word {
  
  Word {
    category: cat,
    english,
    spanish,
    active: true
  }
}
/*
fn create_word(cat:String, english:String, spanish:String) -> Word {
  
  Word {
    category: cat,
    english,
    spanish,
    active: true
  }
}
*/
fn array_slices() {
  let xs = [1,2,3,4,5];
  let ys = &xs[1..3];
   for y in ys.iter() {
    println!("y={}", y)
  }

}
fn string_slices() {
    let s = String::from("abcdefghijklmnop");
    let one = &s[0..4];  // up to but not including 4
    
  println!("{}", one);

  
}
fn ownership_examples() {
  let mut s = String::from("abc");
  ownership02(&s);

 s.push_str("def");
 println!("{}", s);
 
  ownership01(s);

 let mut s2 = String::from("xyz");
  copy_it(&mut s2)
  
}
/*
Taking ownership
*/
fn ownership01(s:String) {
   println!("{}", s);
  
}

/*
borrow
*/
fn ownership02(s:&String) {
   println!("{}", s);
  
}

/*
mutate
*/
fn copy_it(s:&mut String) {
   println!("{}", s);
   s.push_str("123")
  
}

fn get_name() -> String {
    let s = String::from("abc");

    s
  
}

/*
fn fizz_buzz(i:i32) -> String {
  
  if i % 5 == 0  && i % 3 == 0 {
    return "FizzBuzz";
  } else if i % 3 == 0 {
    return "Fizz";
  } else if i % 5 == 0 {
    return "Buzz";
    
  } else {
    return "Boom";
  }
}
*/

fn for_loop() {
  let xs = [1,2,3,4,5];
  
  for x in xs.iter() {
    println!("x={}", x)
  }
}
