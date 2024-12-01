use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::stdout;
use std::io;
use regex::Regex;

fn file_to_vec(filename: String) -> io::Result<Vec<Vec<i32>>> {
    let mut vec1 : Vec<i32> = vec![];
    let mut vec2 : Vec<i32> = vec![];
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let re = Regex::new(r"   ").unwrap();
    let mut count=0;
    for line in file_reader.lines(){
        count = count+1;
        let aux=0;
        let num=0;


        let hay = line.unwrap();
        let matchy = re.find(&hay);
        let mut number1 = &hay[..matchy.unwrap().start()];
        let mut number2 = &hay[matchy.unwrap().end()..];
        println!("{number1} {number2}");
        vec1.push(number1.parse::<i32>().unwrap());
        vec2.push(number2.parse::<i32>().unwrap());
    }
    let vecR : Vec<Vec<i32>> = vec![vec1, vec2];
    return Ok(vecR);
}


fn extractInt(text: String, vec: &Vec<i32>){
    let bytes = text.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        println!("{:?}", item);
    }
}

fn getCount(vec: Vec<i32>, num: i32) -> i32{
    let index = vec.binary_search(&num);
    return match index {
        Ok(value) =>  (helpCount(vec, &num, value))* i32::try_from(num).unwrap(),
        Err(error) => 0,
    }
}

fn helpCount(vec: Vec<i32>, num: &i32, index: usize) -> i32{
    let mut lower:usize = index;
    let mut higher:usize = index;
    let size = vec.len();
    while(lower>0 && vec[lower] == *num){
        lower = lower-1;
    }
    let mut aux = 0;
    if(lower==0 && vec[0]==*num){
        aux = 1;
    } 
    while(higher<size && vec[higher] == *num){
        higher = higher+1;
    }
    println!("num: {num} lower:{lower} higher: {higher}");
    return (higher-lower-1 + aux).try_into().unwrap();
    

}


fn main(){
    let stdout = stdout();
    let test = file_to_vec("2.in".to_string());

    let mut arrs = test.unwrap();
    arrs[0].sort();
    arrs[1].sort();

    //let vecResult: Vec<i32> = arrs[0].iter().zip(arrs[1].iter()).map(|(&x, &y)| ((y-x).abs())).collect();
    //println!("{:?}", vecResult);
    //let res: i32 = vecResult.into_iter().sum();
    //println!("result: {res}");

    //arrs[0].dedup();
    let vecResult2: Vec<i32> = arrs[0].clone().into_iter().map(| x | { getCount(arrs[1].clone(), x) }).collect();
    println!("{:?}", vecResult2);
    let res2: i32 = vecResult2.into_iter().sum();
    println!("{res2}")
}