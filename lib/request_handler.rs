use std::fs::File;
use std::io::{Read,Write};
use decoder;

fn subtract_char(x:&String,y:char) -> String{
    let mut z = String::new();
    for i in x.chars() {
    if i != y {
    z = format!("{}{}",z,i.to_string());
            }
        }
    z
    }

fn get_body_request_line(request: &String) -> String {
    let list_of_lines_incomplete = split_string(request,'\n');
    let mut list_of_lines = Vec::new();
    for i in list_of_lines_incomplete.iter() {
        list_of_lines.push(subtract_char(i, '\r'));
    }
    let mut line_index:usize = 0;
    let mut start_reading_line = String::new();
    let mut finished = false;
    let mut counter = 0;
    let mut output = String::new();
    for (index,value) in list_of_lines.iter().enumerate() {
        if value == "" {
            if finished != true {
                line_index = index;
                start_reading_line = value.to_string();
                finished = true;
            }
        }
    }
    finished = false;
    for i in request.chars() {
        if counter == line_index+1 {
            output = format!("{}{}",output,i.to_string());
            finished=true;
        }
        if i == '\n' {
            if finished != true {counter+=1;}
        }
    }
    output
}

fn read_between_chars(input: &String,x:char,y:char) -> String {
    let mut get_chars = false;
    let mut finished = false;
    let mut output = String::new();
    for i in input.chars() {
        if i == y {get_chars=false;}
        if get_chars == true {
            output = format!("{}{}",output,i.to_string());
        }
        if i == x {
            if finished != true {
                get_chars=true;
                finished=true;
                }
            }
    }
    if finished != true {
        output = "NONE".to_string();
    }
    output
}

fn split_string(input:&String,char_a:char) -> Vec<String> {
    let mut parameters_list = Vec::new();
    let mut temp_string = String::new();
    for i in input.chars() {
        if i == char_a {parameters_list.push(temp_string);temp_string = String::from("");}
        else {temp_string = format!("{}{}",temp_string,i.to_string());}
    }
    parameters_list
}

fn especial_split_string(input:&String,char_a:char) -> Vec<String> {
    let mut parameters_list = Vec::new();
    let mut temp_string = String::new();
    let mut stop = false;
    let mut stop2 = false;
    for i in input.chars() {
        if i == '[' {stop=true;}
        if i == ']' {stop=false;}
        if stop != true {
            if i == char_a {parameters_list.push(temp_string);temp_string = String::from("");stop2=true;}
        }
        if stop2 != true {
            if i != ']' {
                if i != '[' {
                    if i != '\r' {temp_string = format!("{}{}",temp_string,i.to_string());}
                }
            }
        }
        stop2 = false;
    }
    parameters_list
}

fn is_function(input:&String) -> bool {
    let mut output = false;
    if read_between_chars(input,'(',')') == "" {
        output = true;
    }
    output
}

fn execute_instruction(instruction:&String,parameters:&Vec<String>) {
    let mut instructions_list = Vec::new();
    instructions_list.push("character-decoder()".to_string());
    instructions_list.push("decode-from-web-input()".to_string());
    if instruction == &instructions_list[0].to_string() {decoder::decode_txt_file(&parameters[0].to_string(),&parameters[1].to_string());}
    if instruction == &instructions_list[1].to_string() {decoder::decode_string_input(&parameters[0].to_string(),&parameters[1].to_string());}
    
}

pub fn handle_request(s1:&String) -> String {
    let mut parameters = Vec::new();
    let mut function_a = String::new();
    let mut file_directory = String::new();
    let mut file_content = String::new();
    let mut response = String::new();
    let mut requested_file: File;
    let line = split_string(s1,'\n')[0].to_string();
    if is_function(&line) == true {
        parameters = especial_split_string(&get_body_request_line(s1),',');
        function_a = read_between_chars(&line,'/',' ');
        execute_instruction(&function_a,&parameters);
        response = format!("HTTP/1.1 200 OK");
    }
    else {
        file_directory = read_between_chars(&line,'/',' ');
        if file_directory == "" {
            requested_file = File::open("RinWorld.html").unwrap();
            requested_file.read_to_string(&mut file_content);
        }else {
            requested_file = File::open(&file_directory).unwrap();
            requested_file.read_to_string(&mut file_content);
        }
        response = format!("HTTP/1.1 200 OK\nContent-Length:{}\n\n{}",file_content.len(),file_content);
    }
    response
}

fn main() {
    handle_request(&"POST /xor() HTTP/1.1\n\rHost: 127.0.0.1\n\rConnection: keep-alive\n\r\n\r[gola,Formal],fojd,".to_string());
}