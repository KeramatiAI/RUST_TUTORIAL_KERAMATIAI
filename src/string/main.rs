fn main() {
    // 1. نوع &str (String Slice)
    // این‌ها معمولاً رشته‌های ثابت هستند که در زمان کامپایل در حافظه برنامه قرار می‌گیرند.
    let fixed_string: &str = "Hello this is a fixed string";
    println!("{}", fixed_string);

    // 2. نوع String (رشته داینامیک)
    // این نوع در Heap ذخیره می‌شود و می‌توانید آن را تغییر دهید (اضافه یا کم کنید).
    let mut dynamic_string = String::from("Rust");

    // اضافه کردن یک کاراکتر یا رشته به انتهای آن
    dynamic_string.push_str(" Is Good.");
    println!("{}", dynamic_string);

    // 3. تبدیل &str به String و برعکس
    let s1 = "JAVA".to_string();
    let s2: &str = &dynamic_string; // گرفتن Slice از String (بسیار رایج)

    println!("s1: {}, s2: {}", s1, s2);

    let company:&'static str = "KERAMATIAI";
    let location:&'static str = "TEHRAN";
    println!("company is : {} location :{}",company,location);

    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());


    let mut z = String::new();
    z.push_str("hello");
    println!("{}",z);

    let name1 = "Hello TutorialsPoint ,
   Hello!".to_string();
    println!("{}",name1);

    let name1 = "Hello TutorialsPoint ,
   Hello!".to_string();         //String object
    let name2 = name1.replace("Hello","Howdy");    //find and replace
    println!("{}",name2);

    let example_string = String::from("example_string");
    print_literal(example_string.as_str());

    let mut company = "DAVOUD".to_string();
    company.push('s');
    println!("{}",company);

    let mut company = "DAVOUD".to_string();
    company.push_str(" KERAMATI");
    println!("{}",company);

    let fullname = " DAVOUD KERAMATI IS A TEACHER";
    println!("length is {}",fullname.len());

    println!();
    let fullname = " MY NAME IS DAVOUD \r\n";
    println!("Before trim ");
    println!("length is {}",fullname.len());
    println!();
    println!("After trim ");
    println!("length is {}",fullname.trim().len());

    println!();

    let msg = "davoud keramati is a teacher".to_string();
    let mut i = 1;

    for token in msg.split_whitespace(){
        println!("token {} {}",i,token);
        i+=1;
    }

    println!();

    let fullname = "Kannan,Sudhakaran,Tutorialspoint";

    for token in fullname.split(","){
        println!("token is {}",token);
    }

    //store in a Vector
    println!("\n");
    let tokens:Vec<&str>= fullname.split(",").collect();
    println!("firstName is {}",tokens[0]);
    println!("lastname is {}",tokens[1]);
    println!("company is {}",tokens[2]);

    println!();

    let n1 = "Iran , Tehran".to_string();

    for n in n1.chars(){
        println!("{}",n);
    }

    println!();

    let n1 = "Davoud".to_string();
    let n2 = " Keramati".to_string();

    let n3 = n1 + &n2; // n2 reference is passed
    println!("{}",n3);

    println!();

    let number = 2020;
    let number_as_string = number.to_string();

    // convert number to string
    println!("{}",number_as_string);
    println!("{}",number_as_string=="2020");

    println!();

    let n1 = "Tutorials".to_string();
    let n2 = "Point".to_string();
    let n3 = format!("{} {}",n1,n2);
    println!("{}",n3);
}


fn print_literal(data:&str ){
    println!("displaying string literal {}",data);
}


